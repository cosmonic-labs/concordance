use crate::*;

pub(crate) fn initialize_rover(input: InitializeRover) -> Result<EventList> {
    let event = RoverInitialized {
        moon_id: input.moon_id,
        rover_id: input.rover_id,
        mothership_id: input.mothership_id,
        tick: input.tick as _,
        pilot_key: RoverInitializedPilotKey(input.pilot_key.0),
        position: RoverInitializedPosition {
            x: input.position.x,
            y: input.position.y,
        },
    };
    Ok(vec![Event::new(RoverInitialized::TYPE, STREAM, &event)])
}

pub(crate) fn change_destination(
    input: ChangeDestination,
    state: Option<RoverAggregateState>,
) -> Result<EventList> {
    let Some(state) = state else {
        return Err(anyhow::anyhow!("Cannot set destination. Rover {} not found", input.rover_id));
    };

    // TODO: enforce that new position is a valid destination

    let event = RoverDestinationChanged {
        colony_id: input.colony_id,
        moon_id: input.moon_id,
        mothership_id: input.mothership_id,
        position: RoverDestinationChangedPosition {
            x: input.position.x,
            y: input.position.y,
        },
        rover_id: input.rover_id,
        tick: state.current_tick as _,
    };
    Ok(vec![Event::new(
        RoverDestinationChanged::TYPE,
        STREAM,
        &event,
    )])
}
