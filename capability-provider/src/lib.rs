use wasmbus_rpc::error::RpcResult;

mod config;
mod consumers;
mod events;
mod eventsourcing;
mod natsclient;
mod state;
mod wcprovider;

pub use config::BaseConfiguration;
pub use wcprovider::ConcordanceProvider;

pub type Result<T> = RpcResult<T>;
