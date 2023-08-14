use anyhow::Result;
use handlebars::Handlebars;
use rdf::{graph::Graph, node::Node};
use serde::Serialize;

use crate::{
    generator::{method_case, title_case, trait_case},
    model::{inbound_to_node, Entity, EntityType},
    templates::Asset,
};

#[derive(Serialize, Debug, Clone)]
struct ImplWrapperContext {
    traitname: String,
    rootname: String,
    impltype: String,
    inbound_commands: Vec<Entity>,
    inbound_events: Vec<Entity>,
}

pub(crate) fn render(g: &Graph, n: &Node) -> Result<String> {
    let inbound = inbound_to_node(g, n);
    let (inbound_commands, inbound_events): (Vec<_>, Vec<_>) = inbound
        .clone()
        .into_iter()
        .partition(|input| input.entity_type == EntityType::Command);
    let mut handlebars = Handlebars::new();
    handlebars.register_helper("title-case", Box::new(title_case));
    handlebars.register_helper("trait-name", Box::new(trait_case));
    handlebars.register_helper("method-name", Box::new(method_case));

    let template = Asset::get("gen_evt_handler.hbs").unwrap();
    let template_str = std::str::from_utf8(template.data.as_ref())?;

    let entity = Entity::new_from_node(g, n);

    let entity_name = entity.name.to_string();

    let wrapper = ImplWrapperContext {
        traitname: inflector::cases::classcase::to_class_case(&entity_name),
        rootname: entity_name.to_string(),
        impltype: entity.entity_type.to_trait_name(),
        inbound_commands: inbound_commands,
        inbound_events: inbound_events,
    };

    handlebars
        .render_template(template_str, &wrapper)
        .map_err(|e| anyhow::anyhow!("Template render failure: {}", e))
}
