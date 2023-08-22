use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "./templates"]
pub(crate) struct Asset;
