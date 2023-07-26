//! # Concordance configuration
//! This module contains functionality and types for managing configuration of the capability
//! provider both through host data and through link definitions

use case::CaseExt;
use core::fmt;
use std::{collections::HashMap, hash::Hash};

use crate::eventsourcing::Event as ConcordanceEvent;
use crate::natsclient::SEND_TIMEOUT_DURATION;
use crate::Result;
use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};
use tracing::Instrument;
use tracing::{error, instrument};
use wasmbus_rpc::{core::LinkDefinition, wascap::prelude::KeyPair};

const ROLE_KEY: &str = "role";
const INTEREST_KEY: &str = "interest";
const ENTITY_NAME_KEY: &str = "name";
const KEY_FIELD_KEY: &str = "key";
const MAX_MESSAGES_PER_BATCH_KEY: &str = "max_messages_per_batch";

const REQUIRED_KEYS: &[&str] = &["role", "interest", "name"];

const ROLE_AGGREGATE: &str = "aggregate";
const ROLE_PROJECTOR: &str = "projector";
const ROLE_PROCESS_MANAGER: &str = "process_manager";
const ROLE_NOTIFIER: &str = "notifier";

const DEFAULT_BATCH_MAX: usize = 200; // this is the default set by the NATS client when you leave the value off

#[derive(Clone, Serialize, Deserialize)]
pub struct BaseConfiguration {
    /// Address of the NATS server
    pub nats_url: String,
    /// User JWT for connecting to the core NATS
    pub user_jwt: Option<String>,
    /// User seed for connecting to the core NATS
    pub user_seed: Option<String>,
    /// JetStream domain for the JS context used by this provider
    pub js_domain: Option<String>,
}

impl Default for BaseConfiguration {
    fn default() -> Self {
        Self {
            nats_url: "127.0.0.1:4222".to_string(),
            user_jwt: None,
            user_seed: None,
            js_domain: None,
        }
    }
}

impl BaseConfiguration {
    pub async fn get_nats_connection(&self) -> wasmbus_rpc::error::RpcResult<async_nats::Client> {
        let base_opts = match (
            self.user_jwt.clone().unwrap_or_default(),
            self.user_seed.clone().unwrap_or_default(),
        ) {
            (jwt, seed) if !jwt.trim().is_empty() && !seed.trim().is_empty() => {
                let key_pair = std::sync::Arc::new(
                    KeyPair::from_seed(&seed).map_err(|err| format!("key init: {err}"))?,
                );
                async_nats::ConnectOptions::with_jwt(jwt, move |nonce| {
                    let key_pair = key_pair.clone();
                    async move { key_pair.sign(&nonce).map_err(async_nats::AuthError::new) }
                })
            }
            (jwt, seed) if jwt.trim().is_empty() && seed.trim().is_empty() => {
                async_nats::ConnectOptions::default()
            }
            _ => {
                return Err("must provide both jwt and seed for jwt authentication".into());
            }
        };
        let base_opts = base_opts.request_timeout(Some(SEND_TIMEOUT_DURATION));
        Ok(
            wasmbus_rpc::rpc_client::with_connection_event_logging(base_opts)
                .name("Concordance Event Sourcing")
                .connect(&self.nats_url)
                .instrument(tracing::debug_span!("async connect"))
                .await
                .map_err(|e| {
                    format!("failed to make NATS connection to {}: {}", self.nats_url, e)
                })?,
        )
    }
}

/// All entities participating in an event sourced system must declare their interest.
/// Aggregates declare interest in the stream that corresponds to their name, notifiers and process managers declare interest
/// in an explicit list of event types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InterestDeclaration {
    pub actor_id: String,
    pub key_field: String,
    pub entity_name: String,
    pub role: ActorRole,
    pub interest: ActorInterest,
    pub interest_constraint: InterestConstraint,
    pub link_definition: LinkDefinition,
}

impl InterestDeclaration {
    pub fn extract_key_value_from_payload(&self, payload: &serde_json::Value) -> String {
        payload
            .get(&self.key_field)
            .cloned()
            .map(|s| s.as_str().unwrap_or_default().trim().to_string())
            .unwrap_or_default()
    }

    pub fn extract_max_messages_per_batch(&self) -> usize {
        self.link_definition
            .values
            .get(MAX_MESSAGES_PER_BATCH_KEY)
            .map(|s| s.parse::<usize>().unwrap_or(DEFAULT_BATCH_MAX))
            .unwrap_or(DEFAULT_BATCH_MAX)
    }
}

