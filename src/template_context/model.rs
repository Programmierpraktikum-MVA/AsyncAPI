use crate::parser::json_schema_parser::types::RustSchemaRepresentation;

use super::{Model, SimplifiedOperation};

// extracts all message models from the pub and sub channels, makes sure no model with the same identifyer is added twice
pub fn extract_model_from_channels(
    pub_channels: Vec<(&String, SimplifiedOperation)>,
    sub_channels: Vec<(&String, SimplifiedOperation)>,
) -> Model {
    let all_message_payloads = pub_channels
        .into_iter()
        .chain(sub_channels.into_iter())
        .flat_map(|(_key, operation)| operation.messages)
        .map(|message| message.payload);

    let all_okay = all_message_payloads.flatten();
    let all_schemas: Vec<RustSchemaRepresentation> = all_okay
        .flat_map(|x| x.get_related_models_recursive())
        .collect();

    let all_unique_ids = all_schemas.iter().map(|x| x.identifier.clone());
    let unique_ids_set: std::collections::HashSet<String> = all_unique_ids.into_iter().collect();
    let unique_ids = unique_ids_set.into_iter().collect::<Vec<String>>();
    let unique_schemas = unique_ids
        .into_iter()
        .map(|unique_id| {
            let schema = all_schemas
                .iter()
                .find(|schema| schema.identifier == unique_id)
                .unwrap();
            schema.clone()
        })
        .collect::<Vec<RustSchemaRepresentation>>();

    Model {
        message_models: unique_schemas,
    }
    // let mut messages: HashMap<String, SimplifiedMessage> = HashMap::new();
    // let mut enums: HashMap<String, MultiStructEnum> = HashMap::new();

    // // Chain the pub_channels and sub_channels vectors into a single iterator
    // for (_, operation) in pub_channels.into_iter().chain(sub_channels.into_iter()) {
    //     for message in operation.messages {
    //         messages.insert(message.unique_id.clone(), message);
    //     }
    //     // if let Some(multiple_messages_enum) = operation.multiple_messages_enum {
    //     //     enums.insert(
    //     //         multiple_messages_enum.unique_id.clone(),
    //     //         multiple_messages_enum,
    //     //     );
    //     // }
    // }
    // Model {
    //     messages: messages.into_values().collect(),
    //     // enums: enums.into_values().collect(),
    // }
}
