use crate::{
    asyncapi_model::{AsyncAPI, ReferenceOr},
    template_model::{
        Model, MultiStructEnum, SimplifiedMessage, SimplifiedOperation, TemplateContext,
    },
};
use std::{collections::HashMap, io};

fn extract_model_from_channels(
    pub_channels: Vec<(&String, SimplifiedOperation)>,
    sub_channels: Vec<(&String, SimplifiedOperation)>,
) -> Model {
    let mut messages: HashMap<String, SimplifiedMessage> = HashMap::new();
    let mut enums: HashMap<String, MultiStructEnum> = HashMap::new();

    // Chain the pub_channels and sub_channels vectors into a single iterator
    for (_, operation) in pub_channels.into_iter().chain(sub_channels.into_iter()) {
        for message in operation.messages {
            messages.insert(message.unique_id.clone(), message);
        }
        if let Some(multiple_messages_enum) = operation.multiple_messages_enum.clone() {
            enums.insert(
                multiple_messages_enum.unique_id.clone(),
                multiple_messages_enum,
            );
        }
    }

    Model {
        messages: messages.into_values().collect(),
        enums: enums.into_values().collect(),
    }
}

pub fn spec_to_pubsub_template_type<'a>(
    spec: &'a AsyncAPI,
) -> Result<TemplateContext<'a>, io::Error> {
    let item = spec
        .servers
        .first()
        .expect("Server field is required in the specification!")
        .1;
    let server = match item {
        ReferenceOr::Item(it) => it,
        ReferenceOr::Reference { reference: _ } => None.unwrap(),
    };

    let publish_channels = spec.get_publish_channels_operations();
    let subscribe_channels = spec.get_subscribe_channels_operations();
    let models: Model =
        extract_model_from_channels(publish_channels.clone(), subscribe_channels.clone());

    // println!("models: {:?}", models);
    let pubsub_template: TemplateContext<'a> = TemplateContext {
        server,
        subscribe_channels,
        publish_channels,
        title: &spec.info.title,
        description: &spec.info.description,
        model: models,
    };
    Ok(pubsub_template)
}
