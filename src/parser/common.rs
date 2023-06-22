use std::{collections::HashSet, fs, io::ErrorKind, path::Path};

use regex::Regex;

use crate::{asyncapi_model::AsyncAPI, parser::preprocessor::fill_message_and_payload_names};

use super::{
    preprocessor::{resolve_refs, sanitize_operation_ids_and_check_duplicate},
    validator::validate_asyncapi_schema,
};

use reqwest::blocking::get;

pub fn parse_spec_to_model(specs_dir: &Path) -> Result<AsyncAPI, Box<dyn std::error::Error>> {
    let spec = parse_string_to_serde_json_value(specs_dir);
    let version = parse_spec_version(&spec);

    // Construct the URL of the validator schema for the parsed version.
    let versioned_validator_schema_url =
        format!("https://asyncapi.com/definitions/{}.json", version);

    // Download the versioned validator schema.
    let validator = download_validator_schema(&versioned_validator_schema_url)?;

    // Validate the spec against the versioned validator schema.
    validate_asyncapi_schema(&validator, &spec);

    let preprocessed_spec = preprocess_schema(spec);
    let spec = serde_json::from_value::<AsyncAPI>(preprocessed_spec)?;

    Ok(spec)
}

fn download_validator_schema(url: &str) -> Result<serde_json::Value, reqwest::Error> {
    println!("📥 Downloading validator schema from: {}", url);
    let response = match get(url) {
        Ok(response) => response,
        Err(error) => {
            println!("❌ Error downloading validator schema: {}", error);
            std::process::exit(1);
        }
    };
    let validator = response.json()?;
    println!("✅ Validator schema downloaded successfully!");
    Ok(validator)
}

fn preprocess_schema(spec: serde_json::Value) -> serde_json::Value {
    let with_message_names = fill_message_and_payload_names(spec.clone(), spec, false, false, None);
    let resolved_refs = resolve_refs(with_message_names.clone(), with_message_names);
    let mut seen = HashSet::new();
    sanitize_operation_ids_and_check_duplicate(resolved_refs.clone(), resolved_refs, &mut seen)
}

fn parse_spec_version(spec: &serde_json::Value) -> String {
    let version = spec["asyncapi"].as_str().unwrap();
    version.to_string()
}

fn parse_string_to_serde_json_value(file_path: &Path) -> serde_json::Value {
    match fs::read_to_string(file_path) {
        Ok(file_string) => {
            // check if file is yaml or json
            let parsed_value = match file_path.extension() {
                Some(ext) => match ext.to_str() {
                    Some("yaml") | Some("yml") => {
                        serde_yaml::from_str::<serde_json::Value>(&file_string).unwrap()
                    }
                    Some("json") => {
                        serde_json::from_str::<serde_json::Value>(&file_string).unwrap()
                    }
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
        Err(error) => {
            match error.kind() {
                ErrorKind::NotFound => {
                    println!("Error: The file '{:?}' could not be found.", file_path)
                }
                _ => println!("An unexpected error occurred: {}", error),
            }
            panic!("Could not read file");
        }
    }
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
