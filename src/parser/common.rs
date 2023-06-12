use std::{collections::HashSet, fs, path::Path};

use regex::Regex;

use crate::asyncapi_model::AsyncAPI;

use super::{
    preprocessor::{resolve_refs, sanitize_operation_ids_and_check_duplicate},
    validator::validate_asyncapi_schema,
};

pub fn parse_spec_to_model(
    spec_path: &Path,
    validator_schema_path: &Path,
) -> Result<AsyncAPI, serde_json::Error> {
    let spec = parse_string_to_serde_json_value(spec_path);
    let validator = parse_string_to_serde_json_value(validator_schema_path);

    validate_asyncapi_schema(&validator, &spec);

    let preprocessed_spec = preprocess_schema(spec);
    let spec = serde_json::from_value::<AsyncAPI>(preprocessed_spec)?;
    Ok(spec)
}

fn preprocess_schema(spec: serde_json::Value) -> serde_json::Value {
    let resolved_refs = resolve_refs(spec.clone(), spec);
    let mut seen = HashSet::new();
    let sanitized =
        sanitize_operation_ids_and_check_duplicate(resolved_refs.clone(), resolved_refs, &mut seen);
    println!("Preprocessed spec: {}", sanitized);
    sanitized
}

fn parse_string_to_serde_json_value(file_path: &Path) -> serde_json::Value {
    let file_string = fs::read_to_string(file_path).expect("File could not be read");
    // check if file is yaml or json
    let parsed_value = match file_path.extension() {
        Some(ext) => match ext.to_str() {
            Some("yaml") | Some("yml") => {
                serde_yaml::from_str::<serde_json::Value>(&file_string).unwrap()
            }
            Some("json") => serde_json::from_str::<serde_json::Value>(&file_string).unwrap(),
            _ => {
                panic!("File has an unsupported extension");
            }
        },
        None => {
            panic!("File has no extension");
        }
    };
    parsed_value
}

pub fn validate_identifier_string(s: &str, camel_case: bool) -> String {
    let re = Regex::new(r"[^\w\s]").unwrap();
    let mut sanitized = re.replace_all(s, "").to_string();

    // split into words and process each word
    let words: Vec<&str> = sanitized.split_whitespace().collect();

    if camel_case {
        sanitized = words
            .into_iter()
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(f) => f.to_uppercase().chain(chars).collect(),
                }
            })
            .collect();
    } else {
        sanitized = words.join("_").to_lowercase();
    }

    sanitized
}
