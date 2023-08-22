use anyhow::Result;
use serde::{Deserialize, Serialize};
use wasmcloud_interface_logging::{debug, error};

//use lunarfrontiers_model::*;
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct RoverAggregateState {
    pub placeholder: u16,
}

concordance_gen::generate!({
    path: "../eventcatalog",
    role: "aggregate",
    entity: "rover"
});

const STREAM: &str = "rover";

impl RoverAggregate for RoverAggregateImpl {
    fn handle_initialize_rover(
        &self,
        input: InitializeRover,
        state: Option<RoverAggregateState>,
    ) -> Result<EventList> {
        todo!()
    }

    fn apply_rover_initialized(
        &self,
        input: RoverInitialized,
        state: Option<RoverAggregateState>,
    ) -> Result<StateAck> {
        todo!()
    }

    fn apply_rover_destination_changed(
        &self,
        input: RoverDestinationChanged,
        state: Option<RoverAggregateState>,
    ) -> Result<StateAck> {
        todo!()
    }

    fn apply_rover_destination_reached(
        &self,
        input: RoverDestinationReached,
        state: Option<RoverAggregateState>,
    ) -> Result<StateAck> {
        todo!()
    }

    fn apply_rover_stopped(
        &self,
        input: RoverStopped,
        state: Option<RoverAggregateState>,
    ) -> Result<StateAck> {
        todo!()
    }

    fn apply_rover_started(
        &self,
        input: RoverStarted,
        state: Option<RoverAggregateState>,
    ) -> Result<StateAck> {
        todo!()
    }

    fn apply_rover_position_changed(
        &self,
        input: RoverPositionChanged,
        state: Option<RoverAggregateState>,
    ) -> Result<StateAck> {
        todo!()
    }
}
