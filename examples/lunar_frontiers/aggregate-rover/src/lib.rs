use eventsourcing::*;
use serde::{Deserialize, Serialize};
use wasmbus_rpc::actor::prelude::*;

use lunarfrontiers_model::*;

use wasmcloud_interface_logging::debug;

#[allow(dead_code)]
mod eventsourcing;

#[allow(dead_code)]
mod system_traits;

#[allow(dead_code)]
mod genimpl;

use genimpl::RoverAggregateImpl;
use system_traits::*;

const STREAM: &str = "rover";

impl RoverAggregate for RoverAggregateImpl {
    // -- Command Handling

    fn handle_provision_rover(
        &self,
        input: ProvisionRover,
        state: Option<RoverAggregateState>,
    ) -> RpcResult<EventList> {
        todo!()
    }

    fn handle_set_destination(
        &self,
        input: SetDestination,
        state: Option<RoverAggregateState>,
    ) -> RpcResult<EventList> {
        todo!()
    }

    fn handle_stop(&self, input: Stop, state: Option<RoverAggregateState>) -> RpcResult<EventList> {
        todo!()
    }

    fn handle_start(
        &self,
        input: Start,
        state: Option<RoverAggregateState>,
    ) -> RpcResult<EventList> {
        todo!()
    }

    fn handle_build_structure(
        &self,
        input: BuildStructure,
        state: Option<RoverAggregateState>,
    ) -> RpcResult<EventList> {
        todo!()
    }

    fn handle_cancel_construction(
        &self,
        input: CancelConstruction,
        state: Option<RoverAggregateState>,
    ) -> RpcResult<EventList> {
        todo!()
    }

    // -- Event Application --

    fn apply_rover_initialized(
        &self,
        input: RoverInitialized,
        state: Option<RoverAggregateState>,
    ) -> RpcResult<StateAck> {
        todo!()
    }

    fn apply_rover_destination_changed(
        &self,
        input: RoverDestinationChanged,
        state: Option<RoverAggregateState>,
    ) -> RpcResult<StateAck> {
        todo!()
    }

    fn apply_rover_stopped(
        &self,
        input: RoverStopped,
        state: Option<RoverAggregateState>,
    ) -> RpcResult<StateAck> {
        todo!()
    }

    fn apply_rover_started(
        &self,
        input: RoverStarted,
        state: Option<RoverAggregateState>,
    ) -> RpcResult<StateAck> {
        todo!()
    }

    fn apply_construction_began(
        &self,
        input: ConstructionBegan,
        state: Option<RoverAggregateState>,
    ) -> RpcResult<StateAck> {
        todo!()
    }

    fn apply_construction_cancelled(
        &self,
        input: ConstructionCancelled,
        state: Option<RoverAggregateState>,
    ) -> RpcResult<StateAck> {
        todo!()
    }

    fn apply_construction_progressed(
        &self,
        input: ConstructionProgressed,
        state: Option<RoverAggregateState>,
    ) -> RpcResult<StateAck> {
        todo!()
    }

    fn apply_resource_quantity_changed(
        &self,
        input: ResourceQuantityChanged,
        state: Option<RoverAggregateState>,
    ) -> RpcResult<StateAck> {
        todo!()
    }

    fn apply_rover_position_changed(
        &self,
        input: RoverPositionChanged,
        state: Option<RoverAggregateState>,
    ) -> RpcResult<StateAck> {
        todo!()
    }
}

// NOTE: we ultimately won't need this either, it'll come with Concordance gen
impl StateAck {
    fn ok(state: Option<RoverAggregateState>) -> StateAck {
        StateAck {
            succeeded: true,
            error: None,
            state: state
                .clone()
                .map(|s| serde_json::to_vec(&s).unwrap_or_default()),
        }
    }

    fn error(msg: &str, state: Option<RoverAggregateState>) -> StateAck {
        StateAck {
            succeeded: false,
            error: Some(msg.to_string()),
            state: state
                .clone()
                .map(|s| serde_json::to_vec(&s).unwrap_or_default()),
        }
    }
}

impl Event {
    fn new(event_type: &str, payload: impl Serialize) -> Event {
        Event {
            event_type: event_type.to_string(),
            stream: STREAM.to_string(),
            payload: serde_json::to_vec(&payload).unwrap_or_default(),
        }
    }
}
