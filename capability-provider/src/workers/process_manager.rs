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

pub struct ProcessManagerWorker {
    pub nc: async_nats::Client,
    pub context: Context,
    pub interest: InterestDeclaration,
    pub state: EntityState,
}
