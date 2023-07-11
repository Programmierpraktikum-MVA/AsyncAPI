use crate::{
    asyncapi_model::{Message, Operation, OperationMessageType, Payload, ReferenceOr, Schema},
    parser::{
        common::validate_identifier_string,
        json_schema_parser::{parse_json_schema_to_rust_type, types::RustSchemaRepresentation},
    },
};

use super::types::{SimplifiedMessage, SimplifiedOperation};

pub fn simplify_operation(operation: &Operation, channel_name: &str) -> SimplifiedOperation {
    let unique_id = operation
        .operation_id
        .clone()
        .unwrap_or_else(|| validate_identifier_string(channel_name, false));

    let messages: Vec<SimplifiedMessage> = match &operation.message {
        Some(operation_message) => match operation_message {
            OperationMessageType::Map(map) => map
                .into_iter()
                .map(|(_, m)| simplify_message(m, &unique_id))
                .collect(),
            OperationMessageType::Single(message_or_ref) => {
                vec![simplify_message(message_or_ref, &unique_id)]
            }
            OperationMessageType::OneOf(multiple_messages) => multiple_messages
                .one_of
                .iter()
                .map(|m| simplify_message(m, &unique_id))
                .collect(),
        },
        _ => vec![],
    };
    // let message_enum =
    //     build_multi_message_enum(&messages, format!("{}Message", unique_id).as_str());
    SimplifiedOperation {
        unique_id,
        original_operation: operation.clone(),
        messages,
        // multiple_messages_enum: message_enum,
    }
}

pub fn simplify_message(
    message_or_ref: &ReferenceOr<Message>,
    unique_parent_id: &str,
) -> SimplifiedMessage {
    if let ReferenceOr::Item(message) = message_or_ref {
        let mut unique_id: String = "".to_string();
        let payload = match &message.payload {
            Some(schema) => {
                if let Payload::Schema(schema) = schema {
                    let message_name = match &message.name {
                        Some(name) => name.to_string(),
                        None => {
                            format!("{}Message", unique_parent_id)
                        }
                    };
                    unique_id = validate_identifier_string(
                        &message_name,
                        false,
                    );
                    let simplified_schema = simplify_schema(schema, &unique_id);
                    Some(simplified_schema)
                } else {
                    None
                }
            }
            None => None,
        };
        SimplifiedMessage {
            unique_id,
            original_message: message.clone(),
            payload,
            payload_schema: message.payload_schema.clone(),
        }
    } else {
        panic!("Refs should be resolved by now");
    }
}

pub fn simplify_schema(schema: &Schema, unique_parent_id: &str) -> RustSchemaRepresentation {
    let schema_name = match &schema.schema_data.name {
        Some(name) => validate_identifier_string(name, false),
        None => validate_identifier_string(unique_parent_id, false),
    };
    parse_json_schema_to_rust_type(schema, &schema_name).unwrap()
}
