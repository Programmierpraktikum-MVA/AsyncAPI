use crate::model;
use std::{path::Path, fs};

pub fn parse_asyncapi_yaml_file(path: &Path) -> Result<model::AsyncAPI, serde_yaml::Error> {
    let string_content = fs::read_to_string(path).expect("file could not be read");
    // check if file is yaml or json
    let parsed = match path.extension() {
        Some(ext) => match ext.to_str() {
            Some("yaml") => serde_yaml::from_str::<model::AsyncAPI>(&string_content).unwrap(),
            Some("json") => serde_json::from_str::<model::AsyncAPI>(&string_content).unwrap(),
            _ => {
                panic!("file has no extension");
            }
        },
        None => {
            panic!("file has no extension");
        }
    };
    Ok(parsed)
}