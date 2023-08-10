use serde::{Deserialize, Serialize};

use crate::{GridCoordinate, ResourceType, StructureType};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct RoverPositionChanged {
    pub tick: u16,
    pub rover_id: String,
    pub new_position: GridCoordinate,
}

// NOTE: Eventually we won't need this and it'll come for free with Concordance-gen
impl RoverPositionChanged {
    pub const TYPE: &str = "rover_position_changed";
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct RoverStopped {
    pub tick: u16,
    pub rover_id: String,
    pub location: GridCoordinate,
}

impl RoverStopped {
    pub const TYPE: &str = "rover_stopped";
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct RoverStarted {
    pub tick: u16,
    pub rover_id: String,
    pub location: GridCoordinate,
}

impl RoverStarted {
    pub const TYPE: &str = "rover_started";
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct RoverInitialized {
    pub tick: u16,
    pub rover_id: String,
    pub mothership_id: String,
    pub colony_id: String,
    pub autopilot_id: Option<String>,
    pub location: GridCoordinate,
}

impl RoverInitialized {
    pub const TYPE: &str = "rover_initialized";
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct RoverDestinationChanged {
    pub tick: u16,
    pub rover_id: String,
    pub new_destination: GridCoordinate,
}

impl RoverDestinationChanged {
    pub const TYPE: &str = "rover_destination_changed";
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ConstructionBegan {
    pub tick: u16,
    pub rover_id: String,
    pub structure_type: StructureType,
    pub structure_id: String,
    pub required_ticks: u16,
}

impl ConstructionBegan {
    pub const TYPE: &str = "construction_began";
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ConstructionProgressed {
    pub tick: u16,
    pub rover_id: String,
    pub ticks_remaining: u16,
}

impl ConstructionProgressed {
    pub const TYPE: &str = "construction_progressed";
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ConstructionCancelled {
    pub tick: u16,
    pub rover_id: String,
    pub reason: Option<String>,
}

impl ConstructionCancelled {
    pub const TYPE: &str = "construction_cancelled";
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ResourceQuantityChanged {
    pub tick: u16,
    pub rover_id: String,
    pub resource_type: ResourceType,
    pub new_quantity: u16,
    pub reason: QuantityChangeReason,
}

impl ResourceQuantityChanged {
    pub const TYPE: &str = "resource_quantity_changed";
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub enum QuantityChangeReason {
    Harvested,
    Consumed,
    DirectAction,
    #[default]
    Other,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ColonyClaimed {
    pub colony_id: String,
    pub name: String,
    pub grid_height: u32,
    pub grid_width: u32,
}
