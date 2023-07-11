use serde_json::json;
use std::{collections::HashSet, panic};

use crate::parser::common::{self, validate_identifier_string};

pub fn preprocess_schema(spec: serde_json::Value) -> serde_json::Value {
    let with_message_names = fill_message_and_payload_names(spec.clone(), spec, false, false, None);
    let resolved_refs = resolve_refs(with_message_names.clone(), with_message_names);
    let with_payload_schemas = duplicate_payload_schemas(resolved_refs.clone(), resolved_refs);
    let mut seen = HashSet::new();
    sanitize_operation_ids_and_check_duplicate(
        with_payload_schemas.clone(),
        with_payload_schemas,
        &mut seen,
    )
}

pub fn sanitize_operation_ids_and_check_duplicate(
    json: serde_json::Value,
    root_json: serde_json::Value,
    seen_operation_ids: &mut HashSet<String>,
) -> serde_json::Value {
    match json {
        serde_json::Value::Object(map) => {
            let mut new_map = serde_json::Map::new();
            for (key, value) in map {
                if key == "operationId" {
                    if let serde_json::Value::String(string_val) = &value {
                        let sanitized_val = validate_identifier_string(string_val.as_str(), false);
                        if seen_operation_ids.contains(&sanitized_val) {
                            panic!("Duplicate operationId found: {}", sanitized_val);
                        } else {
                            seen_operation_ids.insert(sanitized_val.clone());
                            new_map.insert(key, json!(sanitized_val));
                        }
                    } else {
                        panic!("operationId value is not a string");
                    }
                } else {
                    new_map.insert(
                        key,
                        sanitize_operation_ids_and_check_duplicate(
                            value,
                            root_json.clone(),
                            seen_operation_ids,
                        ),
                    );
                }
            }
            serde_json::Value::Object(new_map)
        }
        serde_json::Value::Array(array) => {
            let new_array = array
                .into_iter()
                .map(|value| {
                    sanitize_operation_ids_and_check_duplicate(
                        value,
                        root_json.clone(),
                        seen_operation_ids,
                    )
                })
                .collect();
            serde_json::Value::Array(new_array)
        }
        _ => json,
    }
}

pub fn resolve_refs(json: serde_json::Value, root_json: serde_json::Value) -> serde_json::Value {
    match json {
        serde_json::Value::Object(map) => {
            let mut new_map = serde_json::Map::new();
            for (key, value) in map {
                if key == "$ref" {
                    if let serde_json::Value::String(string_val) = value {
                        let correct_json = common::resolve_json_path(
                            root_json.clone(),
                            string_val.trim_start_matches("#/"),
                        );
                        return resolve_refs(correct_json, root_json);
                    } else {
                        panic!("$ref value is not a string");
                    }
                }
                let new_value = resolve_refs(value, root_json.clone());
                new_map.insert(key, new_value);
            }
            serde_json::Value::Object(new_map)
        }
        serde_json::Value::Array(array) => {
            let new_array = array
                .into_iter()
                .map(|value| resolve_refs(value, root_json.clone()))
                .collect();
            serde_json::Value::Array(new_array)
        }
        _ => json,
    }
}

pub fn duplicate_payload_schemas(
    json: serde_json::Value,
    root_json: serde_json::Value,
) -> serde_json::Value {
    match json {
        serde_json::Value::Object(map) => {
            let mut new_map = serde_json::Map::new();
            for (key, value) in map {
                if key == "payload" {
                    if let serde_json::Value::Object(schema) = value {
                        // insert schema as json string
                        new_map.insert(
                            "schema".into(),
                            serde_json::Value::String(serde_json::to_string(&schema).unwrap()),
                        );
                        new_map.insert("payload".into(), serde_json::Value::Object(schema.clone()));
                    }
                } else {
                    let new_value = duplicate_payload_schemas(value, root_json.clone());
                    new_map.insert(key, new_value);
                }
            }
            serde_json::Value::Object(new_map)
        }
        serde_json::Value::Array(array) => {
            let new_array = array
                .into_iter()
                .map(|value| duplicate_payload_schemas(value, root_json.clone()))
                .collect();
            serde_json::Value::Array(new_array)
        }
        _ => json,
    }
}

pub fn fill_message_and_payload_names(
    json: serde_json::Value,
    root_json: serde_json::Value,
    is_message: bool,
    is_message_map: bool,
    message_name: Option<&str>,
) -> serde_json::Value {
    match json {
        serde_json::Value::Object(map) => {
            let mut new_map = serde_json::Map::new();
            for (key, value) in map {
                let mut msg_name: Option<&str> = None;
                if is_message_map {
                    msg_name = Some(key.as_str());
                }
                let mut inside_message = is_message_map;
                if key == "message" {
                    inside_message = true;
                }
                let mut inside_message_map = false;
                if key == "messages" || key == "schemas" {
                    inside_message_map = true;
                }
                let new_value = fill_message_and_payload_names(
                    value,
                    root_json.clone(),
                    inside_message,
                    inside_message_map,
                    msg_name,
                );
                new_map.insert(key, new_value);
            }
            if !new_map.contains_key("name") && is_message {
                if let Some(message_name) = message_name {
                    new_map.insert("name".to_string(), json!(message_name));
                }
            }

            serde_json::Value::Object(new_map)
        }
        serde_json::Value::Array(array) => {
            let new_array = array
                .into_iter()
                .map(|value| {
                    fill_message_and_payload_names(
                        value,
                        root_json.clone(),
                        is_message,
                        is_message_map,
                        message_name,
                    )
                })
                .collect();
            serde_json::Value::Array(new_array)
        }
        _ => json,
    }
}

#[cfg(test)]
mod tests {
    use std::{fs, path::Path};

    use super::*;
    use crate::*;

    const SCHEMAS: [&str; 1] = ["./example/specs/basic_ref.yml"];

    //parse file to json, allowed files are yaml and json
    fn parse_test(path: &Path) -> serde_json::Value {
        let string_content = fs::read_to_string(path).expect("file could not be read");
        // check if file is yaml or json
        let parsed: serde_json::Value = match path.extension() {
            Some(ext) => match ext.to_str() {
                Some("yaml") => serde_yaml::from_str::<serde_json::Value>(&string_content).unwrap(),
                Some("yml") => serde_yaml::from_str::<serde_json::Value>(&string_content).unwrap(),
                Some("json") => serde_json::from_str::<serde_json::Value>(&string_content).unwrap(),
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
    fn resolves_refs() {
        for schema_paths in SCHEMAS {
            let definition = parse_test(Path::new(schema_paths));
            let resolved: serde_json::Value = resolve_refs(definition.clone(), definition.clone());
            let filename_without_extension = Path::new(schema_paths)
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap();
            let out_dir = Path::new("./test_output/{}.rs").join(filename_without_extension);
            utils::write_to_path_create_dir(&resolved.to_string(), &out_dir).unwrap();
        }
    }
}
