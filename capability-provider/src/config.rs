//! # Concordance configuration
//! This module contains functionality and types for managing configuration of the capability
//! provider both through host data and through link definitions

use async_nats::Command;
use case::CaseExt;
use core::fmt;
use std::collections::HashMap;

use crate::Result;
use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};
use tracing::{error, instrument};
use wasmbus_rpc::core::LinkDefinition;

const ROLE_KEY: &str = "role";
const INTEREST_KEY: &str = "interest";
const ENTITY_NAME_KEY: &str = "name";

const REQUIRED_KEYS: &[&str] = &["role", "interest", "name"];

const ROLE_AGGREGATE: &str = "aggregate";
const ROLE_PROJECTOR: &str = "projector";
const ROLE_PROCESS_MANAGER: &str = "process_manager";
const ROLE_NOTIFIER: &str = "notifier";

#[derive(Clone, Default)]
pub struct BaseConfiguration {}

/// All entities participating in an event sourced system must declare their interest.
/// Aggregates declare interest in the stream that corresponds to their name, notifiers and process managers declare interest
/// in an explicit list of event types.
#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct InterestDeclaration {
    pub actor_id: String,
    pub entity_name: String,
    pub role: ActorRole,
    pub interest: ActorInterest,
    pub interest_constraint: InterestConstraint,
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]

pub enum InterestConstraint {
    Commands,
    Events,
}

impl fmt::Display for InterestConstraint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InterestConstraint::Commands => write!(f, "commands"),
            InterestConstraint::Events => write!(f, "events"),
        }
    }
}

impl InterestDeclaration {
    pub fn aggregate_for_commands(actor_id: &str, entity_name: &str) -> InterestDeclaration {
        InterestDeclaration {
            actor_id: actor_id.to_string(),
            entity_name: entity_name.to_string(),
            role: ActorRole::Aggregate,
            interest_constraint: InterestConstraint::Commands,
            interest: ActorInterest::AggregateStream(entity_name.to_string()),
        }
    }

    pub fn aggregate_for_events(actor_id: &str, entity_name: &str) -> InterestDeclaration {
        InterestDeclaration {
            actor_id: actor_id.to_string(),
            entity_name: entity_name.to_string(),
            role: ActorRole::Aggregate,
            interest_constraint: InterestConstraint::Events,
            interest: ActorInterest::AggregateStream(entity_name.to_string()),
        }
    }

    // per current design, if an entity isn't an aggregate, it will never request commands
    pub fn new(
        actor_id: &str,
        entity_name: &str,
        role: ActorRole,
        interest: ActorInterest,
    ) -> InterestDeclaration {
        InterestDeclaration {
            actor_id: actor_id.to_string(),
            entity_name: entity_name.to_string(),
            role,
            interest_constraint: InterestConstraint::Events,
            interest,
        }
    }

    pub fn from_linkdefinition(
        source: &LinkDefinition,
    ) -> std::result::Result<Vec<InterestDeclaration>, String> {
        if let Some(raw) = LinkConfigurationRaw::from_linkdef(&source) {
            let mut interested_parties = vec![];
            let role = ActorRole::from(raw.role);
            if role == ActorRole::Unknown {
                return Err(format!(
                    "Unknown declared role for actor. Aborting link set."
                ));
            }
            if role == ActorRole::Aggregate {
                interested_parties.push(Self::aggregate_for_commands(&source.actor_id, &raw.name));
                interested_parties.push(Self::aggregate_for_events(&source.actor_id, &raw.name));
            } else {
                interested_parties.push(InterestDeclaration::new(
                    &source.actor_id,
                    &raw.name,
                    role.clone(),
                    ActorInterest::from_role_interest(&raw.interest, &role)
                        .map_err(|e| e.to_string())?,
                ));
            }
            Ok(interested_parties)
        } else {
            Err(format!(
                "Failed to parse valid interest declaration from link definition: {source:?}"
            ))
        }
    }

    pub fn consumer_name(&self) -> String {
        let name = self.entity_name.clone();
        match self.role {
            ActorRole::Aggregate => {
                if let InterestConstraint::Commands = self.interest_constraint {
                    format!("AGG_CMD_{name}")
                } else {
                    format!("AGG_EVT_{name}")
                }
            }
            ActorRole::ProcessManager => {
                format!("PROCMAN_{name}")
            }
            ActorRole::Notifier => {
                format!("NOTIFIER_{name}")
            }
            ActorRole::Projector => {
                format!("PROJ_{name}")
            }
            ActorRole::Unknown => {
                "".to_string() // unknown decls can be used for publish-only entities or are filtered out via error early
            }
        }
    }
}

