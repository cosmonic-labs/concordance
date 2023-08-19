use crate::generator::register_helpers;
use crate::model::eventcatalog::EventCatalogSite;
use crate::{
    model::{AggregateSummary, EntityType},
    templates::Asset,
};
use anyhow::Result;
use handlebars::Handlebars;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
struct AggregateContext {
    summary: AggregateSummary,
    traitname: String,
}

#[derive(Serialize, Debug, Clone)]
struct ImplContext {
    traitname: String,
    rootname: String,
    impltype: String,
    summary: AggregateSummary,
}

pub(crate) fn render(catalog: &EventCatalogSite, aggregate: &AggregateSummary) -> Result<String> {
    let mut handlebars = Handlebars::new();
    register_helpers(&mut handlebars);

    let template = Asset::get("agg_trait.hbs").unwrap();
    let template_str = std::str::from_utf8(template.data.as_ref())?;

    let impl_template = Asset::get("agg_impl.hbs").unwrap();
    let template_impl_str = std::str::from_utf8(impl_template.data.as_ref())?;

    let wrapper = AggregateContext {
        summary: aggregate.clone(),
        traitname: inflector::cases::classcase::to_class_case(&aggregate.name),
    };

    let impl_wrapper = ImplContext {
        traitname: wrapper.traitname.clone(),
        rootname: aggregate.name.clone(),
        impltype: EntityType::Aggregate.to_trait_name(),
        summary: aggregate.clone(),
    };

    let agg_trait = handlebars
        .render_template(template_str, &wrapper)
        .map_err(|e| anyhow::anyhow!("Template render failure: {}", e))?;

    let agg_impl = handlebars
        .render_template(template_impl_str, &impl_wrapper)
        .map_err(|e| anyhow::anyhow!("Template render failure: {}", e))?;

    let mut structs = Vec::new();
    let all = aggregate
        .inbound_commands
        .iter()
        .chain(aggregate.inbound_events.iter());

    for entity in all {
        if let Some(schema) = catalog.schemas.get(&entity.name) {
            let tsettings = typify::TypeSpaceSettings::default();
            let mut tspace = typify::TypeSpace::new(&tsettings);
            tspace.add_root_schema(serde_json::from_value(schema.clone())?)?;
            // tspace.add_type(serde_json::from_value(schema.clone())?);
            structs.push(tspace.to_stream().to_string());
        }
    }
    
    Ok(format!(
        "\n{}\n\n{}\n\n{}",
        structs.join("\n"),
        agg_trait,
        agg_impl
    ))
}
