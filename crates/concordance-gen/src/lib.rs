//!
//! # Concordance Code Generation
//! This crate provides an entry point into the code generation macro for concordance. This crate utilizes
//! a `core` crate containing the reusable generation logic and a proc macro crate that contains the actual
//! macro. 
//! 
//! There are a few convenience wrappers around stock Concordance types like `StateAck` and `ProcessManagerAck`, etc.
pub mod eventsourcing;

use eventsourcing::{CommandList, Event, OutputCommand, ProcessManagerAck, StateAck, StatelessAck};

pub use concordance_gen_macro::*;
use serde::Serialize;
use wasmbus_rpc::error::{RpcError, RpcResult};

impl StateAck {
    /// Indicates that a stateful event handler completed successfully. Return `None` for the state to
    /// remove the state from storage
    pub fn ok(state: Option<impl Serialize + Clone>) -> StateAck {
        StateAck {
            succeeded: true,
            error: None,
            state: state
                .clone()
                .map(|s| serde_json::to_vec(&s).unwrap_or_default()),
        }
    }

    /// Indicates a processing failure within a stateful event handler. Note that even when returning
    /// an error, you should still return the most recent _stable_ state wherever appropriate. Do not
    /// return `None` just because there was an error, as that will _erase_ the state from storage.
    pub fn error(msg: &str, state: Option<impl Serialize + Clone>) -> StateAck {
        StateAck {
            succeeded: false,
            error: Some(msg.to_string()),
            state: state
                .clone()
                .map(|s| serde_json::to_vec(&s).unwrap_or_default()),
        }
    }
}

impl ProcessManagerAck {
    /// Ackknowledges successfully a process manager operation. This will create an ack that also contains
    /// the list of output commands to be requested of the given stream
    pub fn ok(state: Option<impl Serialize>, cmds: CommandList) -> Self {
        Self {
            state: state.map(|s| serialize_json(&s).unwrap_or_default()),
            commands: cmds,
        }
    }
}

impl Event {
    /// A convenience wrapper for creating a new event targeting a given stream. Note that aggregates are
    /// the components that care about streams, all others declare their interest on a per-event basis
    pub fn new(event_type: &str, stream: &str, payload: impl Serialize) -> Event {
        Event {
            event_type: event_type.to_string(),
            stream: stream.to_string(),
            payload: serde_json::to_vec(&payload).unwrap_or_default(),
        }
    }
}

impl OutputCommand {
    /// Convenience method for creating an output command from a process manager handler
    pub fn new(cmd_type: &str, payload: &impl Serialize, stream: &str, key: &str) -> Self {
        OutputCommand {
            aggregate_key: key.to_string(),
            aggregate_stream: stream.to_string(),
            command_type: cmd_type.to_string(),
            json_payload: serde_json::to_vec(payload).unwrap_or_default(),
        }
    }
}

// Convenience in case someone wants to call `into` to convert a `Result` into a `StatelessAck`

impl Into<StatelessAck> for Result<(), RpcError> {
    fn into(self) -> StatelessAck {
        match self {
            Ok(_) => StatelessAck::ok(),
            Err(e) => StatelessAck::error(e.to_string()),
        }
    }
}

impl StatelessAck {
    /// Acknowledges a successful stateless handler completion
    pub fn ok() -> Self {
        Self {
            error: None,
            succeeded: true,
        }
    }

    /// Indicates a failure to process an inbound entity by a stateless handler
    pub fn error(msg: String) -> Self {
        Self {
            error: Some(msg),
            succeeded: false,
        }
    }
}

fn serialize_json<T: Serialize>(data: &T) -> RpcResult<Vec<u8>> {
    serde_json::to_vec(data).map_err(|e| format!("Serialization failure: {e:?}").into())
}
