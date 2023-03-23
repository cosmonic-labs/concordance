namespace com.cosmonic.eventsourcing

use org.wasmcloud.model#wasmbus
use org.wasmcloud.model#U32
use org.wasmcloud.model#U64

use com.cosmonic.eventsourcing#Event
use com.cosmonic.eventsourcing#EventWithState



// ****-
// Aggregates
// An aggregate accepts an incoming command and responds with a list of events to be 
// emitted into the stream
// ****-
@wasmbus(
    contractId: "cosmonic:eventsourcing",
    actorReceive: true
)
service AggregateService {
    version: "0.1",
    operations: [ HandleCommand, ApplyEvent ]
}

operation HandleCommand {
    input: StatefulCommand,
    output: EventList,
}

operation ApplyEvent {
    input: EventWithState,
    output: StateAck
}



list EventList {
    member: Event
}


// This is the response from an aggregate that comes back from handling an event
// If the state returned from handling an event is missing (None in Rust), then
// the state (if exists) will be deleted
structure StateAck {        
    state: Blob,

    @required
    succeeded: Boolean,

    /// Optional error message
    error: String
}


// The stateful command is always specifically directed to an instance of an aggregate, identified
// by the aggregate's name and the unique key. For example, the `Order` aggregate and the `key` field
// contains an order number. This is used by the capability provider and is -not- meant for external
// entities to use directly
structure StatefulCommand {
    // State for an aggregate is optional. If an aggregate with the given key does not yet exist, then
    // this field will be empty/missing
    state: Blob,

    // The unique type for this command. Upon ingestion this will be prefixed and normalized to snake case
    @required
    commandType: String,

    // The raw data for the command. The capability provider treats this value as opaque and never applies
    // meaning to it
    @required
    payload: Blob,        

    // The target name of the aggregate. Name will be sanitized and as such can be mixed case
    @required
    aggregate: String,

    // The unique identifier that corresponds to an instance of the target aggregate. Aggregates do not
    // assign themselves an identifier and so all commands must contain a target ID, even if it does not
    // yet exist
    @required
    key: String
}
