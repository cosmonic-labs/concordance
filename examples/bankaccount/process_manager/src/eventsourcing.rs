// This file is @generated by wasmcloud/weld-codegen 0.7.0.
// It is not intended for manual editing.
// namespace: com.cosmonic.eventsourcing

#[allow(unused_imports)]
use async_trait::async_trait;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::{borrow::Borrow, borrow::Cow, io::Write, string::ToString};
#[allow(unused_imports)]
use wasmbus_rpc::{
    cbor::*,
    common::{
        deserialize, message_format, serialize, Context, Message, MessageDispatch, MessageFormat,
        SendOpts, Transport,
    },
    error::{RpcError, RpcResult},
    Timestamp,
};

#[allow(dead_code)]
pub const SMITHY_VERSION: &str = "1.0";

pub type CommandList = Vec<OutputCommand>;

// Encode CommandList as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_command_list<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &CommandList,
) -> RpcResult<()>
where
    <W as wasmbus_rpc::cbor::Write>::Error: std::fmt::Display,
{
    e.array(val.len() as u64)?;
    for item in val.iter() {
        encode_output_command(e, item)?;
    }
    Ok(())
}

// Decode CommandList from cbor input stream
#[doc(hidden)]
pub fn decode_command_list(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<CommandList, RpcError> {
    let __result = {
        if let Some(n) = d.array()? {
            let mut arr: Vec<OutputCommand> = Vec::with_capacity(n as usize);
            for _ in 0..(n as usize) {
                arr.push(decode_output_command(d).map_err(|e| {
                    format!("decoding 'com.cosmonic.eventsourcing#OutputCommand': {}", e)
                })?)
            }
            arr
        } else {
            // indefinite array
            let mut arr: Vec<OutputCommand> = Vec::new();
            loop {
                match d.datatype() {
                    Err(_) => break,
                    Ok(wasmbus_rpc::cbor::Type::Break) => break,
                    Ok(_) => arr.push(decode_output_command(d).map_err(|e| {
                        format!("decoding 'com.cosmonic.eventsourcing#OutputCommand': {}", e)
                    })?),
                }
            }
            arr
        }
    };
    Ok(__result)
}
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Event {
    #[serde(rename = "eventType")]
    #[serde(default)]
    pub event_type: String,
    #[serde(with = "serde_bytes")]
    #[serde(default)]
    pub payload: Vec<u8>,
    #[serde(default)]
    pub stream: String,
}

// Encode Event as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_event<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &Event,
) -> RpcResult<()>
where
    <W as wasmbus_rpc::cbor::Write>::Error: std::fmt::Display,
{
    e.map(3)?;
    e.str("eventType")?;
    e.str(&val.event_type)?;
    e.str("payload")?;
    e.bytes(&val.payload)?;
    e.str("stream")?;
    e.str(&val.stream)?;
    Ok(())
}

// Decode Event from cbor input stream
#[doc(hidden)]
pub fn decode_event(d: &mut wasmbus_rpc::cbor::Decoder<'_>) -> Result<Event, RpcError> {
    let __result = {
        let mut event_type: Option<String> = None;
        let mut payload: Option<Vec<u8>> = None;
        let mut stream: Option<String> = None;

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct Event, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.fixed_array()?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => event_type = Some(d.str()?.to_string()),
                    1 => payload = Some(d.bytes()?.to_vec()),
                    2 => stream = Some(d.str()?.to_string()),
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.fixed_map()?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "eventType" => event_type = Some(d.str()?.to_string()),
                    "payload" => payload = Some(d.bytes()?.to_vec()),
                    "stream" => stream = Some(d.str()?.to_string()),
                    _ => d.skip()?,
                }
            }
        }
        Event {
            event_type: if let Some(__x) = event_type {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field Event.event_type (#0)".to_string(),
                ));
            },

            payload: if let Some(__x) = payload {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field Event.payload (#1)".to_string(),
                ));
            },

            stream: if let Some(__x) = stream {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field Event.stream (#2)".to_string(),
                ));
            },
        }
    };
    Ok(__result)
}
pub type EventList = Vec<Event>;

