use crate::asyncapi_model::{
    schema::{SchemaKind, Type},
    ReferenceOr, Schema,
};

pub mod types;
pub use types::SchemaParserError;
pub mod primitive_schema;

mod array_schema;
pub mod enum_schema;
mod object_schema;
use self::{enum_schema::parse_enum_schema, types::RustSchemaRepresentation};

use super::common::validate_identifier_string;

// parses a json schema to a rust type
pub fn parse_json_schema_to_rust_type(
    schema: &Schema,
    property_name: &str,
) -> Result<RustSchemaRepresentation, types::SchemaParserError> {
    let schema_kind: &SchemaKind = &schema.schema_kind;
    match schema_kind {
        SchemaKind::Type(schema_type) => match schema_type {
            Type::Object(y) => object_schema::parse_object_schema(y, property_name),
            Type::Array(array_type) => array_schema::parse_array_schema(array_type, property_name),
            _primitive_type => {
                primitive_schema::primitive_type_to_string(_primitive_type.clone(), property_name)
            }
        },
        SchemaKind::OneOf { one_of }
        | SchemaKind::AnyOf { any_of: one_of }
        | SchemaKind::AllOf { all_of: one_of } => {
            parse_enum_schema(one_of, property_name)
            //     let rust_schemas = one_of.iter().enumerate().map(|(index,schema)|
            //     match schema {
            //         ReferenceOr::Item(item_schema) => {
            //             let payload_variant_name = format!("{}Payload{}", property_name, index + 1);
            //              parse_json_schema_to_rust_type(
            //                 item_schema,
            //                 payload_variant_name.as_str(),
            //             )

            //             // combined_string.push_str(&result.identifier);
            //             // combined_string.push('\n');
            //             // struct_names.push(payload_variant_name);
            //         }
            //         ReferenceOr::Reference { reference: _ } => {
            //             panic!("Refs should be resolved by now");
            //         }
            //     }
            // ).map(|r| r.unwrap()).collect::<Vec<RustSchemaRepresentation>>();

            //     let payload_enum_name = format!("{}PayloadEnum", property_name);

            //     let struct_names = rust_schemas.iter().map(|e| e.identifier.clone()).collect::<Vec<String>>();
            //     let payload_enum =
            //         enum_schema::build_multi_payload_enum(&struct_names, payload_enum_name.as_str());
            //     if let Some(_payload_enum) = payload_enum {
            //         let final_message =
            //         enum_schema::build_multi_payload_message(property_name, &payload_enum_name);
            //         Ok(
            //             RustSchemaRepresentation {
            //                 identifier: validate_identifier_string(property_name, false),
            //                 model_definition: final_message,
            //                 related_models: rust_schemas.iter().flat_map(|e| e.related_models.clone()).collect(),
            //                 model_type: "enum".to_string(),
            //                 struct_reference: common::validate_identifier_string(property_name, true)
            //             }
            //         )
            //     }
            //     else {
            //         panic!("Failed to build payload enum");
            //     }
        }
        SchemaKind::Any(_s) => {
            panic!("Unsupported schema kind {:?}", _s);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, fs, path::Path};

    use crate::{asyncapi_model::Schema, *};

    use super::parse_json_schema_to_rust_type;

    const SCHEMAS: [&str; 4] = [
        "./example/schemas/userPayload.json",
        "./example/schemas/signupSubscriber.yaml",
        "./example/schemas/userPayloadNested.json",
        "./example/schemas/anyOf.yaml",
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
                let parsed = parse_json_schema_to_rust_type(&s, &name).unwrap();
                let filename_without_extension = Path::new(schema_paths)
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap();
                let out_dir = Path::new("./test_output/{}.rs").join(filename_without_extension);
                utils::write_to_path_create_dir(
                    parsed
                        .get_related_models_recursive()
                        .iter()
                        .map(|x| x.model_definition.clone())
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
