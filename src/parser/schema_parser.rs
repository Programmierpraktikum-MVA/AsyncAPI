use crate::asyncapi_model::{
    schema::{ObjectType, SchemaKind, Type},
    ReferenceOr, Schema,
};
use core::fmt;
use std::{collections::HashMap, format};

use super::common::convert_string_to_valid_type_name;

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
    let before_string = format!(
        "#[derive(Clone, Debug, Deserialize, Serialize)]\npub struct {} {{\n",
        convert_string_to_valid_type_name(property_name, "")
    );
    let after_string = String::from("\n}\n");
    let property_string_iterator: Vec<Result<String, SchemaParserError>> = schema
        .properties
        .iter()
        .map(|(key, val)| match val {
            ReferenceOr::Item(x) => schema_parser_mapper(x, key, all_structs),
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

fn sanitize_property_name(property_name: &str) -> String {
    // TODO: do proper sanitization so that the property name is a valid rust identifier
    property_name.replace('-', "_")
}

fn primitive_type_to_string(
    schema_type: Type,
    property_name: &str,
) -> Result<String, SchemaParserError> {
    // TODO: Add support for arrays
    match schema_type {
        Type::String(_var) => Ok(format!("pub {}: String", sanitize_property_name(property_name))),
        Type::Number(_var) => Ok(format!("pub {}: f64", sanitize_property_name(property_name))),
        Type::Integer(_var) => Ok(format!("pub {}: int64", sanitize_property_name(property_name)) ),
        Type::Boolean{} => Ok(format!("pub {}: bool", sanitize_property_name(property_name))),
        _type => Err(SchemaParserError::GenericError("Unsupported primitive type: Currently only supports string, number, integer and boolean types".to_string(), Some(property_name.into()))),
    }
}

pub fn schema_parser_mapper(
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
                    property_name,
                    convert_string_to_valid_type_name(struct_name.as_str(), "").as_str()
                ))
            }
            _primitive_type => primitive_type_to_string(_primitive_type.clone(), property_name),
        },
        _other_schema_kind => {
            panic!("Unsupported schema kind {:?}", _other_schema_kind);
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
                schema_parser_mapper(&s, &name, structs).unwrap();
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