// Encode EventList as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_event_list<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &EventList,
) -> RpcResult<()>
where
    <W as wasmbus_rpc::cbor::Write>::Error: std::fmt::Display,
{
    e.array(val.len() as u64)?;
    for item in val.iter() {
        encode_event(e, item)?;
    }
    Ok(())
}

// Decode EventList from cbor input stream
#[doc(hidden)]
pub fn decode_event_list(d: &mut wasmbus_rpc::cbor::Decoder<'_>) -> Result<EventList, RpcError> {
    let __result =
        {
            if let Some(n) = d.array()? {
                let mut arr: Vec<Event> = Vec::with_capacity(n as usize);
                for _ in 0..(n as usize) {
                    arr.push(decode_event(d).map_err(|e| {
                        format!("decoding 'com.cosmonic.eventsourcing#Event': {}", e)
                    })?)
                }
                arr
            } else {
                // indefinite array
                let mut arr: Vec<Event> = Vec::new();
                loop {
                    match d.datatype() {
                        Err(_) => break,
                        Ok(wasmbus_rpc::cbor::Type::Break) => break,
                        Ok(_) => arr.push(decode_event(d).map_err(|e| {
                            format!("decoding 'com.cosmonic.eventsourcing#Event': {}", e)
                        })?),
                    }
                }
                arr
            }
        };
    Ok(__result)
}
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct EventWithState {
    pub event: Event,
    #[serde(with = "serde_bytes")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<Vec<u8>>,
}

// Encode EventWithState as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_event_with_state<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &EventWithState,
) -> RpcResult<()>
where
    <W as wasmbus_rpc::cbor::Write>::Error: std::fmt::Display,
{
    e.map(2)?;
    e.str("event")?;
    encode_event(e, &val.event)?;
    if let Some(val) = val.state.as_ref() {
        e.str("state")?;
        e.bytes(val)?;
    } else {
        e.null()?;
    }
    Ok(())
}

// Decode EventWithState from cbor input stream
#[doc(hidden)]
pub fn decode_event_with_state(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<EventWithState, RpcError> {
    let __result = {
        let mut event: Option<Event> = None;
        let mut state: Option<Option<Vec<u8>>> = Some(None);

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct EventWithState, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.fixed_array()?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => {
                        event = Some(decode_event(d).map_err(|e| {
                            format!("decoding 'com.cosmonic.eventsourcing#Event': {}", e)
                        })?)
                    }
                    1 => {
                        state = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.bytes()?.to_vec()))
                        }
                    }

                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.fixed_map()?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "event" => {
                        event = Some(decode_event(d).map_err(|e| {
                            format!("decoding 'com.cosmonic.eventsourcing#Event': {}", e)
                        })?)
                    }
                    "state" => {
                        state = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.bytes()?.to_vec()))
                        }
                    }
                    _ => d.skip()?,
                }
            }
        }
        EventWithState {
            event: if let Some(__x) = event {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field EventWithState.event (#0)".to_string(),
                ));
            },
            state: state.unwrap(),
        }
    };
    Ok(__result)
}
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct OutputCommand {
    #[serde(rename = "commandType")]
    #[serde(default)]
    pub command_type: String,
    #[serde(rename = "jsonPayload")]
    #[serde(with = "serde_bytes")]
    #[serde(default)]
    pub json_payload: Vec<u8>,
    #[serde(default)]
    pub key: String,
}

// Encode OutputCommand as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_output_command<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &OutputCommand,
) -> RpcResult<()>
where
    <W as wasmbus_rpc::cbor::Write>::Error: std::fmt::Display,
{
    e.map(3)?;
    e.str("commandType")?;
    e.str(&val.command_type)?;
    e.str("jsonPayload")?;
    e.bytes(&val.json_payload)?;
    e.str("key")?;
    e.str(&val.key)?;
    Ok(())
}

