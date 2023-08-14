use std::{collections::BTreeMap, path::PathBuf};

use anyhow::Result;
use rdf::reader::rdf_parser::RdfParser;
use rdf::{graph::Graph, node::Node, reader::turtle_parser::TurtleParser, uri::Uri};
use serde::Serialize;

use crate::generator;

#[derive(Debug)]
pub struct Model {
    pub(crate) graph: Graph,
}

impl Model {
    pub fn from_path(path: PathBuf) -> Result<Model> {
        let raw = std::fs::read_to_string(path)?;
        let mut reader = TurtleParser::from_string(raw.to_string());
        let graph = reader.decode().unwrap();

        Ok(Model { graph })
    }

    pub fn from_raw(raw: &str) -> Result<Model> {
        let mut reader = TurtleParser::from_string(raw.to_string());
        let graph = reader.decode().unwrap();

        Ok(Model { graph })
    }

    pub fn generate_aggregate(&self, name: &str) -> Result<String> {
        generator::generate_aggregate(&self, name)
    }

    pub fn generate_process_manager(&self, name: &str) -> Result<String> {
        generator::generate_process_manager(&self, name)
    }

    pub fn generate_general_event_handler(
        &self,
        name: &str,
        entity_type: &EntityType,
    ) -> Result<String> {
        generator::generate_general_event_handler(&self, name, entity_type)
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct AggregateIndex {
    pub aggregates: Vec<AggregateSummary>,
}

#[derive(Serialize, Debug, Clone)]
pub struct EventIndex {
    pub events: Vec<EventSummary>,
}

#[derive(Serialize, Debug, Clone)]
pub struct Notifierindex {
    pub notifiers: Vec<NotifierSummary>,
}

#[derive(Serialize, Debug, Clone)]
pub struct NotifierSummary {
    pub name: String,
    pub description: String,
    pub inbound: Vec<Entity>,
}

impl NotifierSummary {
    pub fn new_from_node(g: &Graph, n: &Node) -> NotifierSummary {
        let entity = Entity::new_from_node(g, n);
        NotifierSummary {
            name: inflector::cases::titlecase::to_title_case(&entity.name),
            description: entity.description,
            inbound: inbound_to_node(g, n),
        }
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct AggregateSummary {
    pub name: String,
    pub description: String,
    pub inbound: Vec<Entity>,
    pub inbound_events: Vec<Entity>, // utility partition to make certain aggregate rendering actions easier
    pub inbound_commands: Vec<Entity>,
    pub outbound: Vec<Entity>,
}

impl AggregateSummary {
    pub fn new_from_node(g: &Graph, n: &Node) -> AggregateSummary {
        let entity = Entity::new_from_node(g, n);
        let inbound = inbound_to_node(g, n);
        let (inbound_commands, inbound_events): (Vec<_>, Vec<_>) = inbound
            .clone()
            .into_iter()
            .partition(|input| input.entity_type == EntityType::Command);
        AggregateSummary {
            name: entity.name.to_string(),
            description: entity.description,
            inbound: inbound_to_node(g, n),
            inbound_commands,
            inbound_events,
            outbound: outbound_from_node(g, n),
        }
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct EventSummary {
    pub name: String,
    pub description: String,
    pub doc: String,
    pub spec: String,
    pub inbound: Vec<Entity>,
    pub outbound: Vec<Entity>,
}

impl EventSummary {
    pub fn new_from_node(g: &Graph, n: &Node) -> EventSummary {
        let entity = Entity::new_from_node(g, n);
        EventSummary {
            name: entity.name.to_string(),
            description: entity.description,
            doc: "".to_string(),  // TODO
            spec: "".to_string(), // TODO
            inbound: inbound_to_node(g, n),
            outbound: outbound_from_node(g, n),
        }
    }
}

#[derive(Serialize, Debug, Clone)]

pub struct ProjectorIndex {
    pub projectors: Vec<ProjectorSummary>,
}

#[derive(Serialize, Debug, Clone)]

pub struct ProjectorSummary {
    pub name: String,
    pub description: String,
    pub doc: String,
    pub inbound: Vec<Entity>,
}

impl ProjectorSummary {
    pub fn new_from_node(g: &Graph, n: &Node) -> ProjectorSummary {
        let entity = Entity::new_from_node(g, n);
        ProjectorSummary {
            name: entity.name.to_string(),
            description: entity.description,
            doc: "".to_string(), // TODO
            inbound: inbound_to_node(g, n),
        }
    }
}

#[derive(Serialize, Debug, Clone)]

pub struct CommandIndex {
    pub commands: Vec<CommandSummary>,
}

#[derive(Serialize, Debug, Clone)]

pub struct CommandSummary {
    pub name: String,
    pub description: String,
    pub doc: String,
    pub inbound: Vec<Entity>,
    pub outbound: Vec<Entity>,
}

impl CommandSummary {
    pub fn new_from_node(g: &Graph, n: &Node) -> CommandSummary {
        let entity = Entity::new_from_node(g, n);
        CommandSummary {
            name: entity.name.to_string(),
            description: entity.description,
            doc: "".to_string(), // TODO
            inbound: inbound_to_node(g, n),
            outbound: outbound_from_node(g, n),
        }
    }
}

#[derive(Serialize, Debug, Clone)]

pub struct ProcessManagerIndex {
    pub process_managers: Vec<ProcessManagerSummary>,
}

#[derive(Serialize, Debug, Clone)]

pub struct ProcessManagerSummary {
    pub name: String,
    pub description: String,
    pub doc: String,
    pub inbound: Vec<Entity>,
    pub outbound: Vec<Entity>,
}

impl ProcessManagerSummary {
    pub fn new_from_node(g: &Graph, n: &Node) -> ProcessManagerSummary {
        let entity = Entity::new_from_node(g, n);
        ProcessManagerSummary {
            name: entity.name.to_string(),
            description: entity.description,
            doc: "".to_string(), // TODO
            inbound: inbound_to_node(g, n),
            outbound: outbound_from_node(g, n),
        }
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct Entity {
    pub name: String,
    pub description: String,
    pub link: String,
    pub entity_type: EntityType,
}

impl Entity {
    pub fn new_from_node(g: &Graph, n: &Node) -> Self {
        let typ = EntityType::from_node(n);
        let name = get_node_name(n);
        Entity {
            description: get_node_description(g, n),
            link: typ.link(&name),
            name: name.to_string(),
            entity_type: typ,
        }
    }
}

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub enum EntityType {
    Event,
    Command,
    Aggregate,
    Projector,
    ProcessManager,
    Notifier,
    Unknown,
}

impl EntityType {
    pub fn to_trait_name(&self) -> String {
        match self {
            EntityType::Command => "Command",
            EntityType::Event => "Event",
            EntityType::Aggregate => "Aggregate",
            EntityType::Projector => "Projector",
            EntityType::ProcessManager => "ProcessManager",
            EntityType::Notifier => "Notifier",
            EntityType::Unknown => "",
        }
        .to_string()
    }

    pub fn prefix(&self) -> String {
        match self {
            EntityType::Command => "command/",
            EntityType::Aggregate => "aggregate/",
            EntityType::Event => "event/",
            EntityType::Notifier => "notifier/",
            EntityType::ProcessManager => "process-manager/",
            EntityType::Projector => "projector/",
            EntityType::Unknown => "__",
        }
        .to_string()
    }

    pub fn link(&self, name: &str) -> String {
        match self {
            EntityType::Command => format!("./cmd_index.md#{}", name),
            EntityType::Event => format!("./evt_index.md#{}", name),
            EntityType::Aggregate => format!("./agg_index.md#{}", name),
            EntityType::Projector => format!("./proj_index.md#{}", name),
            EntityType::ProcessManager => format!("./pm_index.md#{}", name),
            EntityType::Notifier => format!("./notifier_index.md#{}", name),
            EntityType::Unknown => "".to_string(),
        }
    }

    pub fn from_node(n: &Node) -> Self {
        match n {
            Node::UriNode { uri } => {
                let s = uri.to_string();
                if s.starts_with("event/") {
                    EntityType::Event
                } else if s.starts_with("aggregate/") {
                    EntityType::Aggregate
                } else if s.starts_with("command/") {
                    EntityType::Command
                } else if s.starts_with("projector/") {
                    EntityType::Projector
                } else if s.starts_with("process-manager/") {
                    EntityType::ProcessManager
                } else if s.starts_with("notifier/") {
                    EntityType::Notifier
                } else {
                    EntityType::Unknown
                }
            }
            _ => EntityType::Unknown,
        }
    }
}

pub fn get_aggregates(g: &Graph) -> Vec<AggregateSummary> {
    get_nodes_by_prefix(g, "aggregate/")
        .iter()
        .map(|n| AggregateSummary::new_from_node(g, n))
        .collect()
}

pub fn get_events(g: &Graph) -> Vec<EventSummary> {
    get_nodes_by_prefix(g, "event/")
        .iter()
        .map(|n| EventSummary::new_from_node(g, n))
        .collect()
}

pub fn get_commands(g: &Graph) -> Vec<CommandSummary> {
    get_nodes_by_prefix(g, "command/")
        .iter()
        .map(|n| CommandSummary::new_from_node(g, n))
        .collect()
}

pub fn get_process_managers(g: &Graph) -> Vec<ProcessManagerSummary> {
    get_nodes_by_prefix(g, "process-manager/")
        .iter()
        .map(|n| ProcessManagerSummary::new_from_node(g, n))
        .collect()
}

pub fn get_projectors(g: &Graph) -> Vec<ProjectorSummary> {
    get_nodes_by_prefix(g, "projector/")
        .iter()
        .map(|n| ProjectorSummary::new_from_node(g, n))
        .collect()
}

pub fn get_notifiers(g: &Graph) -> Vec<NotifierSummary> {
    get_nodes_by_prefix(g, "notifier/")
        .iter()
        .map(|n| NotifierSummary::new_from_node(g, n))
        .collect()
}

pub fn find_node(g: &Graph, name: &str, prefix: &str) -> Option<Node> {
    get_nodes_by_prefix(g, prefix)
        .into_iter()
        .find(|n| get_node_name(n) == name)
}

pub fn find_entity(g: &Graph, name: &str, prefix: &str) -> Option<Entity> {
    find_node(g, name, prefix).map(|n| Entity::new_from_node(g, &n))
}

// Whether a unique entity appears as an object or a subject, it is unique by its URI
// So this function gathers all unique entities with a given URI prefix and returns the
// unique node for each
fn get_nodes_by_prefix(g: &Graph, prefix: &str) -> Vec<Node> {
    let mut nodes: BTreeMap<String, Node> = BTreeMap::new();

    for t in g.triples_iter() {
        if let Node::UriNode { uri } = t.subject() {
            if uri.to_string().starts_with(prefix) && !nodes.contains_key(uri.to_string()) {
                nodes.insert(uri.to_string().clone(), t.subject().clone());
            }
        }
        if let Node::UriNode { uri } = t.object() {
            if uri.to_string().starts_with(prefix) && !nodes.contains_key(uri.to_string()) {
                nodes.insert(uri.to_string().clone(), t.object().clone());
            }
        }
    }

    nodes.values().cloned().collect()
}

fn get_node_description(g: &Graph, n: &Node) -> String {
    let p = Node::UriNode {
        uri: Uri::new("predicate/desc".to_string()),
    };
    let docs = g.get_triples_with_subject_and_predicate(n, &p);
    if docs.is_empty() {
        "".to_string()
    } else {
        match docs[0].object() {
            Node::LiteralNode { literal: lit, .. } => lit.clone(),
            _ => "".to_string(),
        }
    }
}

pub fn inbound_to_node(g: &Graph, n: &Node) -> Vec<Entity> {
    g.get_triples_with_object(n)
        .iter()
        .map(|t| Entity::new_from_node(g, t.subject()))
        .collect::<Vec<_>>()
}

fn outbound_from_node(g: &Graph, n: &Node) -> Vec<Entity> {
    g.get_triples_with_subject(n)
        .iter()
        .filter(|t| !is_meta_predicate(t.predicate()))
        .map(|t| Entity::new_from_node(g, t.object()))
        .collect::<Vec<_>>()
}

fn is_meta_predicate(n: &Node) -> bool {
    if let Node::UriNode { uri } = n {
        uri.to_string().ends_with("desc")
    } else {
        false
    }
}

fn get_node_name(n: &Node) -> String {
    match n {
        Node::UriNode { uri } => uri.to_string().split("/").last().unwrap().to_string(),
        _ => "".to_string(),
    }
}
