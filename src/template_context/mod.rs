mod channel_operations;
mod model;
mod types;
mod utilities;
use crate::asyncapi_model::{AsyncAPI, ReferenceOr};
use std::io;
pub use types::{Model, MultiStructEnum, SimplifiedMessage, SimplifiedOperation, TemplateContext};

pub fn create_template_context<'a>(spec: &'a AsyncAPI) -> Result<TemplateContext<'a>, io::Error> {
    let item = spec
        .servers
        .first()
        .expect("Server field is required in the specification!")
        .1;
    let server = match item {
        ReferenceOr::Item(it) => it,
        ReferenceOr::Reference { reference: _ } => None.unwrap(),
    };

    let publish_channels = channel_operations::get_publish_channels_operations(spec);
    let subscribe_channels = channel_operations::get_subscribe_channels_operations(spec);
    let model: Model =
        model::extract_model_from_channels(publish_channels.clone(), subscribe_channels.clone());
    let template_context: TemplateContext<'a> = TemplateContext {
        server,
        subscribe_channels,
        publish_channels,
        title: &spec.info.title,
        description: &spec.info.description,
        model,
    };
    Ok(template_context)
}
