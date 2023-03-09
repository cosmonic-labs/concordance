use concordance_es_provider::{BaseConfiguration, ConcordanceProvider};
use tracing::{debug, error, info, instrument, trace, warn};
use wasmbus_rpc::provider::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let host_data = load_host_data()?;
    let config: BaseConfiguration = if let Some(c) = host_data.config_json.as_ref() {
        if let Ok(c) = serde_json::from_str(c) {
            c
        } else {
            warn!("Could not deserialize JSON configuration from host data. Using default.");
            BaseConfiguration::default()
        }
    } else {
        warn!("No host data supplied. Using default configuration.");
        BaseConfiguration::default()
    };
    info!("Concordance connecting to NATS at {}", config.nats_url);
    let provider = ConcordanceProvider::try_new(config).await?;

    provider_run(
        provider,
        host_data,
        Some("Concordance Event Sourcing".to_string()),
    )
    .await?;

    eprintln!("Concordance Event Sourcing provider exiting");

    Ok(())
}
