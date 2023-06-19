use crate::{
    asyncapi_model::{
        schema::{
            ArrayType, IntegerFormat, NumberFormat, ObjectType, SchemaKind, StringFormat, Type,
        },
        ReferenceOr, Schema, VariantOrUnknownOrEmpty,
    },
    template_model::{MultiStructEnum, SimplifiedMessage},
};
use core::fmt;
use std::{collections::HashMap, format, panic};

use super::common::validate_identifier_string;

#[derive(Debug, Clone)]
pub enum SchemaParserError {
    // error message, property name
    GenericError(String, Option<String>),
}
impl fmt::Display for SchemaParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SchemaParserError::GenericError(msg, property_name) => {
                let base_message = match property_name {
                    Some(name) => format!(
                        "Error while parsing schema, inside property:\"{}\";\n Message: {} ",
                        msg, name
                    ),
                    None => "Error while parsing schema".to_string() + &msg.to_string(),
                };
                write!(f, "{}", base_message)
            }
        }
    }
}
impl std::error::Error for SchemaParserError {}

// parses object definition to rust struct, inserts struct into hashmap, returns struct name
fn object_schema_to_string(
    schema: &ObjectType,
    property_name: &str,
    all_structs: &mut HashMap<String, String>,
) -> Result<String, SchemaParserError> {
    let before_string: String = format!(
        "#[derive(Clone, Debug, Deserialize, Serialize)]\npub struct {} {{\n",
        validate_identifier_string(property_name, true)
    );
    let after_string = String::from("\n}\n");
    let property_string_iterator: Vec<Result<String, SchemaParserError>> = schema
        .properties
        .iter()
        .map(|(key, val)| match val {
            ReferenceOr::Item(x) => schema_to_rust_types(x, key, all_structs),
            ReferenceOr::Reference { reference: _ } => Err(SchemaParserError::GenericError(
                "References are not supported yet".to_string(),
                property_name.to_string().into(),
            )),
        })
        .collect();

    // check for errors and return early if any
    if let Some(Err(e)) = property_string_iterator.iter().find(|x| x.is_err()) {
        return Err(e.clone());
    }

    let property_string_iterator = property_string_iterator.into_iter().map(|x| x.unwrap());

    let property_string = property_string_iterator
        .collect::<Vec<String>>()
        .join(",\n");
    let full_struct = before_string + &property_string + &after_string;
    all_structs.insert(property_name.to_string(), full_struct);
    Ok(property_name.to_string())
}