// Decode OutputCommand from cbor input stream
#[doc(hidden)]
pub fn decode_output_command(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<OutputCommand, RpcError> {
    let __result = {
        let mut command_type: Option<String> = None;
        let mut json_payload: Option<Vec<u8>> = None;
        let mut key: Option<String> = None;

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct OutputCommand, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.fixed_array()?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => command_type = Some(d.str()?.to_string()),
                    1 => json_payload = Some(d.bytes()?.to_vec()),
                    2 => key = Some(d.str()?.to_string()),
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.fixed_map()?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "commandType" => command_type = Some(d.str()?.to_string()),
                    "jsonPayload" => json_payload = Some(d.bytes()?.to_vec()),
                    "key" => key = Some(d.str()?.to_string()),
                    _ => d.skip()?,
                }
            }
        }
        OutputCommand {
            command_type: if let Some(__x) = command_type {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field OutputCommand.command_type (#0)".to_string(),
                ));
            },

            json_payload: if let Some(__x) = json_payload {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field OutputCommand.json_payload (#1)".to_string(),
                ));
            },

            key: if let Some(__x) = key {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field OutputCommand.key (#2)".to_string(),
                ));
            },
        }
    };
    Ok(__result)
}
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct ProcessManagerAck {
    pub commands: CommandList,
    #[serde(with = "serde_bytes")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<Vec<u8>>,
}

// Encode ProcessManagerAck as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_process_manager_ack<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &ProcessManagerAck,
) -> RpcResult<()>
where
    <W as wasmbus_rpc::cbor::Write>::Error: std::fmt::Display,
{
    e.map(2)?;
    e.str("commands")?;
    encode_command_list(e, &val.commands)?;
    if let Some(val) = val.state.as_ref() {
        e.str("state")?;
        e.bytes(val)?;
    } else {
        e.null()?;
    }
    Ok(())
}

// Decode ProcessManagerAck from cbor input stream
#[doc(hidden)]
pub fn decode_process_manager_ack(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<ProcessManagerAck, RpcError> {
    let __result = {
        let mut commands: Option<CommandList> = None;
        let mut state: Option<Option<Vec<u8>>> = Some(None);

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct ProcessManagerAck, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.fixed_array()?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => {
                        commands = Some(decode_command_list(d).map_err(|e| {
                            format!("decoding 'com.cosmonic.eventsourcing#CommandList': {}", e)
                        })?)
                    }
                    1 => {
                        state = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.bytes()?.to_vec()))
                        }
                    }

                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.fixed_map()?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "commands" => {
                        commands = Some(decode_command_list(d).map_err(|e| {
                            format!("decoding 'com.cosmonic.eventsourcing#CommandList': {}", e)
                        })?)
                    }
                    "state" => {
                        state = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.bytes()?.to_vec()))
                        }
                    }
                    _ => d.skip()?,
                }
            }
        }
        ProcessManagerAck {
            commands: if let Some(__x) = commands {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field ProcessManagerAck.commands (#0)".to_string(),
                ));
            },
            state: state.unwrap(),
        }
    };
    Ok(__result)
}
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct StateAck {
    /// Optional error message
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(with = "serde_bytes")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<Vec<u8>>,
    #[serde(default)]
    pub succeeded: bool,
}

// Encode StateAck as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_state_ack<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &StateAck,
) -> RpcResult<()>
where
    <W as wasmbus_rpc::cbor::Write>::Error: std::fmt::Display,
{
    e.map(3)?;
    if let Some(val) = val.error.as_ref() {
        e.str("error")?;
        e.str(val)?;
    } else {
        e.null()?;
    }
    if let Some(val) = val.state.as_ref() {
        e.str("state")?;
        e.bytes(val)?;
    } else {
        e.null()?;
    }
    e.str("succeeded")?;
    e.bool(val.succeeded)?;
    Ok(())
}

