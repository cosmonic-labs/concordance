use wasmbus_rpc::error::RpcResult;

mod config;
mod consumers;
mod eventsourcing;
mod natsclient;
mod process_manager;
mod router;
mod wcprovider;

pub type Result<T> = RpcResult<T>;
