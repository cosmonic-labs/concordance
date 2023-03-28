
// Tell the code generator how to reference symbols defined in this namespace
metadata package = [{
    namespace: "com.cosmonic.eventsourcing",
    crate: "cosmonic_eventsourcing",
    py_module: "cosmonic_eventsourcing"
}]

namespace com.cosmonic.eventsourcing

use org.wasmcloud.model#wasmbus
use org.wasmcloud.model#U32
use org.wasmcloud.model#U64

// A stateless event handler service can be used for any interested entity that
// does not need its state reconstituted before or after event application. This can
// be a notifier or a projector under the current set of terminology.
@wasmbus(
    contractId: "cosmonic:eventsourcing",
    actorReceive: true
)
service StatelessEventHandlerService {
    version: "0.1",
    operations: [ ApplyStatelessEvent ]
}


// Applies an event to the the handler and obtains a simple ack in response
operation ApplyStatelessEvent {
    input: Event,
    output: StatelessAck,
}


// Represents an internal event
structure Event {
    @required
    stream: String,
   
    @required
    eventType: String,

    @required
    payload: Blob    
}

// This is passed to an aggregate or a process manager to allow it to apply the event to a given state. 
// The handler must be a pure function such that f(event, state) = state', neither aggregates nor process managers
// can perform side effects while processing events
structure EventWithState {
    // Event to be applied
    @required
    event: Event,

    // Aggregate or PM state to which the new event is applied    
    state: Blob    
}

// Returned from a general event application by a notifier or projector
structure StatelessAck {
    @required
    succeeded: Boolean,

    /// Optional error message
    error: String
}
