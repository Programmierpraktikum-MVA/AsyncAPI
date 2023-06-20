use crate::asyncapi_model::schema::ArrayType;

use super::{primitive_schema::format_to_rust_type, *};

pub fn parse_array_schema(
    array_type: &ArrayType,
    property_name: &str,
) -> Result<RustSchemaRepresentation, SchemaParserError> {
    let identifyer = validate_identifier_string(property_name, true);

    let item_type = match &array_type.items {
        Some(type_box) => match type_box {
            ReferenceOr::Item(schema) => match &schema.schema_kind {
                SchemaKind::Type(schema_type) => format_to_rust_type(schema_type),
                _ => panic!("Unsupported schema kind"),
            },
            ReferenceOr::Reference { reference: _ } => {
                return Err(SchemaParserError::GenericError(
                    "References are not supported yet".to_string(),
                    property_name.to_string().into(),
                ))
            }
        },
        None => {
            return Err(SchemaParserError::GenericError(
                "Array type without item type".into(),
                None,
            ))
        }
    };
    // Ok(format!(
    //     "pub {}: Vec<{}>",
    //     validate_identifier_string(property_name, false),
    //     item_type
    // ))
    Ok(RustSchemaRepresentation {
        identifier: identifyer,
        struct_reference: format!("Vec<{}>", item_type),
        model_definition: "".to_string(),
        related_models: vec![],
        model_type: "array".to_string(),
    })
}
