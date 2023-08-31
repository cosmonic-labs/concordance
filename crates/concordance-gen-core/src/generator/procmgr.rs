use super::register_helpers;
use crate::{
    model::{trim_summary_name, EntityType, EventCatalogSite, ProcessManagerSummary},
    templates::Asset,
};
use anyhow::Result;
use handlebars::Handlebars;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
struct ProcessManagerContext {
    pm: ProcessManagerSummary,
    traitname: String,
    impltype: String,
}

pub(crate) fn render(
    catalog: &EventCatalogSite,
    procmgr: &ProcessManagerSummary,
) -> Result<String> {
    let mut handlebars = Handlebars::new();
    register_helpers(&mut handlebars);

    let template = Asset::get("proc_mgr.hbs").unwrap();
    let template_str = std::str::from_utf8(template.data.as_ref())?;

    let trim_name = trim_summary_name(&procmgr.name, &EntityType::ProcessManager);

    let wrapper = ProcessManagerContext {
        pm: procmgr.clone(),
        traitname: inflector::cases::classcase::to_class_case(&trim_name),
        impltype: EntityType::ProcessManager.to_trait_name(),
    };

    let procman = handlebars
        .render_template(template_str, &wrapper)
        .map_err(|e| anyhow::anyhow!("Template render failure: {}", e))?;

    let mut structs = Vec::new();
    let all = procmgr.inbound.iter().chain(procmgr.outbound.iter());

    for entity in all {
        if let Some(schema) = catalog.schemas.get(&entity.name) {
            let tsettings = typify::TypeSpaceSettings::default();
            let mut tspace = typify::TypeSpace::new(&tsettings);
            tspace.add_root_schema(serde_json::from_value(schema.clone())?)?;
            // tspace.add_type(serde_json::from_value(schema.clone())?);
            structs.push(tspace.to_stream().to_string());
        }
    }

    Ok(format!("\n{}\n\n{}", structs.join("\n\n"), procman))
}