impl fmt::Display for InterestDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = self.entity_name.clone();
        let roledesc = self.role.to_string();
        let constraint = self.interest_constraint.to_string();
        write!(f, "{} ({}) - source type: {}", name, roledesc, constraint)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct LinkConfigurationRaw {
    pub role: String,
    pub interest: String,
    pub name: String,
}

impl LinkConfigurationRaw {
    pub fn from_linkdef(ld: &LinkDefinition) -> Option<Self> {
        let mut values = HashMap::new();
        for (k, v) in ld.values.iter() {
            values.insert(k.to_lowercase().to_owned(), v.to_owned());
        }
        if let Some(b64) = values.get("config_b64") {
            if let Ok(bytes) = general_purpose::URL_SAFE_NO_PAD.decode(b64.as_bytes()) {
                if let Ok(c) = serde_json::from_slice(&bytes) {
                    return Some(c);
                }
            }
        } else {
            if !REQUIRED_KEYS.iter().all(|k| values.contains_key(*k)) {
                return None;
            } else {
                return Some(LinkConfigurationRaw {
                    role: values
                        .get(ROLE_KEY)
                        .map(|s| s.as_str())
                        .unwrap_or("")
                        .to_lowercase(),
                    interest: values
                        .get(INTEREST_KEY)
                        .map(|s| s.as_str())
                        .unwrap_or("")
                        .to_string(),
                    name: values
                        .get(ENTITY_NAME_KEY)
                        .map(|s| s.as_str())
                        .unwrap_or_default()
                        .to_string(),
                });
            }
        }
        None
    }
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub enum ActorRole {
    Aggregate,
    Projector,
    ProcessManager,
    Notifier,
    Unknown,
}

impl fmt::Display for ActorRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ActorRole::*;
        match self {
            Aggregate => {
                write!(f, "aggregate")
            }
            Projector => {
                write!(f, "projector")
            }
            ProcessManager => {
                write!(f, "process manager")
            }
            Notifier => {
                write!(f, "notifier")
            }
            Unknown => {
                write!(f, "unknown")
            }
        }
    }
}

