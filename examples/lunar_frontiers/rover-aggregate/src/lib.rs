use anyhow::Result;
use serde::{Deserialize, Serialize};
use wasmcloud_interface_logging::error;

mod commands;
mod events;
mod state;

use state::RoverAggregateState;

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
        _state: Option<RoverAggregateState>,
    ) -> Result<EventList> {
        commands::initialize_rover(input)
    }

    fn handle_change_destination(
        &self,
        input: ChangeDestination,
        state: Option<RoverAggregateState>,
    ) -> Result<EventList> {
        commands::change_destination(input, state)
    }

    // -- EVENTS --

    fn apply_rover_initialized(
        &self,
        input: RoverInitialized,
        _state: Option<RoverAggregateState>,
    ) -> Result<StateAck> {
        events::apply_rover_initialized(input)
    }

    fn apply_rover_destination_changed(
        &self,
        input: RoverDestinationChanged,
        state: Option<RoverAggregateState>,
    ) -> Result<StateAck> {
        events::apply_destination_changed(input, state)
    }

    fn apply_rover_destination_reached(
        &self,
        input: RoverDestinationReached,
        state: Option<RoverAggregateState>,
    ) -> Result<StateAck> {
        events::apply_destination_reached(input, state)
    }

    fn apply_rover_stopped(
        &self,
        input: RoverStopped,
        state: Option<RoverAggregateState>,
    ) -> Result<StateAck> {
        events::apply_rover_stopped(input, state)
    }

    fn apply_rover_started(
        &self,
        input: RoverStarted,
        state: Option<RoverAggregateState>,
    ) -> Result<StateAck> {
        events::apply_rover_started(input, state)
    }

    fn apply_rover_position_changed(
        &self,
        input: RoverPositionChanged,
        state: Option<RoverAggregateState>,
    ) -> Result<StateAck> {
        events::apply_position_changed(input, state)
    }
}
