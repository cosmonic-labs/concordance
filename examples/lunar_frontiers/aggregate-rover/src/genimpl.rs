// TODO: unhardcode this
use crate::eventsourcing::*;

use crate::*;

use wasmcloud_interface_logging::error;

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, AggregateService)]
pub(crate) struct RoverAggregateImpl {}

#[async_trait]
impl AggregateService for RoverAggregateImpl {
    async fn handle_command(&self, _ctx: &Context, arg: &StatefulCommand) -> RpcResult<EventList> {
        let state: Option<RoverAggregateState> = arg
            .state
            .clone()
            .map(|bytes| deserialize_json(&bytes).unwrap_or_default());

        match arg.command_type.as_str() {
            ProvisionRover::TYPE => {
                RoverAggregate::handle_provision_rover(self, deserialize_json(&arg.payload)?, state)
            }

            SetDestination::TYPE => {
                RoverAggregate::handle_set_destination(self, deserialize_json(&arg.payload)?, state)
            }

            Stop::TYPE => RoverAggregate::handle_stop(self, deserialize_json(&arg.payload)?, state),

            Start::TYPE => {
                RoverAggregate::handle_start(self, deserialize_json(&arg.payload)?, state)
            }

            BuildStructure::TYPE => {
                RoverAggregate::handle_build_structure(self, deserialize_json(&arg.payload)?, state)
            }

            CancelConstruction::TYPE => RoverAggregate::handle_cancel_construction(
                self,
                deserialize_json(&arg.payload)?,
                state,
            ),

            e => {
                error!("Unsupported command type: {e}. Interest configuration for this Aggregate is probably incorect.");
                Ok(vec![])
            }
        }
    }

    async fn apply_event(&self, _ctx: &Context, arg: &EventWithState) -> RpcResult<StateAck> {
        let state: Option<RoverAggregateState> = arg
            .state
            .clone()
            .map(|bytes| deserialize_json(&bytes).unwrap_or_default());

        Ok(match arg.event.event_type.as_str() {
            RoverInitialized::TYPE => RoverAggregate::apply_rover_initialized(
                self,
                deserialize_json(&arg.event.payload)?,
                state,
            )?,
            RoverDestinationChanged::TYPE => RoverAggregate::apply_rover_destination_changed(
                self,
                deserialize_json(&arg.event.payload)?,
                state,
            )?,
            RoverStopped::TYPE => RoverAggregate::apply_rover_stopped(
                self,
                deserialize_json(&arg.event.payload)?,
                state,
            )?,
            RoverStarted::TYPE => RoverAggregate::apply_rover_started(
                self,
                deserialize_json(&arg.event.payload)?,
                state,
            )?,
            ConstructionBegan::TYPE => RoverAggregate::apply_construction_began(
                self,
                deserialize_json(&arg.event.payload)?,
                state,
            )?,
            ConstructionCancelled::TYPE => RoverAggregate::apply_construction_cancelled(
                self,
                deserialize_json(&arg.event.payload)?,
                state,
            )?,
            ConstructionProgressed::TYPE => RoverAggregate::apply_construction_progressed(
                self,
                deserialize_json(&arg.event.payload)?,
                state,
            )?,
            ResourceQuantityChanged::TYPE => RoverAggregate::apply_resource_quantity_changed(
                self,
                deserialize_json(&arg.event.payload)?,
                state,
            )?,
            RoverPositionChanged::TYPE => RoverAggregate::apply_rover_position_changed(
                self,
                deserialize_json(&arg.event.payload)?,
                state,
            )?,
            e => {
                debug!("Non-state-mutating event received '{e}'. Acking and moving on.");
                StateAck::ok(state)
            }
        })
    }
}

fn deserialize_json<'de, T: Deserialize<'de>>(buf: &'de [u8]) -> RpcResult<T> {
    serde_json::from_slice(buf).map_err(|e| format!("Deserialization failure: {e:?}").into())
}

fn serialize_json<T: Serialize>(data: &T) -> RpcResult<Vec<u8>> {
    serde_json::to_vec(data).map_err(|e| format!("Serialization failure: {e:?}").into())
}
