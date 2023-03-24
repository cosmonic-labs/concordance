namespace com.cosmonic.eventsourcing

use org.wasmcloud.model#wasmbus
use org.wasmcloud.model#U32
use org.wasmcloud.model#U64

use com.cosmonic.eventsourcing#Event
use com.cosmonic.eventsourcing#EventWithState


// ****-
// Process Managers
// A process manager accepts events from the stream and returns one or more commands to affect
// change to advance or terminate the process
// ****-
@wasmbus(
    contractId: "cosmonic:eventsourcing",
    actorReceive: true
)
service ProcessManagerService {
    version: "0.1",
    operations: [ HandleEvent ]
}

// Accept a state-bearing event and reply with a list of commands to execute along with modified state
operation HandleEvent {
    input: EventWithState,
    output: ProcessManagerAck
}

list CommandList {
    member: OutputCommand
}

structure ProcessManagerAck {    
    state: Blob,

    @required
    commands: CommandList
}

structure OutputCommand {
    @required
    commandType: String,

    @required
    aggregate_stream: String,

    @required
    aggregate_key: String,

    // The JSON payload will be converted into a Rust serde_json::Value and ultimately passed on a RawCommand and published to CC_COMMANDS
    @required
    jsonPayload: Blob,
}
