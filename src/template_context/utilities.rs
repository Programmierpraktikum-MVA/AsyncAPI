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
        .unwrap_or_else(|| validate_identifier_string(channel_name, true));

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
                    unique_id = validate_identifier_string(
                        format!(
                            "{}{}Message",
                            message.name.as_ref().unwrap_or(
                                schema
                                    .schema_data
                                    .name
                                    .as_ref()
                                    .unwrap_or(&String::from(""))
                            ),
                            unique_parent_id
                        )
                        .as_str(),
                        true,
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
        }
    } else {
        panic!("Refs should be resolved by now");
    }
}

pub fn simplify_schema(schema: &Schema, unique_parent_id: &str) -> RustSchemaRepresentation {
    parse_json_schema_to_rust_type(schema, unique_parent_id).unwrap()
    // let rust_schema = parse_json_schema_to_rust_type(schema, unique_parent_id).unwrap();
    // let mut schema_source = rust_schema.related_models.clone();
    // schema_source.push(rust_schema.clone());
    // let schemas = schema_source
    //     .into_iter().map(|s| s.model_definition).collect::<Vec<String>>().join("\n");
    // let struct_name =rust_schema.identifier.clone();
    // TODO: this whole thing will need to be refactored, there's no way this will work in this form
    // the idea is that we need to get the payload enum and its members out of the schema
    // but we save it as string only... so the whole parsing function will need to be restructured and way more modular
    // why you ask? we want to automatically generate match code for the payload, but currently it wont work without refactor

    // let payload_enum_name = format!("{}PayloadEnum", unique_parent_id);
    // let mut multiple_payload_enum = None;
    // if schemas.contains_key(&payload_enum_name) {
    //     multiple_payload_enum = Some(MultiStructEnum {
    //         unique_id: payload_enum_name,
    //         messages: vec![],
    //         struct_definition: "".to_string(),
    //     });
    // }
    // RustSchemaRepresentation {
    //     unique_id: unique_parent_id.to_string(),
    //     original_schema: schema.clone(),
    //     struct_definition: schemas,
    //     struct_names: vec![struct_name],
    //     // multiple_payload_enum: None,
    // }
}
