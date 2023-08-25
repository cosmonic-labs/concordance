use crate::*;

impl From<RoverInitialized> for RoverAggregateState {
    fn from(input: RoverInitialized) -> Self {
        RoverAggregateState {
            rover_id: input.rover_id,
            position: (input.position.x as _, input.position.y as _),
            moon_id: input.moon_id,
            mothership_id: input.mothership_id,
            pilot_key: input.pilot_key.0,
            destination: None,
            current_tick: input.tick as _,
            moving: false,
        }
    }
}

/*
 * A note on ticks. Per the design of this sample, there is no explicit "tickadvanced" event, which
 * is explained in some blog posts and Kevin's book. Instead, the tick is advanced by the tick field
 * on incoming events, where the aggregate will always keep the highest of the tick values it has seen.
 */

pub(crate) fn apply_rover_initialized(input: RoverInitialized) -> Result<StateAck> {
    Ok(StateAck::ok(input.into()))
}

pub(crate) fn apply_destination_changed(
    input: RoverDestinationChanged,
    state: Option<RoverAggregateState>,
) -> Result<StateAck> {
    let Some(state) = state else {
        return Ok(StateAck::error(
            "Rover does not exist",
            None::<RoverAggregateState>,
        ));
    };

    Ok(StateAck::ok(Some(RoverAggregateState {
        destination: Some((input.position.x as _, input.position.y as _)),
        current_tick: state.current_tick.max(input.tick as _),
        ..state
    })))
}

pub(crate) fn apply_destination_reached(
    input: RoverDestinationReached,
    state: Option<RoverAggregateState>,
) -> Result<StateAck> {
    let Some(state) = state else {
        return Ok(StateAck::error(
            "Rover does not exist",
            None::<RoverAggregateState>,
        ));
    };

    // Rovers auto-stop at their destination and the "old" destination is wiped. Like what happens
    // when a car GPS says "you have reached your destination".
    Ok(StateAck::ok(Some(RoverAggregateState {
        destination: None,
        moving: false,
        current_tick: state.current_tick.max(input.tick as _),
        ..state
    })))
}

pub(crate) fn apply_rover_stopped(
    input: RoverStopped,
    state: Option<RoverAggregateState>,
) -> Result<StateAck> {
    let Some(state) = state else {
        return Ok(StateAck::error(
            "Rover does not exist",
            None::<RoverAggregateState>,
        ));
    };

    Ok(StateAck::ok(Some(RoverAggregateState {
        moving: false,
        position: (input.position.x as _, input.position.y as _),
        current_tick: state.current_tick.max(input.tick as _),
        ..state
    })))
}

pub(crate) fn apply_rover_started(
    input: RoverStarted,
    state: Option<RoverAggregateState>,
) -> Result<StateAck> {
    let Some(state) = state else {
        return Ok(StateAck::error(
            "Rover does not exist",
            None::<RoverAggregateState>,
        ));
    };

    Ok(StateAck::ok(Some(RoverAggregateState {
        moving: true,
        position: (input.position.x as _, input.position.y as _),
        current_tick: state.current_tick.max(input.tick as _),
        ..state
    })))
}

pub(crate) fn apply_position_changed(
    input: RoverPositionChanged,
    state: Option<RoverAggregateState>,
) -> Result<StateAck> {
    let Some(state) = state else {
        return Ok(StateAck::error(
            "Rover does not exist",
            None::<RoverAggregateState>,
        ));
    };

    Ok(StateAck::ok(Some(RoverAggregateState {
        position: (input.position.x as _, input.position.y as _),
        current_tick: state.current_tick.max(input.tick as _),
        ..state
    })))
}
