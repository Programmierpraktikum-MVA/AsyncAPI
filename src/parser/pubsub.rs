use super::schema_parser_mapper;
use crate::{
    asyncapi_model::{AsyncAPI, Operation, OperationMessageType, Payload, ReferenceOr, Schema},
    parser::common::convert_string_to_valid_type_name,
    template_model::PubsubTemplate,
};
use std::{collections::HashMap, io};

fn transform_schema_to_string_vec(schema: &Schema, root_struct_name: &str) -> Vec<String> {
    let mut structs = HashMap::new();
    schema_parser_mapper(&schema.clone(), root_struct_name, &mut structs);
    vec![structs
        .values()
        .map(|v| v.to_string())
        .collect::<Vec<String>>()
        .join("\n")]
}

fn extract_schemas_from_channels(
    pub_channels: Vec<(&String, &Operation)>,
    sub_channels: Vec<(&String, &Operation)>,
) -> Vec<String> {
    return pub_channels
        .iter()
        .chain(sub_channels.iter())
        .flat_map(|x| {
            let root_msg_name = convert_string_to_valid_type_name(x.0, "");
            let channel = x.1;
            let operation_message = channel.message.as_ref().unwrap();
            println!("\noperation_message: {:?}", operation_message);
            match operation_message {
                OperationMessageType::Map(map) => map
                    .values()
                    .flat_map(|message_ref_or_item| match message_ref_or_item {
                        ReferenceOr::Item(message) => match &message.payload {
                            Some(Payload::Schema(schema)) => {
                                println!("\nmap schema: {:?}", schema);
                                transform_schema_to_string_vec(schema, &root_msg_name)
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
                                transform_schema_to_string_vec(schema, &root_msg_name)
                            }
                            _ => vec![], // or handle Payload::Any here
                        },
                        ReferenceOr::Reference { reference: _ } => vec![], // or handle ReferenceOr::Reference here
                    }
                }
            }
        })
        .collect();
}

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

    let schemas: Vec<String> = extract_schemas_from_channels(pub_channels, sub_channels);
    let joined_schemas = schemas.join("\n");
    println!("\nJoined schemas: {:?}", joined_schemas);

    Ok(PubsubTemplate {
        server_url: server.url,
        channel_name: spec.channels.first().unwrap().0.clone(),
        schema: joined_schemas,
    })
}
