use crate::{
    asyncapi_model::{
        schema::{IntegerFormat, NumberFormat, StringFormat, Type},
        VariantOrUnknownOrEmpty,
    },
    parser::common::validate_identifier_string,
};
use std::panic;

use super::{types::RustSchemaRepresentation, SchemaParserError};

pub fn primitive_type_to_string(
    schema_type: Type,
    property_name: &str,
) -> Result<RustSchemaRepresentation, SchemaParserError> {
    let variable_name = if !property_name.is_empty() {
        property_name.to_string()
    } else {
        "value_with_no_name".to_string()
    };

    Ok(RustSchemaRepresentation {
        unique_id: validate_identifier_string(&variable_name, false),
        struct_reference: format_to_rust_type(&schema_type),
        model_definition: "".to_string(),
        related_models: vec![],
        model_type: "primitive".to_string(),
    })
}

pub fn format_to_rust_type(schema_type: &Type) -> String {
    match schema_type {
        //TODO: Add suggested validators for each string format
        Type::String(_var) => {
            match &_var.format {
                VariantOrUnknownOrEmpty::Item(item) => {
                    match item {
                        StringFormat::Date => {
                            // Date format is usually "yyyy-mm-dd"
                            // You could validate it with a regex
                            // if date_regex.is_match(_var) {
                            //     return "String".to_string();
                            // }
                            // panic!("Invalid Date format for string");
                            "String".to_string()
                        },
                        StringFormat::DateTime => {
                            // DateTime format could be ISO 8601: "2023-01-16T23:28:56.782Z"
                            // You could validate it using chrono's parse function
                            // if let Ok(_dt) = chrono::DateTime::parse_from_rfc3339(_var) {
                            //     return "String".to_string();
                            // }
                            // panic!("Invalid DateTime format for string");
                            "String".to_string()
                        },
                        StringFormat::Password => {
                            // Password might not need specific validation, just keep as String
                            "String".to_string()
                        },
                        StringFormat::Byte => {
                            // Bytes could be base64 encoded string
                            // if base64::decode(_var).is_ok() {
                            //     return "String".to_string();
                            // }
                            // panic!("Invalid Byte format for string");
                            "String".to_string()
                        },
                        StringFormat::Binary => {
                            // Binary data might be just represented as a String
                            "String".to_string()
                        },
                    }
                },
                VariantOrUnknownOrEmpty::Unknown(_unknown) => "String".to_string(),
                VariantOrUnknownOrEmpty::Empty => "String".to_string(),
            }
        },
        Type::Number(_var) => {
            match &_var.format {
                VariantOrUnknownOrEmpty::Item(item) => {
                    match item {
                        NumberFormat::Float => "f32".to_string(),
                        NumberFormat::Double => "f64".to_string(),
                    }
                }
                VariantOrUnknownOrEmpty::Unknown(_unknown) => "f64".to_string(),
                VariantOrUnknownOrEmpty::Empty => "f64".to_string(),
            }
        },
        Type::Integer(_var) => {
            match &_var.format {
                VariantOrUnknownOrEmpty::Item(item) => {
                    match item {
                        IntegerFormat::Int32 => "i32".to_string(),
                        IntegerFormat::Int64 => "i64".to_string(),
                    }
                }
                VariantOrUnknownOrEmpty::Unknown(_unknown) => "i64".to_string(),
                VariantOrUnknownOrEmpty::Empty => "i64".to_string(),
            }
        },
        Type::Boolean{} => "bool".to_string(),
        _type => panic!("Unsupported primitive type: Currently only supports string, number, integer and boolean types"),
    }
}
