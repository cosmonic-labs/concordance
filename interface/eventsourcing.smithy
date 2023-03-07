
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



structure Event {
    @required
    stream: String,

    @required
    key: String,

    @required
    eventType: String,

    @required
    payload: Blob    
}
