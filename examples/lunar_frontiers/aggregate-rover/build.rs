use concordance_gen::EntityType;

const CONFIG: &str = "./codegen.toml";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    weld_codegen::rust_build(CONFIG)?;

    // Generate the relevant traits
    concordance_gen::generate_system_traits("../model/lunar_frontiers.ttl".into(), "./src".into())?;

    // Generate the implementation wrappers
    concordance_gen::generate_impl(
        "../model/lunar_frontiers.ttl".into(),
        "./src".into(),
        "rover".into(),
        EntityType::Aggregate,
        "".to_string(),
    )?;
    Ok(())
}
