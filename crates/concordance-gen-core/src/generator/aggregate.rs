use crate::{
    generator::{method_case, title_case, trait_case},
    model::{AggregateSummary, Entity, EntityType},
    templates::Asset,
};
use anyhow::Result;
use handlebars::{Context, Handlebars, Helper, HelperResult, Output, RenderContext};
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

pub(crate) fn render(aggregate: &AggregateSummary) -> Result<String> {
    let mut handlebars = Handlebars::new();
    handlebars.register_helper("title-case", Box::new(title_case));
    handlebars.register_helper("trait-name", Box::new(trait_case));
    handlebars.register_helper("method-name", Box::new(method_case));

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

    Ok(format!("\n{}\n\n{}", agg_trait, agg_impl))
}
