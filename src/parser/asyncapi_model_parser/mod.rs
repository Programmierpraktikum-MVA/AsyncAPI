use std::path::Path;

use crate::asyncapi_model::AsyncAPI;
use crate::parser::common;
mod preprocessor;
mod validator;

pub fn parse_spec_to_model(
    spec_path: &Path,
    validator_schema_path: &Path,
) -> Result<AsyncAPI, serde_json::Error> {
    //read the specification file
    let spec = common::read_json_or_yaml_to_value(spec_path);
    //read the validator json schema file
    let validator = common::read_json_or_yaml_to_value(validator_schema_path);
    //use the validator to validate the specification
    validator::validate_asyncapi_schema(&validator, &spec);

    // apply preprocessing (add missing names, resolve refs)
    let preprocessed_spec = preprocessor::preprocess_schema(spec);

    //finally read the preprocessed spec into the model
    let spec = serde_json::from_value::<AsyncAPI>(preprocessed_spec)?;
    Ok(spec)
}
