use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct RoverAggregateState {
    pub placeholder: u16,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct ColonyAggregateState {
    pub placeholder: u16,
}
