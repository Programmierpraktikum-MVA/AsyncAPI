use crate::{
    asyncapi_model::{AsyncAPI, OperationMessageType, Payload, ReferenceOr},
    template_model::PubsubTemplate,
};
use std::{collections::HashMap, io};

use super::schema_parser_mapper;

pub fn spec_to_pubsub_template_type(spec: AsyncAPI) -> Result<PubsubTemplate, io::Error> {
    let item = spec.servers.first().unwrap().1;
    let server = match item {
        ReferenceOr::Item(it) => Some(it),
        ReferenceOr::Reference { reference: _ } => None,
    }
    .unwrap()
    .clone();

    let pub_channels = spec.get_publish_channels();
    let sub_channels = spec.get_subscribe_channels();

    let schemas: Vec<String> = pub_channels
        .iter()
        .chain(sub_channels.iter())
        .flat_map(|x| {
            let channel = x.1;
            let operation_message = channel.message.as_ref().unwrap();
            println!("\noperation_message: {:?}", operation_message);
            let test_name = String::from("test_schema");
            match operation_message {
                OperationMessageType::Map(map) => map
                    .values()
                    .flat_map(|message_ref_or_item| match message_ref_or_item {
                        ReferenceOr::Item(message) => match &message.payload {
                            Some(Payload::Schema(schema)) => {
                                println!("\nschema: {:?}", schema);
                                let mut structs = HashMap::new();
                                schema_parser_mapper(
                                    &Box::new(schema.clone()),
                                    &test_name,
                                    &mut structs,
                                );
                                vec![structs
                                    .iter()
                                    .map(|(_, v)| v.to_string())
                                    .collect::<Vec<String>>()
                                    .join("\n")]
                            }
                            Some(Payload::Any(val)) => {
                                println!("\nPayload::Any: {:?}", val);
                                vec![]
                            }
                            None => {
                                println!("\nPayload::None");
                                vec![]
                            }
                        },
                        ReferenceOr::Reference { reference: _ } => {
                            println!("\nReferenceOr::Reference");
                            vec![]
                        }
                    })
                    .collect(),
                OperationMessageType::Single(message_ref_or_item) => {
                    match message_ref_or_item {
                        ReferenceOr::Item(message) => match &message.payload {
                            Some(Payload::Schema(schema)) => {
                                println!("\nsingle schema: {:?}", schema);
                                let mut structs = HashMap::new();
                                schema_parser_mapper(
                                    &Box::new(schema.clone()),
                                    &test_name,
                                    &mut structs,
                                );
                                vec![structs
                                    .iter()
                                    .map(|(_, v)| v.to_string())
                                    .collect::<Vec<String>>()
                                    .join("\n")]
                            }
                            _ => vec![], // or handle Payload::Any here
                        },
                        ReferenceOr::Reference { reference: _ } => vec![], // or handle ReferenceOr::Reference here
                    }
                }
            }
        })
        .collect();

    let joined_schemas = schemas.join("\n");

    println!("\nJoined schemas: {:?}", joined_schemas);

    Ok(PubsubTemplate {
        server_url: server.url,
        channel_name: spec.channels.first().unwrap().0.clone(),
        schema: joined_schemas,
    })
}
