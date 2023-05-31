use jsonschema::JSONSchema;

pub fn validate_asyncapi_schema(validator: &serde_json::Value, instance: &serde_json::Value) {
    let compiled = JSONSchema::compile(validator).expect("A valid schema");
    let result = compiled.validate(instance);
    if let Err(errors) = result {
        for error in errors {
            println!("Validation error: {}", error);
            println!("Instance path: {}", error.instance_path);
        }
        panic!("Validation failed");
    } else {
        println!("Validation succeeded");
    }
}
