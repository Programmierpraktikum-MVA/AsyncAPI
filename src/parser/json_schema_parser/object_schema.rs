use super::*;
use crate::{
    asyncapi_model::{schema::ObjectType, ReferenceOr},
    parser::common::validate_identifier_string,
};

// parses object definition to rust struct, inserts struct into hashmap, returns struct name
pub fn parse_object_schema(
    schema: &ObjectType,
    property_name: &str,
) -> Result<RustSchemaRepresentation, SchemaParserError> {
    let identifyer = validate_identifier_string(property_name, true);
    let before_string: String = format!(
        "#[derive(Clone, Debug, Deserialize, Serialize)]\npub struct {} {{\n",
        identifyer
    );
    let after_string = String::from("\n}\n");
    let property_types: Vec<Result<RustSchemaRepresentation, SchemaParserError>> = schema
        .properties
        .iter()
        .map(|(key, val)| match val {
            ReferenceOr::Item(x) => parse_json_schema_to_rust_type(x, key),
            ReferenceOr::Reference { reference: _ } => Err(SchemaParserError::GenericError(
                "References are not supported yet".to_string(),
                property_name.to_string().into(),
            )),
        })
        .collect();

    // check for errors and return early if any
    if let Some(Err(e)) = property_types.iter().find(|x| x.is_err()) {
        return Err(e.clone());
    }

    let unwrapped_property_types = property_types
        .into_iter()
        .map(|x| x.unwrap())
        .collect::<Vec<RustSchemaRepresentation>>();

    let property_string = unwrapped_property_types
        .iter()
        .map(|x| format!("pub {}: {}", x.unique_id, x.struct_reference))
        .collect::<Vec<String>>()
        .join(",\n");
    let full_struct = before_string + &property_string + &after_string;

    let representation: RustSchemaRepresentation = RustSchemaRepresentation {
        unique_id: identifyer.clone(),
        struct_reference: identifyer,
        model_definition: full_struct,
        related_models: unwrapped_property_types,
        model_type: "struct".to_string(),
    };
    Ok(representation)
}