impl Hash for InterestDeclaration {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.actor_id.hash(state);
        self.entity_name.hash(state);
        self.role.hash(state);
        self.interest.hash(state);
        self.interest_constraint.hash(state);
    }
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
    /// Creates an interest declaration for an aggregate.
    pub fn aggregate_for_commands(
        actor_id: &str,
        entity_name: &str,
        key: &str,
        ld: LinkDefinition,
    ) -> InterestDeclaration {
        InterestDeclaration {
            actor_id: actor_id.to_string(),
            entity_name: entity_name.to_string(),
            role: ActorRole::Aggregate,
            key_field: key.to_string(),
            interest_constraint: InterestConstraint::Commands,
            interest: ActorInterest::AggregateStream(entity_name.to_string()),
            link_definition: ld,
        }
    }

    /// Creates an interest declaration for an aggregate.
    pub fn aggregate_for_events(
        actor_id: &str,
        entity_name: &str,
        key: &str,
        ld: LinkDefinition,
    ) -> InterestDeclaration {
        InterestDeclaration {
            actor_id: actor_id.to_string(),
            entity_name: entity_name.to_string(),
            role: ActorRole::Aggregate,
            key_field: key.to_string(),
            interest_constraint: InterestConstraint::Events,
            interest: ActorInterest::AggregateStream(entity_name.to_string()),
            link_definition: ld,
        }
    }

    // per current design, if an entity isn't an aggregate, it will never request commands
    pub fn new(
        actor_id: &str,
        entity_name: &str,
        role: ActorRole,
        key: &str,
        interest: ActorInterest,
        ld: LinkDefinition,
    ) -> InterestDeclaration {
        InterestDeclaration {
            actor_id: actor_id.to_string(),
            entity_name: entity_name.to_string(),
            role,
            key_field: key.to_string(),
            interest_constraint: InterestConstraint::Events,
            interest,
            link_definition: ld,
        }
    }

    pub fn from_linkdefinition(
        source: LinkDefinition,
    ) -> std::result::Result<Vec<InterestDeclaration>, String> {
        let source = lowercase_ld_keys(source);
        if let Some(raw) = LinkConfigurationRaw::from_linkdef(&source) {
            let mut interested_parties = vec![];
            let role = ActorRole::from(raw.role);
            if role == ActorRole::Unknown {
                return Err("Unknown declared role for actor. Aborting link set.".to_string());
            }
            if role == ActorRole::Aggregate {
                interested_parties.push(Self::aggregate_for_commands(
                    &source.actor_id,
                    &raw.name,
                    &raw.key_field,
                    source.clone(),
                ));
                interested_parties.push(Self::aggregate_for_events(
                    &source.actor_id,
                    &raw.name,
                    &raw.key_field,
                    source.clone(),
                ));
            } else {
                interested_parties.push(InterestDeclaration::new(
                    &source.actor_id,
                    &raw.name,
                    role.clone(),
                    &raw.key_field,
                    ActorInterest::from_role_interest(&raw.interest, &role)
                        .map_err(|e| e.to_string())?,
                    source.clone(),
                ));
            }
            Ok(interested_parties)
        } else {
            Err(format!(
                "Failed to parse valid interest declaration from link definition: {source:?}"
            ))
        }
    }

    pub fn is_interested_in_event(&self, event: &ConcordanceEvent) -> bool {
        self.interest
            .is_interested_in_event(&event.event_type, &event.stream)
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
                format!("PM_{name}")
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

fn lowercase_ld_keys(ld: LinkDefinition) -> LinkDefinition {
    let mut values = HashMap::new();
    for (k, v) in ld.values.iter() {
        values.insert(k.to_lowercase().trim().to_owned(), v.to_owned());
    }
    let mut ldnew = LinkDefinition::default();
    ldnew.actor_id = ld.actor_id;
    ldnew.contract_id = ld.contract_id;
    ldnew.link_name = ld.link_name;
    ldnew.provider_id = ld.provider_id;
    ldnew.values = values;

    ldnew
}

impl fmt::Display for InterestDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = self.entity_name.clone();
        let roledesc = self.role.to_string();
        let constraint = self.interest_constraint.to_string();
        write!(
            f,
            "{} ({}) - source type: {}, target: {}",
            name, roledesc, constraint, self.link_definition.actor_id
        )
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
            ActorRole::Aggregate => Ok(ActorInterest::AggregateStream(input.to_snake())),
            ActorRole::Notifier | ActorRole::Projector => {
                Ok(ActorInterest::EventList(to_snake_list(input)))
            }
            ActorRole::ProcessManager => Ok(ActorInterest::ProcessManager(
                parse_process_manager_interest(input)?,
            )),
            ActorRole::Unknown => Ok(ActorInterest::None),
        }
    }

    pub fn is_interested_in_event(&self, event_type: &str, stream: &str) -> bool {
        match self {
            ActorInterest::AggregateStream(s) => stream == s,
            ActorInterest::EventList(list) => list.contains(&event_type.to_string()),
            ActorInterest::ProcessManager(lifetime) => lifetime.is_interested_in_event(event_type),
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct LinkConfigurationRaw {
    pub role: String,
    pub interest: String,
    pub name: String,
    pub key_field: String,
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
        } else if !REQUIRED_KEYS.iter().all(|k| values.contains_key(*k)) {
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
                key_field: values
                    .get(KEY_FIELD_KEY)
                    .map(|s| s.as_str())
                    .unwrap_or_default()
                    .to_string(),
            });
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

/// A process manager lifetime defines the life cycle of a long running process. A long running process in this case is any
/// process that occurs over the span of more than one event, and does not necessarily correspond to a length of elapsed
/// time
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize, Hash, Eq)]
pub struct ProcessManagerLifetime {
    pub start: String,
    pub advance: Vec<String>,
    pub stop: Vec<String>,
}

