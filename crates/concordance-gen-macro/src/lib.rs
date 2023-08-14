use concordance_gen_core::model::EntityType;
use concordance_gen_core::Model;
use proc_macro2::{Span, TokenStream};
use std::path::{Path, PathBuf};
use syn::parse::{Error, Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{braced, token, Token};

#[proc_macro]
pub fn generate(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    syn::parse_macro_input!(input as Config)
        .expand()
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

struct Config {
    model: Model,
    role: GeneratorRole,
    entity: String,
}

enum Source {
    Path(String),
    Inline(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum GeneratorRole {
    Aggregate,
    Projector,
    ProcessManager,
    Notifier,
}

impl From<String> for GeneratorRole {
    fn from(s: String) -> Self {
        match s.as_str().to_lowercase().trim() {
            "aggregate" => GeneratorRole::Aggregate,
            "projector" => GeneratorRole::Projector,
            "process_manager" => GeneratorRole::ProcessManager,
            "notifier" => GeneratorRole::Notifier,
            _ => panic!("Invalid generator role: {}", s),
        }
    }
}

impl Parse for Config {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let call_site = Span::call_site();

        let mut source = None;
        let mut role = None;
        let mut entity = None;

        if input.peek(token::Brace) {
            let content;
            syn::braced!(content in input);
            let fields = Punctuated::<Opt, Token![,]>::parse_terminated(&content)?;
            for field in fields.into_pairs() {
                match field.into_value() {
                    Opt::Path(s) => {
                        if source.is_some() {
                            return Err(Error::new(s.span(), "cannot specify multiple sources"));
                        }
                        source = Some(Source::Path(s.value()));
                    }
                    Opt::Entity(s) => {
                        if entity.is_some() {
                            return Err(Error::new(s.span(), "cannot specify multiple entities"));
                        }
                        entity = Some(s.value().into());
                    }
                    Opt::Role(s) => {
                        if role.is_some() {
                            return Err(Error::new(s.span(), "cannot specify multiple roles"));
                        }
                        role = Some(s.value().into());
                    }
                }
            }
        } else {
            source = Some(Source::Path(input.parse::<syn::LitStr>()?.value()));
            role = Some(GeneratorRole::Aggregate);
            //if input.parse::<Option<syn::token::In>>()?.is_some() {
            //source = Some(Source::Path(input.parse::<syn::LitStr>()?.value()));
            //}
        }

        let source = source.ok_or_else(|| {
            Error::new(
                call_site,
                "Unable to locate a source for the model definition",
            )
        })?;
        let role = role
            .ok_or_else(|| Error::new(call_site, "Unable to determine a role for the generator"))?;
        let entity =
            entity.ok_or_else(|| Error::new(call_site, "No entity specified for generator"))?;
        let raw = parse_source(&source)?;
        let model = Model::from_raw(&raw).unwrap();

        Ok(Config {
            model,
            role,
            entity,
        })
    }
}

impl Config {
    fn expand(self) -> Result<TokenStream> {
        let src = match self.role {
            GeneratorRole::Aggregate => self
                .model
                .generate_aggregate(&self.entity)
                .map_err(|e| syn::Error::new(Span::call_site(), e)),
            GeneratorRole::Projector => self
                .model
                .generate_general_event_handler(&self.entity, &EntityType::Projector)
                .map_err(|e| syn::Error::new(Span::call_site(), e)),
            GeneratorRole::Notifier => self
                .model
                .generate_general_event_handler(&self.entity, &EntityType::Notifier)
                .map_err(|e| syn::Error::new(Span::call_site(), e)),
            GeneratorRole::ProcessManager => self
                .model
                .generate_process_manager(&self.entity)
                .map_err(|e| syn::Error::new(Span::call_site(), e)),
            _ => Err(syn::Error::new(Span::call_site(), "Not implemented")),
        }?;

        let contents = src.parse::<TokenStream>().unwrap();

        Ok(contents)
    }
}

fn parse_source(source: &Source) -> Result<String> {
    match source {
        Source::Path(path) => {
            let path = Path::new(path);
            if !path.is_file() {
                return Err(Error::new(Span::call_site(), "file not found"));
            }
            std::fs::read_to_string(path).map_err(|e| Error::new(Span::call_site(), e))
        }
        Source::Inline(s) => Ok(s.clone()),
    }
}

mod kw {
    syn::custom_keyword!(path);
    syn::custom_keyword!(role);
    syn::custom_keyword!(entity);
}

enum Opt {
    Path(syn::LitStr),
    Role(syn::LitStr),
    Entity(syn::LitStr),
}

impl Parse for Opt {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let l = input.lookahead1();
        if l.peek(kw::path) {
            input.parse::<kw::path>()?;
            input.parse::<Token![:]>()?;
            Ok(Opt::Path(input.parse()?))
        } else if l.peek(kw::role) {
            input.parse::<kw::role>()?;
            input.parse::<Token![:]>()?;
            Ok(Opt::Role(input.parse()?))
        } else if l.peek(kw::entity) {
            input.parse::<kw::entity>()?;
            input.parse::<Token![:]>()?;
            Ok(Opt::Entity(input.parse()?))
        } else {
            Err(l.error())
        }
    }
}
