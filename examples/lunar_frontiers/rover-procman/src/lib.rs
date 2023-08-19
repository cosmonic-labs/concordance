use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct RoverPilotProcessManagerState {
    pub placeholder: u16,
}

concordance_gen::generate!({
    path: "../eventcatalog",
    role: "process_manager",
    entity: "rover pilot"
});

#[async_trait]
impl RoverPilotProcessManager for RoverPilotProcessManagerImpl {
    async fn handle_rover_destination_changed(
        &self,
        input: RoverDestinationChanged,
        state: Option<RoverPilotProcessManagerState>,
    ) -> RpcResult<ProcessManagerAck> {
        todo!()
    }

    async fn handle_rover_position_changed(
        &self,
        input: RoverPositionChanged,
        state: Option<RoverPilotProcessManagerState>,
    ) -> RpcResult<ProcessManagerAck> {
        todo!()
    }

    async fn handle_rover_stopped(
        &self,
        input: RoverStopped,
        state: Option<RoverPilotProcessManagerState>,
    ) -> RpcResult<ProcessManagerAck> {
        todo!()
    }

    async fn handle_rover_started(
        &self,
        input: RoverStarted,
        state: Option<RoverPilotProcessManagerState>,
    ) -> RpcResult<ProcessManagerAck> {
        todo!()
    }

    async fn handle_construction_began(
        &self,
        input: ConstructionBegan,
        state: Option<RoverPilotProcessManagerState>,
    ) -> RpcResult<ProcessManagerAck> {
        todo!()
    }

    async fn handle_construction_cancelled(
        &self,
        input: ConstructionCancelled,
        state: Option<RoverPilotProcessManagerState>,
    ) -> RpcResult<ProcessManagerAck> {
        todo!()
    }

    async fn handle_rover_destination_reached(
        &self,
        input: RoverDestinationReached,
        state: Option<RoverPilotProcessManagerState>,
    ) -> RpcResult<ProcessManagerAck> {
        todo!()
    }

    async fn handle_rover_initialized(
        &self,
        input: RoverInitialized,
        state: Option<RoverPilotProcessManagerState>,
    ) -> RpcResult<ProcessManagerAck> {
        todo!()
    }

    async fn handle_construction_completed(
        &self,
        input: ConstructionCompleted,
        state: Option<RoverPilotProcessManagerState>,
    ) -> RpcResult<ProcessManagerAck> {
        todo!()
    }
}
