use std::path::PathBuf;

use anyhow::Result;
use serde::Serialize;

pub mod eventcatalog;

pub use self::eventcatalog::EventCatalogSite;

/// This is the main reusable model for Concordance code generation. This model is designed to
/// read the metadata from an eventcatalog site and generate code for specific event sourcing
/// components as wasmCloud components
pub struct Model {
    pub(crate) catalog: EventCatalogSite,
}

impl Model {
    /// Loads a model from the given path. This path should be the root directory of an eventcatalog web site. Make sure
    /// that all of the events within this site have the `event` tag, commands are tagged as `command`, and that all
    /// of the producers and consumers are defined properly
    pub fn new_from_path(path: PathBuf) -> Result<Model> {
        Ok(Model {
            catalog: EventCatalogSite::from_directory(path)?,
        })
    }

    /// Emits a string containing the required generated trait, implementation, and model code for a process manager
    pub fn generate_process_manager(&self, name: &str) -> Result<String> {
        self.catalog.generate_process_manager(name)
    }

    /// Emits a string containing the required trait, implementation, and model code for an aggregate
    pub fn generate_aggregate(&self, name: &str) -> Result<String> {
        self.catalog.generate_aggregate(name)
    }

    /// Emits a string containing the required trait, implementation, and model code for a general event handler,
    /// which could be a projector or a notifier.
    pub fn generate_general_event_handler(
        &self,
        name: &str,
        entity_type: &EntityType,
    ) -> Result<String> {
        self.catalog
            .generate_general_event_handler(name, entity_type)
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct GenHandlerSummary {
    pub(crate) name: String,
    pub(crate) entity_type: EntityType,
    pub(crate) inbound: Vec<Entity>,
    pub(crate) outbound: Vec<Entity>,
}

impl GenHandlerSummary {
    pub fn new_from_eventcatalog(
        catalog: &EventCatalogSite,
        name: &str,
        entity_type: EntityType,
    ) -> Result<GenHandlerSummary> {
        let service = catalog
            .get_service(name, entity_type.clone())
            .expect(&format!("service '{}' not found", name));
        let (inbound, outbound) = catalog.get_inbound_outbound(service);
        Ok(GenHandlerSummary {
            name: trim_summary_name(&service.name, &entity_type),
            entity_type,
            inbound,
            outbound,
        })
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct AggregateSummary {
    pub name: String,
    pub description: String,
    pub inbound_commands: Vec<Entity>, // utility partition to make certain aggregate rendering actions easier
    pub inbound_events: Vec<Entity>,
}

impl AggregateSummary {
    pub fn new_from_eventcatalog(
        catalog: &EventCatalogSite,
        name: &str,
    ) -> Result<AggregateSummary> {
        let service = catalog
            .get_service(name, EntityType::Aggregate)
            .expect(&format!("service '{}' not found", name));
        let (inbound, _outbound) = catalog.get_inbound_outbound(service);

        let (in_events, in_commands): (Vec<Entity>, Vec<Entity>) = inbound
            .into_iter()
            .partition(|input| input.entity_type == EntityType::Event);

        Ok(AggregateSummary {
            name: trim_summary_name(&service.name, &EntityType::Aggregate),
            description: service.summary.clone().unwrap_or_default(),
            inbound_commands: in_commands,
            inbound_events: in_events,
        })
    }
}

/// Converts "Rover Aggregate" into "rover"
pub(crate) fn trim_summary_name(name: &str, entity_type: &EntityType) -> String {
    let name = name.to_lowercase();
    let mut et = entity_type.to_trait_name().to_lowercase();
    if *entity_type == EntityType::ProcessManager {
        et = "process manager".to_string();
    }
    let newname = name.replace(&et, "");

    newname.trim().to_string()
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

#[derive(Serialize, Debug, Clone)]

pub struct ProcessManagerSummary {
    pub name: String,
    pub description: String,
    pub doc: String,
    pub inbound: Vec<Entity>,
    pub outbound: Vec<Entity>,
}

impl ProcessManagerSummary {
    pub fn new_from_eventcatalog(
        catalog: &EventCatalogSite,
        name: &str,
    ) -> Result<ProcessManagerSummary> {
        let service = catalog
            .get_service(name, EntityType::ProcessManager)
            .expect(&format!("service '{}' not found", name));
        let (inbound, outbound) = catalog.get_inbound_outbound(service);

        Ok(ProcessManagerSummary {
            name: service.name.to_string(),
            description: service.summary.clone().unwrap_or_default(),
            inbound: inbound.clone(),
            outbound: outbound,
            doc: "".to_string(), // TODO
        })
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct Entity {
    pub name: String,
    pub description: String,
    pub link: String,
    pub entity_type: EntityType,
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
}
