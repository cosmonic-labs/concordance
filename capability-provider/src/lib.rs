use wasmbus_rpc::error::RpcResult;

mod config;
mod consumers;
mod events;

#[allow(dead_code)]
mod eventsourcing;

mod natsclient;
mod state;
mod wcprovider;
mod workers;

pub use config::BaseConfiguration;
pub use wcprovider::ConcordanceProvider;

pub type Result<T> = RpcResult<T>;
