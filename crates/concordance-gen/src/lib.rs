mod model;
mod templates;
mod codegen;
mod docgen;


use std::{path::PathBuf, error::Error, fs::{File, create_dir_all}, io::Write};

use codegen::{TraitsContext, render_trait_list};
use docgen::{render_aggregate_list, render_event_list, render_command_list, render_pm_list, render_projector_list, render_notifier_list};
use model::{get_aggregates, get_events, get_commands, get_process_managers, get_projectors, get_notifiers};
use rdf::{graph::Graph, reader::{turtle_parser::TurtleParser, rdf_parser::RdfParser}};

pub use model::*;

/// Generates a single implementation for the given entity. The entity name and entity type must match the name and
/// entity type as declared in the Turtle RDF (ttl) model specification file. Note that your actor will likely see numerous
/// compilation errors until you create your own implementation of the system traits.
pub fn generate_impl(source: PathBuf, output: PathBuf, entity_name: String, entity_type: EntityType, esinterface: String) -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string(source)?;
    let mut reader = TurtleParser::from_string(input.to_string());
    let graph = reader.decode()?;

    codegen::generate_impl(&graph, esinterface, entity_type, entity_name, output)?;
    Ok(())
}

/// Generates the set of traits as described in the model specification Turtle RDF (ttl) file. These traits are for developers
/// to implement, while the code from `generate_impl` creates the wrappers that invoke developer code. Currently this function
/// generates traits for every applicable entity discovered in the model. In the future, we allow the generation of a single
/// trait. 
pub fn generate_system_traits(source: PathBuf, output: PathBuf) -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string(source)?;
    let mut reader = TurtleParser::from_string(input.to_string());
    let graph = reader.decode()?;

    let ctx = TraitsContext {
        aggregates: get_aggregates(&graph),
        events: get_events(&graph),
        commands: get_commands(&graph),
        pms: get_process_managers(&graph),
        projectors: get_projectors(&graph),
        notifiers: get_notifiers(&graph),
    };

    let mut file = File::create(output.join("system_traits.rs"))?;
    file.write_all(render_trait_list(ctx)?.as_bytes())?;
    Ok(())
}

/// Generate markdown documentation based on the model specification Turtle RDF (ttl) file. This function emits a number of 
/// markdown files, an index file for each entity type.
pub fn generate_doc(source: PathBuf, output: PathBuf) -> Result<(), Box<dyn Error>> {

    if !output.exists() {
        create_dir_all(output.clone())?;
    }
    if !output.is_dir() {
        return Err(format!("{:?} is not a directory.", output).into());
    }

    let input = std::fs::read_to_string(source)?;
    let mut reader = TurtleParser::from_string(input.to_string());
    let graph = reader.decode()?;
    let aggregates = get_aggregates(&graph);
    let events = get_events(&graph);
    let commands = get_commands(&graph);
    let pms = get_process_managers(&graph);
    let projectors = get_projectors(&graph);
    let notifiers = get_notifiers(&graph);

    let mut file = File::create(output.join("agg_index.md"))?;
    file.write_all(render_aggregate_list(aggregates)?.as_bytes())?;

    let mut file = File::create(output.join("evt_index.md"))?;
    file.write_all(render_event_list(events)?.as_bytes())?;

    let mut file = File::create(output.join("cmd_index.md"))?;
    file.write_all(render_command_list(commands)?.as_bytes())?;

    let mut file = File::create(output.join("pm_index.md"))?;
    file.write_all(render_pm_list(pms)?.as_bytes())?;

    let mut file = File::create(output.join("proj_index.md"))?;
    file.write_all(render_projector_list(projectors)?.as_bytes())?;

    let mut file = File::create(output.join("notifier_index.md"))?;
    file.write_all(render_notifier_list(notifiers)?.as_bytes())?;

    Ok(())
}                               