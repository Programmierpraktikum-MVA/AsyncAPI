use super::{schema_parser::SchemaParserError, schema_parser_mapper};
use crate::{
    asyncapi_model::{AsyncAPI, OperationMessageType, Payload, ReferenceOr, Schema},
    parser::common::convert_string_to_valid_type_name,
    template_model::PubsubTemplate,
};
use std::{collections::HashMap, io};

fn transform_schema_to_string_vec(
    schema: &Schema,
    root_struct_name: &str,
) -> Result<Vec<String>, SchemaParserError> {
    let mut structs = HashMap::new();
    schema_parser_mapper(&schema.clone(), root_struct_name, &mut structs)?;
    Ok(vec![structs
        .values()
        .map(|v| v.to_string())
        .collect::<Vec<String>>()
        .join("\n")])
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

fn parse_single_message_operation_type(
    message_ref_or_item: &ReferenceOr<crate::asyncapi_model::Message>,
    root_msg_name: String,
    spec: &AsyncAPI,
) -> Vec<String> {
    match message_ref_or_item {
        ReferenceOr::Item(message) => match &message.payload {
            Some(Payload::Schema(schema)) => {
                println!("\nmap schema: {:?}", schema);
                transform_schema_to_string_vec(schema, &root_msg_name).unwrap()
            }
            Some(Payload::Any(val)) => {
                panic!("\nWARNING: Unsupported payload (Any): {:?}", val);
            }
            None => {
                panic!("\nWARNING: No payload found for message: {:?}", message)
            }
        },
        ReferenceOr::Reference { reference } => {
            let reference_path = parse_message_reference_path(&reference);
            if reference_path.is_none() {
                panic!("\nWARNING: Invaid reference: {:?}", reference);
            }
            let schema = spec.get_schema_from_reference(reference_path.unwrap().as_str());
            transform_schema_to_string_vec(schema, &root_msg_name).unwrap()
        }
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
            match operation_message {
                OperationMessageType::Map(map) => map
                    .values()
                    .flat_map(
                        |message_ref_or_item: &ReferenceOr<crate::asyncapi_model::Message>| {
                            parse_single_message_operation_type(
                                message_ref_or_item,
                                root_msg_name.clone(),
                                spec,
                            )
                        },
                    )
                    .collect(),
                OperationMessageType::Single(message_ref_or_item) => {
                    parse_single_message_operation_type(
                        message_ref_or_item,
                        root_msg_name.clone(),
                        spec,
                    )
                }
            }
        })
        .collect();
}

pub fn spec_to_pubsub_template_type<'a>(
    spec: &'a AsyncAPI,
) -> Result<PubsubTemplate<'a>, io::Error> {
    let item = spec.servers.first().unwrap().1;
    let server = match item {
        ReferenceOr::Item(it) => it,
        ReferenceOr::Reference { reference: _ } => None.unwrap(),
    };

    let schemas: Vec<String> = extract_schemas_from_asyncapi(spec);
    let joined_schemas = schemas.join("\n");
    println!("\nJoined schemas: {:?}", joined_schemas);

    let publish_channels = spec.get_publish_channels_operations();
    let subscribe_channels = spec.get_subscribe_channels_operations();

    let pubsub_template: PubsubTemplate<'a> = PubsubTemplate {
        server,
        subscribe_channels,
        publish_channels,
    };
    Ok(pubsub_template)
}
