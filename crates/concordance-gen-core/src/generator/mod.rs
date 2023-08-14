use anyhow::anyhow;
use anyhow::Result;
use handlebars::{Context, Handlebars, Helper, HelperResult, Output, RenderContext};

use crate::model;
use crate::model::AggregateSummary;
use crate::model::EntityType;
use crate::model::ProcessManagerSummary;
use crate::Model;

mod aggregate;
mod genhandler;
mod procmgr;

pub(crate) fn generate_aggregate(model: &Model, entity_name: &str) -> Result<String> {
    let Some(summary_node)= model::find_node(&model.graph, entity_name, &EntityType::Aggregate.prefix()) else {
        return Err(anyhow!("Could not find aggregate for entity {}", entity_name));
    };
    let aggregate_summary = AggregateSummary::new_from_node(&model.graph, &summary_node);

    aggregate::render(&aggregate_summary)
}

pub(crate) fn generate_general_event_handler(
    model: &Model,
    entity_name: &str,
    entity_type: &EntityType,
) -> Result<String> {
    let Some(summary_node)= model::find_node(&model.graph, entity_name, &entity_type.prefix()) else {
        return Err(anyhow!("Could not find graph node for entity {} of type {:?}", entity_name, entity_type));
    };
    genhandler::render(&model.graph, &summary_node)
}

pub(crate) fn generate_process_manager(model: &Model, entity_name: &str) -> Result<String> {
    let Some(summary_node)= model::find_node(&model.graph, entity_name, &EntityType::ProcessManager.prefix()) else {
        return Err(anyhow!("Could not find process manager for entity {}", entity_name));
    };
    let procmgr_summary = ProcessManagerSummary::new_from_node(&model.graph, &summary_node);
    procmgr::render(&procmgr_summary)
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

fn title_case(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    rc: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let param = h.param(0).unwrap();

    out.write(&inflector::cases::titlecase::to_title_case(
        param.value().as_str().unwrap(),
    ))?;

    Ok(())
}
