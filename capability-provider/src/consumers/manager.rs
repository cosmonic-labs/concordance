use async_nats::jetstream::stream::Stream as NatsStream;
use futures::{Stream, StreamExt};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
use tracing::{error, trace, Instrument};

use crate::{
    config::{InterestConstraint, InterestDeclaration},
    natsclient::AckableMessage,
};

use super::{CreateConsumer, WorkError, WorkHandles, WorkResult, Worker};

#[derive(Clone)]
pub struct ConsumerManager {
    handles: WorkHandles,
    evt_stream: NatsStream,
    cmd_stream: NatsStream,
}

impl ConsumerManager {
    pub fn new(evt_stream: NatsStream, cmd_stream: NatsStream) -> ConsumerManager {
        ConsumerManager {
            handles: Arc::new(RwLock::new(HashMap::default())),
            evt_stream,
            cmd_stream,
        }
    }

    #[allow(unused)]
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
            } else {
                C::create(self.evt_stream.clone(), interest.clone()).await?
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

async fn work_fn<C, W>(mut consumer: C, worker: W, _interest: InterestDeclaration) -> WorkResult<()>
where
    W: Worker + Send,
    C: Stream<Item = Result<AckableMessage<W::Message>, async_nats::Error>> + Unpin,
{
    loop {
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
    use std::sync::Arc;

    use serde_json::json;
    use tokio::sync::RwLock;
    use wasmbus_rpc::core::LinkDefinition;

    use crate::{
        config::InterestDeclaration,
        consumers::{
            manager::ConsumerManager, CommandConsumer, EventConsumer, RawCommand, WorkResult,
            Worker,
        },
        natsclient::{
            test::{clear_streams, create_js_context, publish_command},
            AckableMessage, NatsClient,
        },
        state::EntityState,
        workers::{AggregateCommandWorker, AggregateEventWorker},
    };

    #[tokio::test]
    async fn aggregates_get_two_consumers() {
        let nc = async_nats::connect("127.0.0.1").await.unwrap();
        let js = create_js_context().await;
        clear_streams(js.clone()).await;

        let client = NatsClient::new(js.clone());
        let (e, c) = client.ensure_streams().await.unwrap();
        let cm = ConsumerManager::new(e, c);
        let interest = InterestDeclaration::aggregate_for_commands(
            "MXBOB",
            "bankaccount",
            "account_number",
            LinkDefinition::default(),
        );
        let state = EntityState::new_from_context(&js).await.unwrap();

        cm.add_consumer::<AggregateCommandWorker, CommandConsumer>(
            interest.clone(),
            AggregateCommandWorker {
                nc: nc.clone(),
                context: js.clone(),
                interest: interest.clone(),
                state: state.clone(),
            },
        )
        .await
        .unwrap();

        let interest2 = InterestDeclaration::aggregate_for_events(
            "MXBOB",
            "bankaccount",
            "account_number",
            LinkDefinition::default(),
        );
        cm.add_consumer::<AggregateEventWorker, EventConsumer>(
            interest2.clone(),
            AggregateEventWorker {
                nc,
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

        let client = NatsClient::new(js.clone());
        let (e, c) = client.ensure_streams().await.unwrap();
        let cm = ConsumerManager::new(e, c);
        let interest = InterestDeclaration::aggregate_for_commands(
            "MXBOB",
            "bankaccount",
            "account_number",
            LinkDefinition::default(),
        );
        let _state = EntityState::new_from_context(&js).await.unwrap();

        let msgs = Arc::new(RwLock::new(Vec::new()));
        cm.add_consumer::<MockCommandWorker, CommandConsumer>(
            interest.clone(),
            MockCommandWorker::new(msgs.clone()),
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

        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        assert_eq!(3, msgs.read().await.len());

        clear_streams(js.clone()).await;

        assert!(true);
    }

    struct MockCommandWorker {
        pub messages: Arc<RwLock<Vec<AckableMessage<RawCommand>>>>,
    }

    impl MockCommandWorker {
        pub fn new(messages: Arc<RwLock<Vec<AckableMessage<RawCommand>>>>) -> Self {
            MockCommandWorker { messages }
        }
    }

    #[async_trait::async_trait]
    impl Worker for MockCommandWorker {
        type Message = RawCommand;

        async fn do_work(&self, message: AckableMessage<Self::Message>) -> WorkResult<()> {
            let mut lock = self.messages.write().await;
            lock.push(message);
            Ok(())
        }
    }
}
