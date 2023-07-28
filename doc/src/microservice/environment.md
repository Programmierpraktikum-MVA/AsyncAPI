# Environment Variables

An `.env`-file is automatically generated from the [.env template](https://github.com/Programmierpraktikum-MVA/AsyncAPI/blob/d05d047c5ea9dfb221f31ecbf5af03387103e342/templates/.env.go)
If you want to extend the .env file feel free to do so in the generated code
- or if you want to customize the generated .enf file before it is generated take a look at [writing your own templates](../generator/templates.md)
The generated microservice uses the following environment variables (with their respective default values):
```json
SERVICE_PORT = "8080"
SERVER_URL = "{{ .server.url }}"
LOG_LEVEL = "DEBUG"     # available levels are ERROR, WARN, INFO, DEBUG and TRACE
OPA_RULES= "path/to/admin/policy"
TRACING_ENABLED = false
SCHEMA_VALIDATION_ENABLED = true
```

Also per channel the subject will be set via an environment variable:
```json
{channel_name}_SUBJECT = "{subject}"    # for normal pub_sub channels
{channel_name}_QUEUE = "{subject}"      # for nats queue channels
{channel_name}_STREAM = "{subject}"     # for nats jetstream channels
```

And for OPA
```json
OPA_ENABLED = false                 # choose if OPA should be enabled
#OPA_REMOTE_URL = "localhost:4042"  # pick the url for an opa server
#OPA_LOCAL_WASM_PATH = "some/path"  # pick the path of a to wasm compiled rego file 
```
- for more information see [Working with Open Policy Agent](./opa.md)
