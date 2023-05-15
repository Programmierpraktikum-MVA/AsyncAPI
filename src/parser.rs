use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File, io::BufReader, path::Path};

/// TODO: lots more possible fields (see: https://docs.rs/asyncapi/0.2.0/asyncapi/index.html) for more information
/// deserialized version of the spec file
#[derive(Debug, Serialize, Deserialize)]
pub struct AsyncApi {
    pub asyncapi: String,
    pub info: Info,
    pub servers: HashMap<String, Server>,
    pub channels: HashMap<String, Channel>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Server {
    pub url: String,
    pub protocol: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Info {
    pub title: String,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Channel {
    pub subscribe: Option<Operation>,
    pub publish: Option<Operation>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Operation {
    #[serde(rename = "operationId")]
    pub operation_id: String,
    pub summary: String,
    pub message: Message,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub payload: Payload,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Payload {
    #[serde(rename = "type")]
    pub data_type: String,
}

pub fn parse_asyncapi_yaml_file(path: &Path) -> Result<AsyncApi, serde_yaml::Error> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let asyncapi_spec = serde_yaml::from_reader(reader)?;
    Ok(asyncapi_spec)
}

impl AsyncApi {
    pub fn get_subscribe_channels(&self) -> Vec<(&String, &Operation)> {
        self.channels
            .iter()
            .filter_map(|(channel_name, channel)| {
                channel
                    .subscribe
                    .as_ref()
                    .map(|operation| (channel_name, operation))
            })
            .collect()
    }
    pub fn get_publish_channels(&self) -> Vec<(&String, &Operation)> {
        self.channels
            .iter()
            .filter_map(|(channel_name, channel)| {
                channel
                    .publish
                    .as_ref()
                    .map(|operation| (channel_name, operation))
            })
            .collect()
    }
}
