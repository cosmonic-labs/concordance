use handlebars::{Context, Handlebars, Helper, HelperResult, Output, RenderContext};

pub(crate) mod aggregate;
pub(crate) mod genhandler;
pub(crate) mod procmgr;

// Helper functions added to the Handlebars context for use in templates
pub(crate) fn register_helpers(handlebars: &mut Handlebars) {
    handlebars.register_helper("title-case", Box::new(title_case));
    handlebars.register_helper("trait-name", Box::new(trait_case));
    handlebars.register_helper("method-name", Box::new(method_case));
}

fn method_case(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _rc: &mut RenderContext,
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
    _rc: &mut RenderContext,
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
    _rc: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let param = h.param(0).unwrap();

    out.write(&inflector::cases::titlecase::to_title_case(
        param.value().as_str().unwrap(),
    ))?;

    Ok(())
}