impl From<String> for ActorRole {
    #[instrument(level = "trace")]
    fn from(source: String) -> Self {
        use ActorRole::*;
        match source.trim().to_lowercase().as_ref() {
            ROLE_AGGREGATE => Aggregate,
            ROLE_NOTIFIER => Notifier,
            ROLE_PROCESS_MANAGER => ProcessManager,
            ROLE_PROJECTOR => Projector,
            u => {
                error!("Unknown role declared: {u}");
                Unknown
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub enum ActorInterest {
    AggregateStream(String),
    EventList(Vec<String>),
    ProcessManager(ProcessManagerLifetime),
    None,
}

impl ActorInterest {
    pub fn from_role_interest(input: &str, role: &ActorRole) -> Result<ActorInterest> {
        match role {
            ActorRole::Aggregate => Ok(ActorInterest::AggregateStream(input.to_string())),
            ActorRole::Notifier | ActorRole::Projector => {
                Ok(ActorInterest::EventList(to_snake_list(input)))
            }
            ActorRole::ProcessManager => Ok(ActorInterest::ProcessManager(
                parse_process_manager_interest(input)?,
            )),
            ActorRole::Unknown => Ok(ActorInterest::None),
        }
    }
}

/// A process manager lifetime defines the life cycle of a long running process. A long running process in this case is any
/// process that occurs over the span of more than one event, and does not necessarily correspond to a length of elapsed
/// time
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize, Hash, Eq)]
pub struct ProcessManagerLifetime {
    start: String,
    advance: Vec<String>,
    stop: Vec<String>,
}

fn parse_process_manager_interest(input: &str) -> Result<ProcessManagerLifetime> {
    serde_json::from_str::<ProcessManagerLifetime>(input)
        .map_err(|e| wasmbus_rpc::error::RpcError::Ser(e.to_string()))
        .map(|lifetime| ProcessManagerLifetime {
            start: lifetime.start.to_snake(),
            advance: lifetime.advance.iter().map(|s| s.to_snake()).collect(),
            stop: lifetime.stop.iter().map(|s| s.to_snake()).collect(),
        })
}

fn to_snake_list(input: &str) -> Vec<String> {
    input
        .split(',')
        .map(|s| s.trim().to_owned())
        .map(|s| s.to_snake())
        .collect()
}

#[cfg(test)]
mod test {
    use super::InterestDeclaration;
    use crate::config::{ActorInterest, ActorRole, ProcessManagerLifetime};
    use std::collections::HashMap;
    use wasmbus_rpc::core::LinkDefinition;

    fn generate_ld(hm: HashMap<String, String>) -> LinkDefinition {
        let mut ld = LinkDefinition::default();

        ld.actor_id = "MAEYUH6M3BIWY5GXHXXUUZNX736AKZ363UY2PQKVHOTHIC2PY2MNVMVA".to_string();
        ld.provider_id = "VAJIAL5WURDEFJLT4HCZS2JD3LRRESRO4PN2ULUKXATIB7PFTLWYYQO6".to_string();
        ld.link_name = "default".to_string();
        ld.contract_id = "cosmonic:evensourcing".to_string();
        ld.values = hm;
        ld
    }

    #[test]
    fn rejects_empty_linkdefinition() {
        let ld = LinkDefinition::default();
        let decl = InterestDeclaration::from_linkdefinition(&ld);

        assert!(decl.is_err());
        let e = decl.err().unwrap();
        assert!(e.starts_with("Failed to parse valid interest declaration from link definition"));
    }

    #[test]
    fn accepts_valid_linkdefinition() {
        let mut hm = HashMap::new();
        hm.insert("ROLE".to_string(), "aggregate".to_string());
        hm.insert("INTEREST".to_string(), "user".to_string());
        hm.insert("NAME".to_string(), "user".to_string());
        let ld = generate_ld(hm);

        let decl = InterestDeclaration::from_linkdefinition(&ld).unwrap();
        // Aggregates produce 2 consumers
        assert_eq!(2, decl.len());
        assert_eq!(
            decl[0].actor_id,
            "MAEYUH6M3BIWY5GXHXXUUZNX736AKZ363UY2PQKVHOTHIC2PY2MNVMVA".to_string()
        );
        assert_eq!(decl[0].role, ActorRole::Aggregate);
        assert_eq!(
            decl[0].interest,
            ActorInterest::AggregateStream("user".to_string())
        );
    }

    #[test]
    fn rejects_bogus_linkdefinition() {
        let mut hm = HashMap::new();
        hm.insert("ROLE".to_string(), "aggregate".to_string());
        hm.insert("INTEREST".to_string(), "bankaccount".to_string());
        let ld = generate_ld(hm);

        let decl = InterestDeclaration::from_linkdefinition(&ld);
        assert!(decl.is_err());
        let e = decl.err().unwrap();
        assert!(e.starts_with("Failed to parse valid interest declaration from link definition:"));
    }

    #[test]
    fn accepts_interest_list() {
        let mut hm = HashMap::new();
        hm.insert("ROLE".to_string(), "notifier".to_string());
        hm.insert(
            "INTEREST".to_string(),
            "order_created,OrderUpdated,orderDeleted".to_string(),
        );
        hm.insert("NAME".to_string(), "order".to_string());
        let ld = generate_ld(hm);
        let decl = &InterestDeclaration::from_linkdefinition(&ld).unwrap()[0];

        assert_eq!(
            decl.interest,
            ActorInterest::EventList(vec![
                "order_created".to_string(),
                "order_updated".to_string(),
                "order_deleted".to_string()
            ])
        );
    }

    #[test]
    fn accepts_process_manager_interest_bad_json() {
        let mut hm = HashMap::new();
        hm.insert("RoLE".to_string(), "process_ManaGeR".to_string());
        hm.insert("InTeResT".to_string(),
            r##"{"start": "orderCreated", "advance": ["orderUpdated", "OrdErShipPeD"], "stop": "OrderCompleted", "order_canceled"]}"##.to_string()
        );
        hm.insert("NAME".to_string(), "order".to_string());
        let ld = generate_ld(hm);
        let decl = InterestDeclaration::from_linkdefinition(&ld);
        assert!(decl.is_err());
    }

    #[test]
    fn accepts_process_manager_interest() {
        let mut hm = HashMap::new();
        hm.insert("RoLE".to_string(), "process_ManaGeR".to_string());
        // WARNING: the conversion to snake case for normalization doesn't cleanly tolerate things like SpoNgeBoB CAsE
        hm.insert("InTeResT".to_string(),
            r##"{"start": "orderCreated", "advance": ["orderUpdated", "OrderShipped"], "stop": ["OrderCompleted", "order_canceled"]}"##.to_string()
        );
        hm.insert("NAME".to_string(), "order".to_string());
        let ld = generate_ld(hm);
        let decl = &InterestDeclaration::from_linkdefinition(&ld).unwrap()[0];

        assert_eq!(
            decl.interest,
            ActorInterest::ProcessManager(ProcessManagerLifetime {
                start: "order_created".to_string(),
                advance: vec!["order_updated".to_string(), "order_shipped".to_string()],
                stop: vec!["order_completed".to_string(), "order_canceled".to_string()]
            })
        );
    }
}
