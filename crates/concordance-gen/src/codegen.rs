use handlebars::{Context, Handlebars, Helper, HelperResult, Output, RenderContext};
use rdf::graph::Graph;
use rdf::reader::rdf_parser::RdfParser;
use rdf::reader::turtle_parser::TurtleParser;
use serde::Serialize;
use std::error::Error;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;
use std::string::ParseError;

use crate::docgen::title_case;
use crate::model::{
    get_aggregates, get_commands, get_events, get_notifiers, get_process_managers, get_projectors,
    AggregateIndex, AggregateSummary, CommandIndex, CommandSummary, Entity, EntityType, EventIndex,
    EventSummary, NotifierSummary, Notifierindex, ProcessManagerIndex, ProcessManagerSummary,
    ProjectorIndex, ProjectorSummary, find_entity, find_node, inbound_to_node,
};
use crate::templates::Asset;

#[derive(Serialize, Debug, Clone)]
pub(crate) struct TraitsContext {
    pub aggregates: Vec<AggregateSummary>,
    pub events: Vec<EventSummary>,
    pub commands: Vec<CommandSummary>,
    pub pms: Vec<ProcessManagerSummary>,
    pub projectors: Vec<ProjectorSummary>,
    pub notifiers: Vec<NotifierSummary>,
}

#[derive(Serialize, Debug, Clone)]
pub(crate) struct ImplWrapperContext {
    traitname: String,
    rootname: String,
    esinterface: String,
    impltype: String,
    inbound_commands: Vec<Entity>,
    inbound_events: Vec<Entity>,
}

impl FromStr for EntityType {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "aggregate" => Ok(EntityType::Aggregate),
            "projector" => Ok(EntityType::Projector),
            "pm" => Ok(EntityType::ProcessManager),
            "notifier" => Ok(EntityType::Notifier),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Unexpected generator type. Valid types: aggregate, projector, pm, notifier",
            )),
        }
    }
}


pub(crate) fn generate_impl(g: &Graph, esinterface: String, entity_type: EntityType, entity_name: String, output: PathBuf) -> Result<(), Box<dyn Error>> {
    let node = find_node(g, &entity_name, &entity_type.prefix()).unwrap();
    
    let inbound = inbound_to_node(g, &node);
    let (inbound_commands, inbound_events): (Vec<_>, Vec<_>) = inbound
        .clone()
        .into_iter()
        .partition(|input| input.entity_type == EntityType::Command);
    let wrapper = ImplWrapperContext {
        traitname: inflector::cases::classcase::to_class_case(
            &entity_name.to_string(),
        ),
        rootname: entity_name.to_string(),
        esinterface: esinterface.to_string(),
        impltype: entity_type.to_trait_name(),
        inbound_commands: inbound_commands.clone(),
        inbound_events: inbound_events.clone(),
    };
    let mut file = File::create(output.join("genimpl.rs"))?;
    file.write_all(render_impl_wrapper(wrapper, &entity_type)?.as_bytes())?;

    Ok(())
}


fn render_impl_wrapper(wrapper: ImplWrapperContext, entity_type: &EntityType) -> Result<String, Box<dyn Error>> {
    let mut handlebars = Handlebars::new();
    handlebars.register_helper("title-case", Box::new(title_case));
    handlebars.register_helper("trait-name", Box::new(trait_case));
    handlebars.register_helper("method-name", Box::new(method_case));

    let template = match entity_type {
        EntityType::Aggregate => Asset::get("agg_impl.hbs").unwrap(),
        EntityType::Notifier | EntityType::Projector => Asset::get("evt_handler_impl.hbs").unwrap(),
        EntityType::ProcessManager => Asset::get("pm_impl.hbs").unwrap(),
        _ => panic!("Unexpected entity type for generation"),
    };

    //let template = Asset::get("gen_impl.hbs").unwrap();
    let template_str = std::str::from_utf8(template.data.as_ref())?;

    handlebars
        .render_template(template_str, &wrapper)
        .map_err(|e| format!("{}", e).into())
}

pub(crate) fn render_trait_list(ctx: TraitsContext) -> Result<String, Box<dyn Error>> {
    let mut handlebars = Handlebars::new();
    handlebars.register_helper("title-case", Box::new(title_case));
    handlebars.register_helper("trait-name", Box::new(trait_case));
    handlebars.register_helper("method-name", Box::new(method_case));

    let template = Asset::get("system_traits.hbs").unwrap();
    let template_str = std::str::from_utf8(template.data.as_ref())?;

    handlebars
        .render_template(template_str, &ctx)
        .map_err(|e| format!("{}", e).into())
}

fn method_case(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    rc: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let param = h.param(0).unwrap();

    out.write(&inflector::cases::snakecase::to_snake_case(
        param.value().as_str().unwrap(),
    ))?;

    Ok(())
}

fn trait_case(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    rc: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let param = h.param(0).unwrap();

    out.write(&inflector::cases::classcase::to_class_case(
        param.value().as_str().unwrap(),
    ))?;

    Ok(())
}
