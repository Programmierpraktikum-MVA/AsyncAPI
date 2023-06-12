use crate::{
    asyncapi_model::{AsyncAPI, ReferenceOr},
    template_model::TemplateContext,
};
use std::io;

pub fn spec_to_pubsub_template_type<'a>(
    spec: &'a AsyncAPI,
) -> Result<TemplateContext<'a>, io::Error> {
    let item = spec.servers.first().unwrap().1;
    let server = match item {
        ReferenceOr::Item(it) => it,
        ReferenceOr::Reference { reference: _ } => None.unwrap(),
    };

    let publish_channels = spec.get_publish_channels_operations();
    let subscribe_channels = spec.get_subscribe_channels_operations();

    let pubsub_template: TemplateContext<'a> = TemplateContext {
        server,
        subscribe_channels,
        publish_channels,
        title: &spec.info.title,
        description: &spec.info.description,
    };
    Ok(pubsub_template)
}
