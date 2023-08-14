mod docgen;
mod model;
mod templates;

pub mod eventsourcing;

use std::{
    error::Error,
    fs::{create_dir_all, File},
    io::Write,
    path::PathBuf,
};

use docgen::{
    render_aggregate_list, render_command_list, render_event_list, render_notifier_list,
    render_pm_list, render_projector_list,
};
use eventsourcing::{CommandList, Event, OutputCommand, ProcessManagerAck, StateAck, StatelessAck};
use model::{
    get_aggregates, get_commands, get_events, get_notifiers, get_process_managers, get_projectors,
};
use rdf::reader::{rdf_parser::RdfParser, turtle_parser::TurtleParser};

pub use model::*;

pub use concordance_gen_macro::*;
use serde::Serialize;
use wasmbus_rpc::error::{RpcError, RpcResult};

/// Generate markdown documentation based on the model specification Turtle RDF (ttl) file. This function emits a number of
/// markdown files, an index file for each entity type.
pub fn generate_doc(source: PathBuf, output: PathBuf) -> Result<(), Box<dyn Error>> {
    if !output.exists() {
        create_dir_all(output.clone())?;
    }
    if !output.is_dir() {
        return Err(format!("{:?} is not a directory.", output).into());
    }

    let input = std::fs::read_to_string(source)?;
    let mut reader = TurtleParser::from_string(input.to_string());
    let graph = reader.decode()?;
    let aggregates = get_aggregates(&graph);
    let events = get_events(&graph);
    let commands = get_commands(&graph);
    let pms = get_process_managers(&graph);
    let projectors = get_projectors(&graph);
    let notifiers = get_notifiers(&graph);

    let mut file = File::create(output.join("agg_index.md"))?;
    file.write_all(render_aggregate_list(aggregates)?.as_bytes())?;

    let mut file = File::create(output.join("evt_index.md"))?;
    file.write_all(render_event_list(events)?.as_bytes())?;

    let mut file = File::create(output.join("cmd_index.md"))?;
    file.write_all(render_command_list(commands)?.as_bytes())?;

    let mut file = File::create(output.join("pm_index.md"))?;
    file.write_all(render_pm_list(pms)?.as_bytes())?;

    let mut file = File::create(output.join("proj_index.md"))?;
    file.write_all(render_projector_list(projectors)?.as_bytes())?;

    let mut file = File::create(output.join("notifier_index.md"))?;
    file.write_all(render_notifier_list(notifiers)?.as_bytes())?;

    Ok(())
}

impl StateAck {
    pub fn ok(state: Option<impl Serialize + Clone>) -> StateAck {
        StateAck {
            succeeded: true,
            error: None,
            state: state
                .clone()
                .map(|s| serde_json::to_vec(&s).unwrap_or_default()),
        }
    }

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
    pub fn ok(state: Option<impl Serialize>, cmds: CommandList) -> Self {
        Self {
            state: state.map(|s| serialize_json(&s).unwrap_or_default()),
            commands: cmds,
        }
    }
}

impl Event {
    pub fn new(event_type: &str, stream: &str, payload: impl Serialize) -> Event {
        Event {
            event_type: event_type.to_string(),
            stream: stream.to_string(),
            payload: serde_json::to_vec(&payload).unwrap_or_default(),
        }
    }
}

impl OutputCommand {
    pub fn new(cmd_type: &str, payload: &impl Serialize, stream: &str, key: &str) -> Self {
        OutputCommand {
            aggregate_key: key.to_string(),
            aggregate_stream: stream.to_string(),
            command_type: cmd_type.to_string(),
            json_payload: serde_json::to_vec(payload).unwrap_or_default(),
        }
    }
}

impl Into<StatelessAck> for Result<(), RpcError> {
    fn into(self) -> StatelessAck {
        match self {
            Ok(_) => StatelessAck::ok(),
            Err(e) => StatelessAck::error(e.to_string()),
        }
    }
}

impl StatelessAck {
    pub fn ok() -> Self {
        Self {
            error: None,
            succeeded: true,
        }
    }

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
