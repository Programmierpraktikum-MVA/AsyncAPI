use super::schema_parser_mapper;
use crate::{
    asyncapi_model::{AsyncAPI, OperationMessageType, Payload, ReferenceOr, Schema},
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

fn parse_message_reference_path(reference: &str) -> Option<String> {
    if reference.starts_with("#/components/messages") {
        let path: Vec<&str> = reference.split('/').collect();
        // skip first two elements ("#" and "components") and return the rest
        Some(path[3].to_string())
    } else {
        None
    }
}
fn extract_schemas_from_asyncapi(spec: &AsyncAPI) -> Vec<String> {
    let channels_ops = spec.get_all_channels_operations();
    return channels_ops
        .iter()
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
                                println!("\nWARNING: Unsupported payload (Any): {:?}", val);
                                vec![]
                            }
                            None => {
                                println!("\nWARNING: No payload found!");
                                vec![]
                            }
                        },
                        ReferenceOr::Reference { reference } => {
                            println!("\nReferenceOr::Reference: {:?}", reference);
                            let schema = spec.get_schema_from_reference(
                                parse_message_reference_path(reference).unwrap().as_str(),
                            );
                            transform_schema_to_string_vec(schema, &root_msg_name)
                        }
                    })
                    .collect(),
                OperationMessageType::Single(message_ref_or_item) => match message_ref_or_item {
                    ReferenceOr::Item(message) => match &message.payload {
                        Some(Payload::Schema(schema)) => {
                            println!("\nsingle schema: {:?}", schema);
                            transform_schema_to_string_vec(schema, &root_msg_name)
                        }
                        _val => {
                            println!("\nWARNING: Unsupported payload (Any): {:?}", _val);
                            vec![]
                        }
                    },
                    ReferenceOr::Reference { reference } => {
                        println!("\nReferenceOr::Reference: {:?}", reference);
                        let schema = spec.get_schema_from_reference(
                            parse_message_reference_path(reference).unwrap().as_str(),
                        );
                        transform_schema_to_string_vec(schema, &root_msg_name)
                    }
                },
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

    let schemas: Vec<String> = extract_schemas_from_asyncapi(&spec);
    let joined_schemas = schemas.join("\n");
    println!("\nJoined schemas: {:?}", joined_schemas);

    Ok(PubsubTemplate {
        server_url: server.url,
        channel_name: spec.channels.first().unwrap().0.clone(),
        schema: joined_schemas,
    })
}
