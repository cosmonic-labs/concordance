use concordance_gen::EntityType;

const CONFIG: &str = "./codegen.toml";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    weld_codegen::rust_build(CONFIG)?;    

    // Generate the relevant traits
    concordance_gen::generate_system_traits("../bankaccount-model.ttl".into(), "./src".into())?;

    // Generate the implementation wrappers
    concordance_gen::generate_impl(
        "../bankaccount-model.ttl".into(),
        "./src".into(),
        "bankaccount".into(),
        EntityType::Projector,
        "".to_string(),
    )?;
    Ok(())
}