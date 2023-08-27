use crate::*;

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct RoverAggregateState {
    pub rover_id: String,
    pub position: (u16, u16),
    pub moon_id: String,
    pub mothership_id: String,
    pub pilot_key: String,
    pub current_tick: u64,

    pub destination: Option<(u16, u16)>,
    pub moving: bool,
}
