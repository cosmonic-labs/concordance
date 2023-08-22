use std::{collections::HashMap, fs, path::PathBuf};

use crate::{
    generator::{aggregate, genhandler, procmgr},
    model::trim_summary_name,
};
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use super::{AggregateSummary, Entity, EntityType, GenHandlerSummary, ProcessManagerSummary};

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Debug, Default)]
pub struct EventCatalogModel {
    site: EventCatalogSite,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Debug, Default)]
pub struct EventCatalogSite {
    events: Vec<EventFrontMatter>,
    services: Vec<ServiceFrontMatter>,
    pub(crate) schemas: HashMap<String, serde_json::Value>,
}

impl EventCatalogSite {
    pub fn from_directory(dir: PathBuf) -> Result<EventCatalogSite> {
        EventCatalogSite::new_from_root_path(dir)
    }

    pub fn generate_aggregate(&self, name: &str) -> Result<String> {
        let aggregate_summary = AggregateSummary::new_from_eventcatalog(&self, name)?;

        aggregate::render(&self, &aggregate_summary)
    }

    pub fn generate_process_manager(&self, name: &str) -> Result<String> {
        let procman_summary = ProcessManagerSummary::new_from_eventcatalog(&self, name)?;

        procmgr::render(&self, &procman_summary)
    }

    pub fn generate_general_event_handler(
        &self,
        name: &str,
        entity_type: &EntityType,
    ) -> Result<String> {
        let summary = GenHandlerSummary::new_from_eventcatalog(&self, name, entity_type.clone())?;

        genhandler::render(&self, &summary)
    }

    pub fn get_service(&self, name: &str, entity_type: EntityType) -> Option<&ServiceFrontMatter> {
        let trimmed_target = trim_summary_name(name, &entity_type);
                
        self.services.iter().find(|s| {
            let s_name = trim_summary_name(&s.name, &s.entity_type_from_tags());            

            s_name.eq_ignore_ascii_case(&trimmed_target) && s.entity_type_from_tags() == entity_type
        })
    }

    pub fn get_inbound_outbound(&self, service: &ServiceFrontMatter) -> (Vec<Entity>, Vec<Entity>) {
        let mut inbound = Vec::new();
        let mut outbound = Vec::new();

        for evt in &self.events {
            if let Some(consumers) = &evt.consumers {
                if consumers.contains(&service.name) {
                    inbound.push(Entity {
                        name: evt.name.clone(),
                        description: evt.summary.clone().unwrap_or_default(),
                        entity_type: evt.entity_type_from_tags(),
                        link: "".to_string(), // unused for this purpose
                    });
                }
            }

            if let Some(producers) = &evt.producers {
                if producers.contains(&service.name) {
                    outbound.push(Entity {
                        name: evt.name.clone(),
                        description: evt.summary.clone().unwrap_or_default(),
                        entity_type: evt.entity_type_from_tags(),
                        link: "".to_string(), // unused for this purpose
                    });
                }
            }
        }

        (inbound, outbound)
    }
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Debug, Default)]
pub(crate) struct EventFrontMatter {
    pub name: String,
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub producers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owners: Option<Vec<String>>,
    #[serde(rename = "externalLinks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_links: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

impl EventFrontMatter {
    fn new_from_path(path: &PathBuf) -> Result<EventFrontMatter> {
        let contents = fs::read_to_string(path)?;
        let event_front_matter = EventFrontMatter::try_from(contents.as_str())?;
        Ok(event_front_matter)
    }

    fn entity_type_from_tags(&self) -> EntityType {
        self.tags.as_ref().map_or(EntityType::Unknown, |tags| {
            if tags.iter().any(|t| t.label == "command") {
                EntityType::Command
            } else if tags.iter().any(|t| t.label == "event") {
                EntityType::Event
            } else {
                // Only care about 2 types of event catalog "events" - event or command
                EntityType::Unknown
            }
        })
    }
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Debug)]
pub struct ServiceFrontMatter {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owners: Option<Vec<String>>,
    #[serde(rename = "externalLinks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_links: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

impl ServiceFrontMatter {
    fn new_from_path(path: &PathBuf) -> Result<ServiceFrontMatter> {
        let contents = fs::read_to_string(path)?;
        let event_front_matter = ServiceFrontMatter::try_from(contents.as_str())?;
        Ok(event_front_matter)
    }

    fn entity_type_from_tags(&self) -> EntityType {
        self.tags.as_ref().map_or(EntityType::Unknown, |tags| {
            if tags.iter().any(|t| t.label == "aggregate") {
                EntityType::Aggregate
            } else if tags.iter().any(|t| t.label == "procman") {
                EntityType::ProcessManager
            } else if tags.iter().any(|t| t.label == "projector") {
                EntityType::Projector
            } else if tags.iter().any(|t| t.label == "notifier") {
                EntityType::Notifier
            } else {
                EntityType::Unknown
            }
        })
    }
}

impl EventCatalogSite {
    pub fn new_from_root_path(path: PathBuf) -> Result<EventCatalogSite> {
        let event_dirs = path.join("./events/");
        let mut site = EventCatalogSite::default();

        for entry in fs::read_dir(event_dirs)? {
            let entry = entry?;
            let path = entry.path().join("index.md");
            if path.exists() {
                let event_front_matter = EventFrontMatter::new_from_path(&path)?;
                site.events.push(event_front_matter);
            }

            let path = entry.path().join("schema.json");
            if path.exists() {                
                let contents = fs::read_to_string(path)?;
                let schema: serde_json::Value = serde_json::from_str(&contents)?;                
                site.schemas
                    .insert(schema["title"].as_str().unwrap().to_string(), schema);
            }
        }

        let service_dirs = path.join("./services/");
        for entry in fs::read_dir(service_dirs)? {
            let entry = entry?;
            let path = entry.path().join("index.md");
            if path.exists() {
                let service_front_matter = ServiceFrontMatter::new_from_path(&path)?;
                site.services.push(service_front_matter);
            }
        }        

        Ok(site)
    }
}

impl TryFrom<&str> for EventFrontMatter {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let split: Vec<&str> = s.split("---").collect();
        serde_yaml::from_str(&split[1])
            .map_err(|e| anyhow!("Failed to parse event front matter: {}", e))
    }
}

impl TryFrom<&str> for ServiceFrontMatter {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let split: Vec<&str> = s.split("---").collect();
        serde_yaml::from_str(&split[1])
            .map_err(|e| anyhow!("Failed to parse service front matter: {}", e))
    }
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Debug)]
pub struct Tag {
    label: String,
}