// Decode StateAck from cbor input stream
#[doc(hidden)]
pub fn decode_state_ack(d: &mut wasmbus_rpc::cbor::Decoder<'_>) -> Result<StateAck, RpcError> {
    let __result = {
        let mut error: Option<Option<String>> = Some(None);
        let mut state: Option<Option<Vec<u8>>> = Some(None);
        let mut succeeded: Option<bool> = None;

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct StateAck, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.fixed_array()?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => {
                        error = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    1 => {
                        state = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.bytes()?.to_vec()))
                        }
                    }
                    2 => succeeded = Some(d.bool()?),
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.fixed_map()?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "error" => {
                        error = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    "state" => {
                        state = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.bytes()?.to_vec()))
                        }
                    }
                    "succeeded" => succeeded = Some(d.bool()?),
                    _ => d.skip()?,
                }
            }
        }
        StateAck {
            error: error.unwrap(),
            state: state.unwrap(),

            succeeded: if let Some(__x) = succeeded {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field StateAck.succeeded (#2)".to_string(),
                ));
            },
        }
    };
    Ok(__result)
}
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct StatefulCommand {
    #[serde(default)]
    pub aggregate: String,
    #[serde(rename = "commandType")]
    #[serde(default)]
    pub command_type: String,
    #[serde(default)]
    pub key: String,
    #[serde(with = "serde_bytes")]
    #[serde(default)]
    pub payload: Vec<u8>,
    #[serde(with = "serde_bytes")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<Vec<u8>>,
}

// Encode StatefulCommand as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_stateful_command<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &StatefulCommand,
) -> RpcResult<()>
where
    <W as wasmbus_rpc::cbor::Write>::Error: std::fmt::Display,
{
    e.map(5)?;
    e.str("aggregate")?;
    e.str(&val.aggregate)?;
    e.str("commandType")?;
    e.str(&val.command_type)?;
    e.str("key")?;
    e.str(&val.key)?;
    e.str("payload")?;
    e.bytes(&val.payload)?;
    if let Some(val) = val.state.as_ref() {
        e.str("state")?;
        e.bytes(val)?;
    } else {
        e.null()?;
    }
    Ok(())
}

// Decode StatefulCommand from cbor input stream
#[doc(hidden)]
pub fn decode_stateful_command(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<StatefulCommand, RpcError> {
    let __result = {
        let mut aggregate: Option<String> = None;
        let mut command_type: Option<String> = None;
        let mut key: Option<String> = None;
        let mut payload: Option<Vec<u8>> = None;
        let mut state: Option<Option<Vec<u8>>> = Some(None);

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct StatefulCommand, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.fixed_array()?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => aggregate = Some(d.str()?.to_string()),
                    1 => command_type = Some(d.str()?.to_string()),
                    2 => key = Some(d.str()?.to_string()),
                    3 => payload = Some(d.bytes()?.to_vec()),
                    4 => {
                        state = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.bytes()?.to_vec()))
                        }
                    }

                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.fixed_map()?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "aggregate" => aggregate = Some(d.str()?.to_string()),
                    "commandType" => command_type = Some(d.str()?.to_string()),
                    "key" => key = Some(d.str()?.to_string()),
                    "payload" => payload = Some(d.bytes()?.to_vec()),
                    "state" => {
                        state = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.bytes()?.to_vec()))
                        }
                    }
                    _ => d.skip()?,
                }
            }
        }
        StatefulCommand {
            aggregate: if let Some(__x) = aggregate {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field StatefulCommand.aggregate (#0)".to_string(),
                ));
            },

            command_type: if let Some(__x) = command_type {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field StatefulCommand.command_type (#1)".to_string(),
                ));
            },

            key: if let Some(__x) = key {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field StatefulCommand.key (#2)".to_string(),
                ));
            },

            payload: if let Some(__x) = payload {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field StatefulCommand.payload (#3)".to_string(),
                ));
            },
            state: state.unwrap(),
        }
    };
    Ok(__result)
}
/// wasmbus.contractId: cosmonic:eventsourcing
/// wasmbus.actorReceive
#[async_trait]
pub trait AggregateService {
    /// returns the capability contract id for this interface
    fn contract_id() -> &'static str {
        "cosmonic:eventsourcing"
    }
    async fn handle_command(&self, ctx: &Context, arg: &StatefulCommand) -> RpcResult<EventList>;
    async fn apply_event(&self, ctx: &Context, arg: &EventWithState) -> RpcResult<StateAck>;
}

