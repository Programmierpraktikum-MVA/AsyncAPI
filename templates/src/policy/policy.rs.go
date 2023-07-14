use anyhow::{anyhow, Result};
use opa_wasm::Runtime;
use reqwest::{self, Body, Client, IntoUrl, Response};
use serde::Serialize;
use std::env;
use wasmtime::{Config, Engine, Module, Store};

pub async fn opa_eval<I>(input: &I) -> Result<serde_json::Value>
where
    I: Serialize,
{
    if let Ok(enabled) = env::var("OPA_ENABLED") {
        let enabled: bool = enabled.parse().unwrap();
        if enabled == false {
            return Ok(serde_json::to_value(true).unwrap());
        }
    }
    if let Ok(url) = env::var("OPA_REMOTE_URL") {
        let url: String = url.parse().unwrap();
        return opa_eval_remote(url, serde_json::to_string(&input)?).await;
    }
    if let Ok(path) = env::var("OPA_LOCAL_WASM_PATH") {
        let path: String = path.parse().unwrap();
        return opa_eval_wasm(tokio::fs::read(path).await.unwrap(), input, "").await;
    }

    return Err(anyhow!("No OPA method provided"));
}

pub async fn opa_eval_remote<I>(url: impl IntoUrl, input: I) -> Result<serde_json::Value>
where
    Body: From<I>,
{
    let client = Client::new();
    let response: Response = client
        .post(url)
        .header("Content-Type", "application/json")
        .body(input)
        .send()
        .await?;
    println!("Response status: {}", response.status());

    let response_body = response.bytes().await?;
    println!("Response body: {:?}", response_body);

    Ok(serde_json::from_slice(response_body.as_ref())?)
}

pub async fn opa_eval_wasm(
    policy: impl AsRef<[u8]>,
    input: impl serde::Serialize,
    data: &str,
) -> Result<serde_json::Value> {
    // Configure the WASM runtime
    let mut config = Config::new();
    config.async_support(true);

    let engine = Engine::new(&config)?;

    // Load the policy WASM module
    let module = Module::new(&engine, policy)?;

    // Create a store which will hold the module instance
    let mut store = Store::new(&engine, ());

    // Instantiate the module
    let runtime = Runtime::new(&mut store, &module).await?;

    let policy = runtime.with_data(&mut store, &data).await?;

    // Evaluate the policy
    let response: serde_json::Value = policy.evaluate(&mut store, data, &input).await?;

    println!("{}", response);
    Ok(response)
}
