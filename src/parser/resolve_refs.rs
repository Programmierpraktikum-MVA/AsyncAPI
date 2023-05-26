pub fn resolve_json_path(json: serde_json::Value, path: &str) -> serde_json::Value {
    let parts = path.split('/').collect::<Vec<&str>>();
    let mut current_json = json;
    for part in parts {
        current_json = current_json[part].clone();
    }
    current_json
}

pub fn resolve_refs(json: serde_json::Value, root_json: serde_json::Value) -> serde_json::Value {
    match json {
        serde_json::Value::Object(map) => {
            let mut new_map = serde_json::Map::new();
            for (key, value) in map {
                if key == "$ref" {
                    if let serde_json::Value::String(string_val) = value {
                        let correct_json = resolve_json_path(
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
