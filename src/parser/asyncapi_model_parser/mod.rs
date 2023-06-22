use std::path::Path;

use crate::asyncapi_model::AsyncAPI;
use crate::parser::common::read_json_or_yaml_to_value;
mod common;
mod preprocessor;
mod validator;

pub fn parse_spec_to_model(specs_dir: &Path) -> Result<AsyncAPI, Box<dyn std::error::Error>> {
    let spec = read_json_or_yaml_to_value(specs_dir);
    let version = common::parse_spec_version(&spec);

    // Construct the URL of the validator schema for the parsed version.
    let versioned_validator_schema_url =
        format!("https://asyncapi.com/definitions/{}.json", version);

    // Download the versioned validator schema.
    let validator = common::download_validator_schema(&versioned_validator_schema_url)?;

    // Validate the spec against the versioned validator schema.
    validator::validate_asyncapi_schema(&validator, &spec);

    let preprocessed_spec = preprocessor::preprocess_schema(spec);
    let spec = serde_json::from_value::<AsyncAPI>(preprocessed_spec)?;

    Ok(spec)
}
