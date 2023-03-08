use async_nats::jetstream::stream::Stream as NatsStream;
use futures::{Stream, StreamExt, TryStreamExt};
use std::{collections::HashMap, sync::Arc};
use tokio::{sync::RwLock, time::timeout};
use tracing::{error, trace, Instrument};

use crate::{
    config::{ActorRole, InterestConstraint, InterestDeclaration},
    natsclient::{AckableMessage, SEND_TIMEOUT_DURATION},
};

use super::{CreateConsumer, WorkError, WorkHandles, WorkResult, Worker};

#[derive(Clone)]
pub struct ConsumerManager {
    handles: WorkHandles,
    evt_stream: NatsStream,
    cmd_stream: NatsStream,
}

impl ConsumerManager {
    fn new(evt_stream: NatsStream, cmd_stream: NatsStream) -> ConsumerManager {
        ConsumerManager {
            handles: Arc::new(RwLock::new(HashMap::default())),
            evt_stream,
            cmd_stream,
        }
    }

    pub async fn consumers(&self) -> Vec<InterestDeclaration> {
        let keys = {
            let lock = self.handles.read().await;
            lock.keys().cloned().collect()
        };
        keys
    }

    pub async fn add_consumer<W, C>(
        &self,
        interest: InterestDeclaration,
        worker: W,
    ) -> Result<(), async_nats::Error>
    where
        W: Worker + Send + Sync + 'static,
        C: Stream<Item = Result<AckableMessage<W::Message>, async_nats::Error>>
            + CreateConsumer<Output = C>
            + Send
            + Unpin
            + 'static,
    {
        let i = interest.clone();
        if !self.has_consumer(&interest).await {
            let consumer = if interest.interest_constraint == InterestConstraint::Commands {
                C::create(self.cmd_stream.clone(), interest.clone()).await?
            } else if interest.interest_constraint == InterestConstraint::Events {
                C::create(self.evt_stream.clone(), interest.clone()).await?
            } else {
                return Err("Tried to create a consumer with an interest constraint of None. This is likely a logic failure".into());
            };

            let handle = tokio::spawn(
                work_fn(consumer, worker, interest)
                    .instrument(tracing::info_span!("consumer_worker", %i)),
            );
            let mut handles = self.handles.write().await;
            handles.insert(i.clone(), handle);
        }
        Ok(())
    }

    /// Checks if this manager has a consumer for the given interest declaration. Returns `false` if it doesn't
    /// exist or has stopped
    pub async fn has_consumer(&self, interest: &InterestDeclaration) -> bool {
        self.handles
            .read()
            .await
            .get(interest)
            .map(|handle| !handle.is_finished())
            .unwrap_or(false)
    }
}

async fn work_fn<C, W>(mut consumer: C, worker: W, interest: InterestDeclaration) -> WorkResult<()>
where
    W: Worker + Send,
    C: Stream<Item = Result<AckableMessage<W::Message>, async_nats::Error>> + Unpin,
{
    loop {
        println!("*** MADE IT HERE ***");
        // Get next value from stream, returning error if the consumer stopped
        let res = consumer.next().await.ok_or(WorkError::ConsumerStopped)?;
        let res = match res {
            Ok(msg) => {
                trace!(message = ?msg, "Got message from consumer");
                worker.do_work(msg).await
            }
            Err(e) => {
                error!(error = %e, "Got error from stream when reading from consumer. Will try again");
                continue;
            }
        };
        match res {
            // Return fatal errors if they occur
            Err(e) if matches!(e, WorkError::Fatal(_)) => return Err(e),
            // For the rest of the errors, right now we just log. Could do nicer retry behavior as this evolves
            Err(e) => error!(error = ?e, "Got error from worker"),
            _ => (),
        }
    }
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::{
        config::InterestDeclaration,
        consumers::{
            command_worker::CommandWorker, event_worker::EventWorker, manager::ConsumerManager,
            CommandConsumer, EventConsumer, RawCommand,
        },
        natsclient::{
            test::{clear_streams, create_js_context, publish_command},
            NatsClient,
        },
        state::EntityState,
    };

    #[tokio::test]
    async fn aggregates_get_two_consumers() {
        let nc = async_nats::connect("127.0.0.1").await.unwrap();
        let js = create_js_context().await;
        clear_streams(js.clone()).await;

        let client = NatsClient::new(nc.clone(), js.clone());
        let (e, c) = client.ensure_streams().await.unwrap();
        let cm = ConsumerManager::new(e, c);
        let interest = InterestDeclaration::aggregate_for_commands("MXBOB", "bankaccount");
        let state = EntityState::new_from_context(&js).await.unwrap();

        cm.add_consumer::<CommandWorker, CommandConsumer>(
            interest.clone(),
            CommandWorker {
                context: js.clone(),
                interest: interest.clone(),
                state: state.clone(),
            },
        )
        .await
        .unwrap();

        let interest2 = InterestDeclaration::aggregate_for_events("MXBOB", "bankaccount");
        cm.add_consumer::<EventWorker, EventConsumer>(
            interest2.clone(),
            EventWorker {
                context: js.clone(),
                interest: interest.clone(),
                state,
            },
        )
        .await
        .unwrap();

        assert!(cm.has_consumer(&interest).await);
        assert_eq!(2, cm.consumers().await.len());
    }

    #[tokio::test]
    async fn command_consumer_worker_function_basic() {
        let nc = async_nats::connect("127.0.0.1").await.unwrap();
        let js = create_js_context().await;
        clear_streams(js.clone()).await;

        let client = NatsClient::new(nc.clone(), js.clone());
        let (e, c) = client.ensure_streams().await.unwrap();
        let cm = ConsumerManager::new(e, c);
        let interest = InterestDeclaration::aggregate_for_commands("MXBOB", "bankaccount");
        let state = EntityState::new_from_context(&js).await.unwrap();

        cm.add_consumer::<CommandWorker, CommandConsumer>(
            interest.clone(),
            CommandWorker {
                context: js.clone(),
                interest: interest.clone(),
                state,
            },
        )
        .await
        .unwrap();

        assert!(cm.has_consumer(&interest).await);
        assert_eq!(1, cm.consumers().await.len());

        let cmds = vec![
            RawCommand {
                command_type: "test_one".to_string(),
                key: "mgr1".to_string(),
                data: json!({
                    "hello": "world"
                }),
            },
            RawCommand {
                command_type: "test_two".to_string(),
                key: "mgr2".to_string(),
                data: json!({
                    "hello": "world2"
                }),
            },
            RawCommand {
                command_type: "test_three".to_string(),
                key: "mgr3".to_string(),
                data: json!({
                    "hello": "world3"
                }),
            },
        ];

        let c = nc.clone();
        for cmd in cmds {
            publish_command(&c, "bankaccount", &cmd).await.unwrap();
        }

        clear_streams(js.clone()).await;

        assert!(true);
    }
}
