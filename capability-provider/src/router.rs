//! # Router
//! This module provides functionality for managing the metadata describing the
//! routing of events and commands to their intended targets

use std::{
    collections::HashSet,
    sync::{Arc, RwLock},
};

use crate::{
    config::{self, ActorInterest, InterestDeclaration},
    Result,
};
use tracing::trace;
use wasmbus_rpc::core::LinkDefinition;

#[derive(Clone)]
pub struct Router {
    interested_parties: Arc<RwLock<HashSet<InterestDeclaration>>>,
}

impl Router {
    pub fn new() -> Self {
        Router {
            interested_parties: Arc::new(RwLock::new(HashSet::new())),
        }
    }

    pub fn register_interest(&self, interest: InterestDeclaration) {
        let mut lock = self.interested_parties.write().unwrap();
        lock.insert(interest);
    }

    pub fn unregister_interest(&self, actor_id: &str) -> Result<()> {
        todo!()
    }
}
