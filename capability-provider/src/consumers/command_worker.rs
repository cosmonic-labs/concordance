use async_nats::jetstream::Context;
use tracing::debug;

use crate::{
    config::InterestDeclaration,
    consumers::WorkError,
    events::publish_es_event,
    eventsourcing::{AggregateService, AggregateServiceSender, StatefulCommand},
    natsclient::AckableMessage,
    state::EntityState,
};

use super::{RawCommand, WorkResult, Worker};

pub struct CommandWorker {
    pub nc: async_nats::Client,
    pub context: Context,
    pub interest: InterestDeclaration,
    pub state: EntityState,
}

impl CommandWorker {
    pub fn new(
        nc: async_nats::Client,
        context: Context,
        interest: InterestDeclaration,
        state: EntityState,
    ) -> Self {
        CommandWorker {
            nc,
            context,
            interest,
            state,
        }
    }
}

#[async_trait::async_trait]
impl Worker for CommandWorker {
    type Message = RawCommand;

    /// Commands always go to aggregates, and their topic filters are always explicit to one topic, so we
    /// don't need as much ceremony around interest-based dispatch for commands as we do for events
    async fn do_work(&self, mut message: AckableMessage<Self::Message>) -> WorkResult<()> {
        debug!(command = ?message.as_ref(), "Handling received command");

        let state = self
            .state
            .fetch_state(
                &self.interest.role,
                &self.interest.entity_name,
                &message.command_type,
            )
            .await
            .map_err(|e| WorkError::NatsError(format!("Failed to load state: {e}").into()))?;

        // NOTE: you can't invoke `for_actor` without an active provider_main. This will even implicitly attempt
        // to create one, so be wary of using this worker for tests.
        let target = AggregateServiceSender::for_actor(&self.interest.link_definition);
        let cmd = StatefulCommand {
            aggregate: self.interest.entity_name.to_string(),
            command_type: message.command_type.to_string(),
            key: message.key.to_string(),
            state,
            payload: serde_json::to_vec(&message.data).map_err(|e| {
                WorkError::Other(format!(
                    "Raw inbound command payload could not be converted to vec: {e}"
                ))
            })?,
        };
        let ctx = wasmbus_rpc::provider::prelude::Context::default();
        let outbound_events = target.handle_command(&ctx, &cmd).await.map_err(|e| {
            WorkError::Other(format!(
                "Aggregate {} failed to handle command {}, {}, {}, ({} bytes): {:?}",
                self.interest.actor_id,
                cmd.aggregate,
                cmd.command_type,
                cmd.key,
                cmd.payload.len(),
                e
            ))
        })?;

        // Reminder that aggregates don't modify their own state when processing commands. That can only
        // happen when handling events.

        for evt in outbound_events {
            publish_es_event(&self.nc, evt)
                .await
                .map_err(|e| WorkError::NatsError(e.into()))?;
        }

        // Now that the outbound has been processed, ack the inbound (which deletes the command from the work queue CC_COMMANDS stream)
        message.ack().await.map_err(|e| WorkError::NatsError(e))?;

        Ok(())
    }
}
