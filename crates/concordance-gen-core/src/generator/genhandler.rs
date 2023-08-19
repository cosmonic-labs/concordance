use anyhow::Result;
use handlebars::Handlebars;
use serde::Serialize;

use crate::{
    generator::register_helpers,
    model::{eventcatalog::EventCatalogSite, GenHandlerSummary},
    templates::Asset,
};

#[derive(Serialize, Debug, Clone)]
pub(crate) struct GenHandlerContext {
    summary: GenHandlerSummary,
    traitname: String,
    rootname: String,
    impltype: String,
}

pub(crate) fn render(catalog: &EventCatalogSite, genhandler: &GenHandlerSummary) -> Result<String> {    

    let mut handlebars = Handlebars::new();
    register_helpers(&mut handlebars);
    let impl_template = Asset::get("gen_evt_handler.hbs").unwrap();
    let template_impl_str = std::str::from_utf8(impl_template.data.as_ref())?;

    let wrapper = GenHandlerContext {
        summary: genhandler.clone(),
        traitname: inflector::cases::classcase::to_class_case(&genhandler.name),
        rootname: genhandler.name.clone(),
        impltype: genhandler.entity_type.to_trait_name(),
    };

    let gen_impl = handlebars
        .render_template(template_impl_str, &wrapper)
        .map_err(|e| anyhow::anyhow!("Template render failure: {}", e))?;

    let mut structs = Vec::new();

    for entity in genhandler.inbound.iter() {
        if let Some(schema) = catalog.schemas.get(&entity.name) {
            let tsettings = typify::TypeSpaceSettings::default();
            let mut tspace = typify::TypeSpace::new(&tsettings);
            tspace.add_root_schema(serde_json::from_value(schema.clone())?)?;

            structs.push(tspace.to_stream().to_string());
        }
    }

    Ok(format!("\n{}\n\n{}", structs.join("\n"), gen_impl))
}