/// AggregateServiceReceiver receives messages defined in the AggregateService service trait
#[doc(hidden)]
#[async_trait]
pub trait AggregateServiceReceiver: MessageDispatch + AggregateService {
    async fn dispatch(&self, ctx: &Context, message: Message<'_>) -> Result<Vec<u8>, RpcError> {
        match message.method {
            "HandleCommand" => {
                let value: StatefulCommand = wasmbus_rpc::common::deserialize(&message.arg)
                    .map_err(|e| RpcError::Deser(format!("'StatefulCommand': {}", e)))?;

                let resp = AggregateService::handle_command(self, ctx, &value).await?;
                let buf = wasmbus_rpc::common::serialize(&resp)?;

                Ok(buf)
            }
            "ApplyEvent" => {
                let value: EventWithState = wasmbus_rpc::common::deserialize(&message.arg)
                    .map_err(|e| RpcError::Deser(format!("'EventWithState': {}", e)))?;

                let resp = AggregateService::apply_event(self, ctx, &value).await?;
                let buf = wasmbus_rpc::common::serialize(&resp)?;

                Ok(buf)
            }
            _ => Err(RpcError::MethodNotHandled(format!(
                "AggregateService::{}",
                message.method
            ))),
        }
    }
}

/// AggregateServiceSender sends messages to a AggregateService service
/// client for sending AggregateService messages
#[derive(Clone, Debug)]
pub struct AggregateServiceSender<T: Transport> {
    transport: T,
}

impl<T: Transport> AggregateServiceSender<T> {
    /// Constructs a AggregateServiceSender with the specified transport
    pub fn via(transport: T) -> Self {
        Self { transport }
    }

    pub fn set_timeout(&self, interval: std::time::Duration) {
        self.transport.set_timeout(interval);
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl<'send> AggregateServiceSender<wasmbus_rpc::provider::ProviderTransport<'send>> {
    /// Constructs a Sender using an actor's LinkDefinition,
    /// Uses the provider's HostBridge for rpc
    pub fn for_actor(ld: &'send wasmbus_rpc::core::LinkDefinition) -> Self {
        Self {
            transport: wasmbus_rpc::provider::ProviderTransport::new(ld, None),
        }
    }
}
#[cfg(target_arch = "wasm32")]
impl AggregateServiceSender<wasmbus_rpc::actor::prelude::WasmHost> {
    /// Constructs a client for actor-to-actor messaging
    /// using the recipient actor's public key
    pub fn to_actor(actor_id: &str) -> Self {
        let transport =
            wasmbus_rpc::actor::prelude::WasmHost::to_actor(actor_id.to_string()).unwrap();
        Self { transport }
    }
}
#[async_trait]
impl<T: Transport + std::marker::Sync + std::marker::Send> AggregateService
    for AggregateServiceSender<T>
{
    #[allow(unused)]
    async fn handle_command(&self, ctx: &Context, arg: &StatefulCommand) -> RpcResult<EventList> {
        let buf = wasmbus_rpc::common::serialize(arg)?;

        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "AggregateService.HandleCommand",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: EventList = wasmbus_rpc::common::deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("'{}': EventList", e)))?;
        Ok(value)
    }
    #[allow(unused)]
    async fn apply_event(&self, ctx: &Context, arg: &EventWithState) -> RpcResult<StateAck> {
        let buf = wasmbus_rpc::common::serialize(arg)?;

        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "AggregateService.ApplyEvent",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: StateAck = wasmbus_rpc::common::deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("'{}': StateAck", e)))?;
        Ok(value)
    }
}

