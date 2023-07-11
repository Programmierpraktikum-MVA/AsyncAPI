# {{.title}}
{{if .description}}
{{.description}}
{{end}}

## Documenation
Open the documentation with the following command:
``` 
    cargo doc --no-deps --open
```

### Testing
You can use a cli command to send a message directly on a specified channel for testing purposes. Simply use the following command in the root directory of the generated project:

```
cargo run -- -c destination/channel -m '{"test": "message"}'
```
When manually sending messages, please use the property names as they are defined in the specification.
Note, to run a second server please change the env variable `SERVICE_PORT` to a different port number.

## Tracing
The generated microservice uses OpenTelemetry for tracing. Each handler function is wrapped in a span, which can be modified to fit your tracing needs. 

Enable the tracer in the `.env` file by setting `TRACING_ENABLED = true`.

The default exporter is the Jaeger exporter. The default configuration is set to export to a Jaeger instance running on `localhost:6831`.

Jaeger can be started in Docker using the following command:
```
docker run -d -p6831:6831/udp -p6832:6832/udp -p16686:16686 -p14268:14268 jaegertracing/all-in-one:latest
```

Access the Jaeger UI at http://localhost:16686 and look for your service name in the dropdown menu.

For more information, visit the [Jaeger website](https://www.jaegertracing.io/docs/getting-started/).

## Validation
The generated microservice uses json schemas for validating the message payload. The schema is the one defined in the specification. Settings like minimum etc. which are supported by json schema can be added there.
The schemas are located in the `schemas` folder. The schema is validated against the message payload in the handler function, you can turn this validation off by changing the SCHEMA_VALIDATION_ENABLED env variable to false.

Warning: Message validation currently has a high performance cost, so it is recommended to only use it in development. in production the schemas in the generated schema folder could be used to feed a schema registry, which can be used to validate the messages. [asyncapi doc](https://www.asyncapi.com/docs/guides/message-validation#schema-registry-validation)

