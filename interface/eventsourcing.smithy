
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


// Represents an internal event
structure Event {
    @required
    stream: String,

    // // Key used to uniquely identify the instance of an interested aggregate. Empty for no key
    // @required
    // aggregate_key: String,

    // // Key used to uniquely identify the instance of an interested process manager. Empty for no key
    // @required
    // pm_key: String,

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

