use concordance_es_provider::{BaseConfiguration, ConcordanceProvider};
use wasmbus_rpc::{core::HostData, provider::prelude::*, wascap::prelude::KeyPair};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Concordance bootstrap");
    let config = BaseConfiguration::default();
    let provider = ConcordanceProvider::try_new(config).await?;

    let mut host_data = HostData::default();
    let kp = KeyPair::new_server();
    let prov_kp = KeyPair::new_service();
    host_data.host_id = kp.public_key();
    host_data.invocation_seed = kp.seed().unwrap();
    host_data.provider_key = prov_kp.public_key();
    host_data.instance_id = uuid::Uuid::new_v4().to_string();
    host_data.link_name = "default".to_string();
    host_data.lattice_rpc_prefix = "default".to_string();

    provider_run(
        provider,
        host_data,
        Some("Concordance bootstrap".to_string()),
    )
    .await?;

    eprintln!("Concordance Event Sourcing bootstrap provider exiting");
    Ok(())
}
