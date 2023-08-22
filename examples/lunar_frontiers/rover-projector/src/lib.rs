use anyhow::Result;
use serde::{Deserialize, Serialize};
use wasmcloud_interface_logging::{debug, error};

concordance_gen::generate!({
    path: "../eventcatalog",
    role: "projector",
    entity: "rover"
});

#[async_trait]
impl RoverProjector for RoverProjectorImpl {
    async fn handle_rover_destination_changed(&self, event: RoverDestinationChanged) -> Result<()> {
        debug!("Rover destination changed: {:?}", event);
        Ok(())
    }

    async fn handle_rover_position_changed(&self, event: RoverPositionChanged) -> Result<()> {
        debug!("Rover destination changed: {:?}", event);
        Ok(())
    }

    async fn handle_rover_stopped(&self, event: RoverStopped) -> Result<()> {
        debug!("Rover stopped: {:?}", event);
        Ok(())
    }

    async fn handle_rover_started(&self, event: RoverStarted) -> Result<()> {
        debug!("Rover started: {:?}", event);
        Ok(())
    }

    async fn handle_rover_destination_reached(&self, event: RoverDestinationReached) -> Result<()> {
        debug!("Rover destination reached: {:?}", event);
        Ok(())
    }

    async fn handle_rover_initialized(&self, event: RoverInitialized) -> Result<()> {
        debug!("Rover initialized: {:?}", event);
        Ok(())
    }
}
