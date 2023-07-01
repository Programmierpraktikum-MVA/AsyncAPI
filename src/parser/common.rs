use std::{fs, io::ErrorKind, path::Path};

use regex::Regex;

pub fn read_json_or_yaml_to_value(file_path: &Path) -> serde_json::Value {
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
                    error("Error: The file '{:?}' could not be found.", file_path)
                }
                _ => error("An unexpected error occurred: {}", error),
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

pub fn resolve_json_path(json: serde_json::Value, path: &str) -> serde_json::Value {
    let parts = path.split('/').collect::<Vec<&str>>();
    let mut current_json = json;
    for part in parts {
        current_json = current_json[part].clone();
    }
    current_json
}
