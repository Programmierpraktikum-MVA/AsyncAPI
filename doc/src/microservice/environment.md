# Environment Variables

- An `.env`-file is automatically generated from the [.env template](https://github.com/Programmierpraktikum-MVA/AsyncAPI/blob/d05d047c5ea9dfb221f31ecbf5af03387103e342/templates/.env.go)
- The generated microservice uses the following environment variables (with their respective default values):
```rust
SERVICE_PORT = "8080"
SERVER_URL = "{{ .server.url }}"
LOG_LEVEL = "DEBUG"
OPA_RULES= "path/to/admin/policy"
TRACING_ENABLED = false
SCHEMA_VALIDATION_ENABLED = true
```
TODO: the rest
