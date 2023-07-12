[package]
name = "{{ to_lower (replace .title " " "_") }}"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
async-nats = "0.29.0"
futures = "0.3.28"
serde = "1.0.164"
serde_json = "1.0.97"
tokio = { version = "1.28.2", features = ["full"] }
dotenv = "0.15.0"
clap = {version = "4.3.0", features = ["derive"]}
opentelemetry = { version = "*", features = ["rt-tokio"] }
opentelemetry-jaeger = { version = "*", features = ["rt-tokio", "isahc_collector_client"] }
log = "0.4.0"
env_logger = "0.10.0"
anyhow = "1.0.71"
reqwest = "0.11.18"
wasmtime = "9.0.3"
opa-wasm = { git = "https://github.com/matrix-org/rust-opa-wasm.git" }
cargo_metadata = "0.15.4"
warp = "0.3.5"
lazy_static = "1.4"
jsonschema = "0.17.0"

