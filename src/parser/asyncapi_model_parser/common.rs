use reqwest::blocking::get;

pub fn download_validator_schema(url: &str) -> Result<serde_json::Value, reqwest::Error> {
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

pub fn parse_spec_version(spec: &serde_json::Value) -> String {
    let version = spec["asyncapi"].as_str().unwrap();
    version.to_string()
}