impl ProcessManagerLifetime {
    pub(crate) fn is_interested_in_event(&self, event_type: &str) -> bool {
        let target = event_type.to_snake();
        self.start == target || self.stop.contains(&target) || self.advance.contains(&target)
    }

    pub(crate) fn event_starts_new_process(&self, event_type: &str) -> bool {
        self.start == event_type.to_snake()
    }
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
    use crate::eventsourcing::Event as ConcordanceEvent;
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
        let decl = InterestDeclaration::from_linkdefinition(ld);

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

        let decl = InterestDeclaration::from_linkdefinition(ld).unwrap();
        // Aggregates produce 2 consumers
        assert_eq!(2, decl.len());
        assert_eq!(
            decl[0].actor_id,
            "MAEYUH6M3BIWY5GXHXXUUZNX736AKZ363UY2PQKVHOTHIC2PY2MNVMVA".to_string()
        );
        assert_eq!(200, decl[0].extract_max_messages_per_batch()); // default is 200
        assert_eq!(decl[0].role, ActorRole::Aggregate);
        assert_eq!(
            decl[0].interest,
            ActorInterest::AggregateStream("user".to_string())
        );
    }

    #[test]
    fn accepts_max_batch_pull_size() {
        let mut hm = HashMap::new();
        hm.insert("ROLE".to_string(), "aggregate".to_string());
        hm.insert("INTEREST".to_string(), "user".to_string());
        hm.insert("NAME".to_string(), "user".to_string());
        hm.insert("maX_meSsaGes_PeR_BaTcH".to_string(), "110".to_string());
        let ld = generate_ld(hm);

        let decl = InterestDeclaration::from_linkdefinition(ld).unwrap();
        // Aggregates produce 2 consumers
        assert_eq!(2, decl.len());
        assert_eq!(
            decl[0].actor_id,
            "MAEYUH6M3BIWY5GXHXXUUZNX736AKZ363UY2PQKVHOTHIC2PY2MNVMVA".to_string()
        );
        assert_eq!(110, decl[0].extract_max_messages_per_batch());
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

        let decl = InterestDeclaration::from_linkdefinition(ld);
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
        let decl = &InterestDeclaration::from_linkdefinition(ld).unwrap()[0];

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
        let decl = InterestDeclaration::from_linkdefinition(ld);
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
        let decl = &InterestDeclaration::from_linkdefinition(ld).unwrap()[0];

        assert_eq!(
            decl.interest,
            ActorInterest::ProcessManager(ProcessManagerLifetime {
                start: "order_created".to_string(),
                advance: vec!["order_updated".to_string(), "order_shipped".to_string()],
                stop: vec!["order_completed".to_string(), "order_canceled".to_string()]
            })
        );
    }

    #[test]
    fn test_interest_paths() {
        // Aggregate happy
        let agg = InterestDeclaration::aggregate_for_events(
            "MXBOB",
            "gameboard",
            "game_id",
            LinkDefinition::default(),
        );
        let ev = ConcordanceEvent {
            event_type: "player_moved".to_string(),
            payload: vec![],
            stream: "gameboard".to_string(),
        };
        assert!(agg.is_interested_in_event(&ev));

        // Aggregate sad
        let ev = ConcordanceEvent {
            event_type: "player_died".to_string(),
            payload: vec![],
            stream: "match".to_string(),
        };
        assert!(!agg.is_interested_in_event(&ev));

        // Procman
        let lifetime = ProcessManagerLifetime {
            start: "game_started".to_string(),
            advance: vec!["turn_advanced".to_string(), "turn_skipped".to_string()],
            stop: vec!["game_finished".to_string(), "game_aborted".to_string()],
        };
        let raw_interest = serde_json::to_string(&lifetime).unwrap();
        let agg = InterestDeclaration::new(
            "MXBOB",
            "gameboard",
            ActorRole::ProcessManager,
            "game_id",
            ActorInterest::from_role_interest(&raw_interest, &ActorRole::ProcessManager)
                .map_err(|e| e.to_string())
                .unwrap(),
            LinkDefinition::default(),
        );
        // let agg = InterestDeclaration::process_manager_for_events(
        //     "MXBOB",
        //     "gameboard",
        //     "game_id",
        //     lifetime,
        //     LinkDefinition::default(),
        // );
        let event_wanted = ConcordanceEvent {
            event_type: "game_started".to_string(),
            stream: "gameboard".to_string(),
            payload: vec![],
        };
        let event_unwanted = ConcordanceEvent {
            event_type: "player_profile_updated".to_string(),
            stream: "gameboard".to_string(),
            payload: vec![],
        };
        assert!(agg.is_interested_in_event(&event_wanted));
        assert!(!agg.is_interested_in_event(&event_unwanted));
    }
}
