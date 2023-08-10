use serde::{Deserialize, Serialize};

mod commands;
mod events;
mod state;

pub use commands::colony::*;
pub use commands::mothership::*;
pub use commands::rover::*;
pub use events::*;
pub use state::*;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct GridCoordinate {
    pub x: u32,
    pub y: u32,
}

/// The type of structure for a given building
#[derive(Serialize, Deserialize, Default, Debug)]
pub enum StructureType {
    #[default]
    ColonyBase,
    PowerGenerator,
    WaterGenerator,
    FoodGenerator,
    ColonistHousing,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub enum ResourceType {
    Power,
    Water,
    Food,
    #[default]
    Colonist,
}