fn format_to_rust_type(schema_type: &Type) -> String {
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

fn primitive_type_to_string(
    schema_type: Type,
    property_name: &str,
) -> Result<String, SchemaParserError> {
    let variable_name = if !property_name.is_empty() {
        validate_identifier_string(property_name, false)
    } else {
        "value_with_no_name".to_string()
    };
    Ok(format!(
        "pub {}: {}",
        variable_name,
        format_to_rust_type(&schema_type)
    ))
}

fn array_type_to_string(
    array_type: &ArrayType,
    property_name: &str,
) -> Result<String, SchemaParserError> {
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
    Ok(format!(
        "pub {}: Vec<{}>",
        validate_identifier_string(property_name, false),
        item_type
    ))
}

pub fn build_multi_message_enum(
    multiple_messages: &Vec<SimplifiedMessage>,
    unique_id: &str,
) -> Option<MultiStructEnum> {
    if multiple_messages.len() == 1 {
        None
    } else {
        let mut string_builder: String = format!(
            "#[derive(Serialize, Deserialize, Debug)]\n#[serde(untagged)]\npub enum {} {{\n",
            validate_identifier_string(unique_id, true)
        );
        for message in multiple_messages {
            let message_name = validate_identifier_string(&message.unique_id, true);
            let message_string = format!("{}({}),\n", message_name, message_name);
            string_builder.push_str(&message_string);
        }
        string_builder.push_str("\n}\n");
        Some(MultiStructEnum {
            unique_id: unique_id.to_string(),
            messages: multiple_messages.to_vec(),
            struct_definition: string_builder,
        })
    }
}

pub fn build_multi_payload_enum(struct_names: &Vec<String>, enum_name: &str) -> Option<String> {
    if struct_names.len() == 1 {
        None
    } else {
        let name = validate_identifier_string(enum_name, true);
        let mut string_builder: String = format!(
            "#[derive(Serialize, Deserialize, Debug)]\n#[serde(untagged)]\npub enum {} {{\n",
            name
        );
        for message in struct_names {
            let message_name = validate_identifier_string(message, true);
            let message_string = format!("{}({}),\n", message_name, message_name);
            string_builder.push_str(&message_string);
        }
        string_builder.push_str("\n}\n");
        Some(string_builder)
    }
}

// this is a message with just the payload, it should be able to be extended with headers and other props
pub fn build_multi_payload_message(message_name: &str, payload_name: &str) -> String {
    format!(
        "#[derive(Serialize, Deserialize, Debug)]\npub struct {} {{\npayload: {},\n}}\n",
        validate_identifier_string(message_name, true),
        validate_identifier_string(payload_name, true),
    )
}

pub fn schema_to_rust_types(
    schema: &Schema,
    property_name: &str,
    all_structs: &mut HashMap<String, String>,
) -> Result<String, SchemaParserError> {
    let schema_kind: &SchemaKind = &schema.schema_kind;
    match schema_kind {
        SchemaKind::Type(schema_type) => match schema_type {
            Type::Object(y) => {
                let struct_name = object_schema_to_string(y, property_name, all_structs)?;
                Ok(format!(
                    "pub {}: {}",
                    struct_name,
                    validate_identifier_string(struct_name.as_str(), true).as_str()
                ))
            }
            Type::Array(array_type) => array_type_to_string(array_type, property_name),
            _primitive_type => primitive_type_to_string(_primitive_type.clone(), property_name),
        },
        SchemaKind::OneOf { one_of }
        | SchemaKind::AnyOf { any_of: one_of }
        | SchemaKind::AllOf { all_of: one_of } => {
            let mut combined_string = String::new();
            let mut struct_names: Vec<String> = vec![];
            for (index, schema) in one_of.iter().enumerate() {
                match schema {
                    ReferenceOr::Item(item_schema) => {
                        let payload_variant_name = format!("{}Payload{}", property_name, index + 1);
                        let result = schema_to_rust_types(
                            item_schema,
                            payload_variant_name.as_str(),
                            all_structs,
                        )?;
                        combined_string.push_str(&result);
                        combined_string.push('\n');
                        struct_names.push(payload_variant_name);
                    }
                    ReferenceOr::Reference { reference: _ } => {
                        panic!("Refs should be resolved by now");
                    }
                }
            }
            let payload_enum_name = format!("{}PayloadEnum", property_name);

            let payload_enum = build_multi_payload_enum(&struct_names, payload_enum_name.as_str());
            if let Some(payload_enum) = payload_enum {
                combined_string.push_str(payload_enum.as_str());
                all_structs.insert(payload_enum_name.clone(), payload_enum);
                let final_message = build_multi_payload_message(property_name, &payload_enum_name);
                all_structs.insert(property_name.to_string(), final_message);
            }
            Ok(combined_string)
        }
        SchemaKind::Any(_s) => {
            panic!("Unsupported schema kind {:?}", _s);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, fs, path::Path};

    use super::*;
    use crate::*;

    const SCHEMAS: [&str; 3] = [
        "./example/schemas/userPayload.json",
        "./example/schemas/signupSubscriber.yaml",
        "./example/schemas/userPayloadNested.json",
    ];

    //parse file to json, allowed files are yaml and json
    fn parse_test(path: &Path) -> HashMap<String, Schema> {
        let string_content = fs::read_to_string(path).expect("file could not be read");
        // check if file is yaml or json
        let parsed: HashMap<String, Schema> = match path.extension() {
            Some(ext) => match ext.to_str() {
                Some("yaml") => {
                    serde_yaml::from_str::<HashMap<String, Schema>>(&string_content).unwrap()
                }
                Some("json") => {
                    serde_json::from_str::<HashMap<String, Schema>>(&string_content).unwrap()
                }
                _ => {
                    panic!("file has no extension");
                }
            },
            None => {
                panic!("file has no extension");
            }
        };
        parsed
    }

    #[test]
    fn can_parse_schema() {
        for schema_paths in SCHEMAS {
            let definition = parse_test(Path::new(schema_paths));
            for (name, schema) in definition {
                let s = Box::new(schema);
                let structs = &mut HashMap::new();
                schema_to_rust_types(&s, &name, structs).unwrap();
                let filename_without_extension = Path::new(schema_paths)
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap();
                let out_dir = Path::new("./test_output/{}.rs").join(filename_without_extension);
                utils::write_to_path_create_dir(
                    structs
                        .iter()
                        .map(|(_, v)| v.to_string())
                        .collect::<Vec<String>>()
                        .join("\n")
                        .as_str(),
                    &out_dir,
                )
                .unwrap();
            }
        }
    }
}
