use crate::model::{self, AsyncAPI, ReferenceOr};
use std::{fs, io, path::Path};

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

pub fn spec_to_pubsub_template_type(spec: AsyncAPI) -> Result<model::PubsubTemplate, io::Error> {
    let item = spec.servers.first().unwrap().1;
    let server = match item {
        ReferenceOr::Item(it) => Some(it),
        ReferenceOr::Reference { reference: _ } => None,
    }
    .unwrap()
    .clone();

    Ok(model::PubsubTemplate {
        server_url: server.url,
        channel_name: spec.channels.first().unwrap().0.clone(),
    })
}
