use crate::asyncapi_model::{
    schema::{ObjectType, SchemaKind, Type},
    ReferenceOr, Schema,
};
use std::{collections::HashMap, format};

use super::common::convert_string_to_valid_type_name;

// parses object definition to rust struct, inserts struct into hashmap, returns struct name
fn object_schema_to_string(
    schema: &ObjectType,
    property_name: &str,
    all_structs: &mut HashMap<String, String>,
) -> String {
    let before_string = format!(
        "#[derive(Clone, Debug, Deserialize, Serialize)]\npub struct {} {{\n",
        convert_string_to_valid_type_name(property_name, "")
    );
    let after_string = String::from("\n}\n");
    let property_string_it = schema.properties.iter().map(|(key, val)| match val {
        ReferenceOr::Item(x) => schema_parser_mapper(x, key, all_structs),
        _ => {
            panic!("Currently only supports string types")
        }
    });
    let property_string = property_string_it.collect::<Vec<String>>().join(",\n");
    let full_struct = before_string + &property_string + &after_string;
    all_structs.insert(property_name.to_string(), full_struct);
    property_name.to_string()
}

fn sanitize_property_name(property_name: &str) -> String {
    // TODO: do proper sanitization so that the property name is a valid rust identifier
    property_name.replace('-', "_")
}

fn primitive_type_to_string(schema_type: Type, property_name: &str) -> String {
    // TODO: Add support for arrays
    match schema_type {
        Type::String(_var) => format!("pub {}: String", sanitize_property_name(property_name)),
        Type::Number(_var) => format!("pub {}: f64", sanitize_property_name(property_name)),
        Type::Integer(_var) => format!("pub {}: int64", sanitize_property_name(property_name)) ,
        Type::Boolean{} => format!("pub {}: bool", sanitize_property_name(property_name)),
        _type => panic!("Unsupported schema type: Currently only supports string, number, integer and boolean types"),
    }
}

pub fn schema_parser_mapper(
    schema: &Schema,
    property_name: &str,
    all_structs: &mut HashMap<String, String>,
) -> String {
    let schema_kind: &SchemaKind = &schema.schema_kind;
    match schema_kind {
        SchemaKind::Type(schema_type) => match schema_type {
            Type::Object(y) => {
                println!("object schema: {:?}", y);
                let struct_name = object_schema_to_string(y, property_name, all_structs);
                format!(
                    "pub {}: {}",
                    property_name,
                    convert_string_to_valid_type_name(struct_name.as_str(), "").as_str()
                )
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
                schema_parser_mapper(&s, &name, structs);
                let filename_without_extension = Path::new(schema_paths)
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap();
                let out_dir = Path::new("./test_output/{}.rs").join(filename_without_extension);
                utils::write_to_file(
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
