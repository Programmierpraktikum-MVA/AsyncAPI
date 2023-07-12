use jsonschema::JSONSchema;

pub fn validate_message_schema(
    validator_path: &std::path::Path,
    instance: &serde_json::Value,
) -> Result<(), String> {
    match std::env::var("SCHEMA_VALIDATION_ENABLED") {
        Ok(enabled) => {
            if enabled == "false" {
                return Ok(());
            }
        }
        Err(_) => return Ok(()),
    };
    // read json schema file as json value
    let schema_source = match std::fs::read(validator_path){
        Ok(schema) => schema,
        Err(_) => return Err("❌ Failed to read schema file in path ".to_string() + validator_path.to_str().unwrap()),
    };
    let schema = match serde_json::from_slice::<serde_json::Value>(&schema_source){
        Ok(schema) => schema,
        Err(_) => return Err("❌ Failed to parse schema file in path ".to_string() + validator_path.to_str().unwrap()),
    };
    let compiled = match JSONSchema::compile(&schema){
        Ok(compiled) => compiled,
        Err(_) => return Err("❌ Failed to compile schema in path".to_string() + validator_path.to_str().unwrap()),
    };
    let result = compiled.validate(instance);
    if let Err(errors) = result {
        Err("❌ Message payload invalid!, errors: ".to_string() + &errors.map(|e| e.to_string()).collect::<Vec<String>>().join(", "))
    } else {
        Ok(())
    }
}
