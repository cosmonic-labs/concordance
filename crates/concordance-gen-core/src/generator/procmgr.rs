use crate::{
    generator::{method_case, title_case, trait_case},
    model::{Entity, EntityType, ProcessManagerSummary},
    templates::Asset,
};
use anyhow::Result;
use handlebars::{Context, Handlebars, Helper, HelperResult, Output, RenderContext};
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
struct ProcessManagerContext {
    pm: ProcessManagerSummary,
    traitname: String,
    impltype: String,
}

pub(crate) fn render(procmgr: &ProcessManagerSummary) -> Result<String> {
    let mut handlebars = Handlebars::new();
    handlebars.register_helper("title-case", Box::new(title_case));
    handlebars.register_helper("trait-name", Box::new(trait_case));
    handlebars.register_helper("method-name", Box::new(method_case));

    let template = Asset::get("proc_mgr.hbs").unwrap();
    let template_str = std::str::from_utf8(template.data.as_ref())?;

    let wrapper = ProcessManagerContext {
        pm: procmgr.clone(),
        traitname: inflector::cases::classcase::to_class_case(&procmgr.name),
        impltype: EntityType::ProcessManager.to_trait_name(),
    };

    handlebars
        .render_template(template_str, &wrapper)
        .map_err(|e| anyhow::anyhow!("Template render failure: {}", e))
}
