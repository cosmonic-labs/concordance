use anyhow::Result;
use serde::{Deserialize, Serialize};
use wasmcloud_interface_logging::{debug, error};

use lunarfrontiers_model::*;

concordance_gen::generate!({
    path: "../model/lunar_frontiers.ttl",
    role: "aggregate",
    entity: "rover"
});

const STREAM: &str = "rover";

impl RoverAggregate for RoverAggregateImpl {
    // -- Command Handling

    fn handle_provision_rover(
        &self,
        input: ProvisionRover,
        state: Option<RoverAggregateState>,
    ) -> Result<EventList> {
        todo!()
    }

    fn handle_set_destination(
        &self,
        input: SetDestination,
        state: Option<RoverAggregateState>,
    ) -> Result<EventList> {
        todo!()
    }

    fn handle_stop(&self, input: Stop, state: Option<RoverAggregateState>) -> Result<EventList> {
        todo!()
    }

    fn handle_start(&self, input: Start, state: Option<RoverAggregateState>) -> Result<EventList> {
        todo!()
    }

    fn handle_build_structure(
        &self,
        input: BuildStructure,
        state: Option<RoverAggregateState>,
    ) -> Result<EventList> {
        todo!()
    }

    fn handle_cancel_construction(
        &self,
        input: CancelConstruction,
        state: Option<RoverAggregateState>,
    ) -> Result<EventList> {
        todo!()
    }

    // -- Event Application --

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

    fn apply_construction_began(
        &self,
        input: ConstructionBegan,
        state: Option<RoverAggregateState>,
    ) -> Result<StateAck> {
        todo!()
    }

    fn apply_construction_cancelled(
        &self,
        input: ConstructionCancelled,
        state: Option<RoverAggregateState>,
    ) -> Result<StateAck> {
        todo!()
    }

    fn apply_construction_progressed(
        &self,
        input: ConstructionProgressed,
        state: Option<RoverAggregateState>,
    ) -> Result<StateAck> {
        todo!()
    }

    fn apply_resource_quantity_changed(
        &self,
        input: ResourceQuantityChanged,
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
