use std::collections::HashMap;

use serde::Serialize;

use crate::asyncapi_model::{
    Message, Operation, OperationMessageType, Payload, ReferenceOr, Schema,
};
use crate::parser::{schema_parser_mapper, validate_identifier_string};

#[derive(Serialize, Debug)]
pub struct SimplifiedOperation {
    pub unique_id: String,
    pub original_operation: Operation,
    // array, da es eine oder mehrere messages geben kann
    pub messages: Vec<SimplifiedMessage>,
}
#[derive(Serialize, Debug)]

pub struct SimplifiedMessage {
    pub unique_id: String,
    pub original_message: Message,
    pub payload: Option<SimplifiedSchema>,
}
#[derive(Serialize, Debug)]

pub struct SimplifiedSchema {
    pub unique_id: String,
    pub original_schema: Schema,
    pub struct_definition: String,
    pub struct_name: String,
}

pub fn simplify_operation(operation: &Operation, channel_name: &str) -> SimplifiedOperation {
    let unique_id = operation
        .operation_id
        .clone()
        .unwrap_or_else(|| validate_identifier_string(channel_name));

    let messages: Vec<SimplifiedMessage> = match &operation.message {
        Some(operation_message) => match operation_message {
            OperationMessageType::Map(map) => map
                .into_iter()
                .map(|(_, m)| simplify_message(m, &unique_id))
                .collect(),
            OperationMessageType::Single(message_or_ref) => {
                vec![simplify_message(message_or_ref, &unique_id)]
            }
        },
        _ => vec![],
    };
    SimplifiedOperation {
        unique_id,
        original_operation: operation.clone(),
        messages,
    }
}

pub fn simplify_message(
    message_or_ref: &ReferenceOr<Message>,
    unique_parent_id: &str,
) -> SimplifiedMessage {
    let unique_id = format!("{}Message", unique_parent_id);
    if let ReferenceOr::Item(message) = message_or_ref {
        let payload: Option<SimplifiedSchema> = match &message.payload {
            Some(schema) => {
                if let Payload::Schema(schema) = schema {
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
        }
    } else {
        panic!("Refs should be resolved by now");
    }
}
pub fn simplify_schema(schema: &Schema, unique_parent_id: &str) -> SimplifiedSchema {
    let unique_id = format!("{}Type", unique_parent_id);
    let mut schemas = HashMap::<String, String>::new();
    let struct_name = schema_parser_mapper(schema, &unique_id, &mut schemas).unwrap();
    SimplifiedSchema {
        unique_id,
        original_schema: schema.clone(),
        struct_definition: schemas.into_values().collect::<Vec<String>>().join("\n"),
        struct_name,
    }
}
