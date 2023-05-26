use std::{fs, path::Path};

use inflector::Inflector;
use regex::Regex;

use crate::asyncapi_model::AsyncAPI;

pub fn parse_spec_to_model(path: &Path) -> Result<AsyncAPI, serde_json::Error> {
    let string_content = fs::read_to_string(path).expect("file could not be read");
    // check if file is yaml or json
    let parsed = match path.extension() {
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
    let with_resolved_references =
        crate::parser::resolve_refs::resolve_refs(parsed.clone(), parsed.clone());
    let spec = serde_json::from_value::<AsyncAPI>(with_resolved_references)?;
    Ok(spec)
}

fn capitalize_first_char(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

pub fn convert_string_to_valid_type_name(s: &str, suffix: &str) -> String {
    let re = Regex::new(r"[^\w\s]").unwrap();
    // Remove special chars, capitalize words, remove spaces
    let mut root_msg_name = re.replace_all(s, " ").to_title_case().replace(' ', "");
    // Append Message to the end of the name
    root_msg_name.push_str(suffix);
    capitalize_first_char(root_msg_name.as_str())
}