/// wasmbus.contractId: cosmonic:eventsourcing
/// wasmbus.actorReceive
#[async_trait]
pub trait ProcessManagerService {
    /// returns the capability contract id for this interface
    fn contract_id() -> &'static str {
        "cosmonic:eventsourcing"
    }
    async fn handle_event(
        &self,
        ctx: &Context,
        arg: &EventWithState,
    ) -> RpcResult<ProcessManagerAck>;
}

/// ProcessManagerServiceReceiver receives messages defined in the ProcessManagerService service trait
#[doc(hidden)]
#[async_trait]
pub trait ProcessManagerServiceReceiver: MessageDispatch + ProcessManagerService {
    async fn dispatch(&self, ctx: &Context, message: Message<'_>) -> Result<Vec<u8>, RpcError> {
        match message.method {
            "HandleEvent" => {
                let value: EventWithState = wasmbus_rpc::common::deserialize(&message.arg)
                    .map_err(|e| RpcError::Deser(format!("'EventWithState': {}", e)))?;

                let resp = ProcessManagerService::handle_event(self, ctx, &value).await?;
                let buf = wasmbus_rpc::common::serialize(&resp)?;

                Ok(buf)
            }
            _ => Err(RpcError::MethodNotHandled(format!(
                "ProcessManagerService::{}",
                message.method
            ))),
        }
    }
}

/// ProcessManagerServiceSender sends messages to a ProcessManagerService service
/// client for sending ProcessManagerService messages
#[derive(Clone, Debug)]
pub struct ProcessManagerServiceSender<T: Transport> {
    transport: T,
}

impl<T: Transport> ProcessManagerServiceSender<T> {
    /// Constructs a ProcessManagerServiceSender with the specified transport
    pub fn via(transport: T) -> Self {
        Self { transport }
    }

    pub fn set_timeout(&self, interval: std::time::Duration) {
        self.transport.set_timeout(interval);
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl<'send> ProcessManagerServiceSender<wasmbus_rpc::provider::ProviderTransport<'send>> {
    /// Constructs a Sender using an actor's LinkDefinition,
    /// Uses the provider's HostBridge for rpc
    pub fn for_actor(ld: &'send wasmbus_rpc::core::LinkDefinition) -> Self {
        Self {
            transport: wasmbus_rpc::provider::ProviderTransport::new(ld, None),
        }
    }
}
#[cfg(target_arch = "wasm32")]
impl ProcessManagerServiceSender<wasmbus_rpc::actor::prelude::WasmHost> {
    /// Constructs a client for actor-to-actor messaging
    /// using the recipient actor's public key
    pub fn to_actor(actor_id: &str) -> Self {
        let transport =
            wasmbus_rpc::actor::prelude::WasmHost::to_actor(actor_id.to_string()).unwrap();
        Self { transport }
    }
}
#[async_trait]
impl<T: Transport + std::marker::Sync + std::marker::Send> ProcessManagerService
    for ProcessManagerServiceSender<T>
{
    #[allow(unused)]
    async fn handle_event(
        &self,
        ctx: &Context,
        arg: &EventWithState,
    ) -> RpcResult<ProcessManagerAck> {
        let buf = wasmbus_rpc::common::serialize(arg)?;

        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "ProcessManagerService.HandleEvent",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: ProcessManagerAck = wasmbus_rpc::common::deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("'{}': ProcessManagerAck", e)))?;
        Ok(value)
    }
}
