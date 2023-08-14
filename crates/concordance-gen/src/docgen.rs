use handlebars::{Context, Handlebars, Helper, HelperResult, Output, RenderContext};
use rdf::reader::rdf_parser::RdfParser;
use rdf::reader::turtle_parser::TurtleParser;
use std::error::Error;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::PathBuf;

use crate::model::{
    get_aggregates, get_commands, get_events, get_notifiers, get_process_managers, get_projectors,
    AggregateIndex, AggregateSummary, CommandIndex, CommandSummary, EventIndex, EventSummary,
    NotifierSummary, Notifierindex, ProcessManagerIndex, ProcessManagerSummary, ProjectorIndex,
    ProjectorSummary,
};
use crate::templates::Asset;

pub(crate) fn render_notifier_list(
    notifiers: Vec<NotifierSummary>,
) -> Result<String, Box<dyn Error>> {
    let mut handlebars = Handlebars::new();
    handlebars.register_helper("title-case", Box::new(title_case));

    let template = Asset::get("notifier_index.hbs").unwrap();
    let template_str = std::str::from_utf8(template.data.as_ref())?;

    let index = Notifierindex { notifiers };

    handlebars
        .render_template(template_str, &index)
        .map_err(|e| format!("{}", e).into())
}

pub(crate) fn render_pm_list(pms: Vec<ProcessManagerSummary>) -> Result<String, Box<dyn Error>> {
    let mut handlebars = Handlebars::new();
    handlebars.register_helper("title-case", Box::new(title_case));

    let template = Asset::get("pm_index.hbs").unwrap();
    let template_str = std::str::from_utf8(template.data.as_ref())?;

    let index = ProcessManagerIndex {
        process_managers: pms,
    };

    handlebars
        .render_template(template_str, &index)
        .map_err(|e| format!("{}", e).into())
}

pub(crate) fn render_projector_list(
    projectors: Vec<ProjectorSummary>,
) -> Result<String, Box<dyn Error>> {
    let mut handlebars = Handlebars::new();
    handlebars.register_helper("title-case", Box::new(title_case));

    let template = Asset::get("proj_index.hbs").unwrap();
    let template_str = std::str::from_utf8(template.data.as_ref())?;

    let index = ProjectorIndex { projectors };

    handlebars
        .render_template(template_str, &index)
        .map_err(|e| format!("{}", e).into())
}

pub(crate) fn render_aggregate_list(
    aggregates: Vec<AggregateSummary>,
) -> Result<String, Box<dyn Error>> {
    let mut handlebars = Handlebars::new();
    handlebars.register_helper("title-case", Box::new(title_case));

    let template = Asset::get("agg_index.hbs").unwrap();
    let template_str = std::str::from_utf8(template.data.as_ref())?;

    let index = AggregateIndex { aggregates };

    handlebars
        .render_template(template_str, &index)
        .map_err(|e| format!("{}", e).into())
}

pub(crate) fn render_command_list(commands: Vec<CommandSummary>) -> Result<String, Box<dyn Error>> {
    let mut handlebars = Handlebars::new();
    handlebars.register_helper("title-case", Box::new(title_case));

    let template = Asset::get("cmd_index.hbs").unwrap();
    let template_str = std::str::from_utf8(template.data.as_ref())?;

    let index = CommandIndex { commands };

    handlebars
        .render_template(template_str, &index)
        .map_err(|e| format!("{}", e).into())
}

pub(crate) fn render_event_list(events: Vec<EventSummary>) -> Result<String, Box<dyn Error>> {
    let mut handlebars = Handlebars::new();
    handlebars.register_helper("title-case", Box::new(title_case));

    let template = Asset::get("evt_index.hbs").unwrap();
    let template_str = std::str::from_utf8(template.data.as_ref()).unwrap();

    let index = EventIndex { events };

    handlebars
        .render_template(template_str, &index)
        .map_err(|e| format!("{}", e).into())
}

pub(crate) fn title_case(
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
