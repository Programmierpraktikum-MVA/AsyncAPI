# Crustagen

Crustagen is a Rust code generator that takes AsyncAPI specifications as input and generates Rust code along with all the required dependencies and auto-generated documentation. It's designed to be an efficient tool to convert AsyncAPI specs into Rust projects ready to build and run.

![Build Status](https://github.com/Programmierpraktikum-MVA/AsyncAPI/actions/workflows/ci.yml/badge.svg)

## Table of Contents

- [Crustagen](#crustagen)
  - [Table of Contents](#table-of-contents)
  - [Requirements](#requirements)
  - [Installation](#installation)
  - [Usage](#usage)
  - [Limitations](#limitations)
  - [Contribute](#contribute)
  - [License](#license)

## Requirements

- Rust 1.70 or later.
- Cargo (comes with Rust).

## Installation

Clone and build the Crustagen project:

```sh
git clone https://github.com/Programmierpraktikum-MVA/AsyncAPI.git
cd AsyncAPI
just install # Alternatively, you can use 'cargo build --release'
```

## Usage

To generate Rust code from an AsyncAPI specification, use the following `just` command:

```sh
just run example/specs/basic.yaml output # Alternatively, you can use 'cargo run -- -s ./example/specs/basic.yaml -o ./output'
```

This will generate a Rust project in the specified output directory.

To run the server, navigate to the output directory (replace `{project-id}` with the actual project directory name, the title of the spec) and use the `just` command:

```sh
just start-service {project-id} # Alternatively, you can use 'cd output/{project-id} && cargo run'
```

To view the auto-generated documentation, use the following command:

```sh
just service-doc {project-id} # Alternatively, you can use 'cd output/{project-id} && cargo doc --open'
```

Remember to replace `{project-id}` with the name of your generated microservice (`title` field from the provided spec).

## Types
Rust types will be generated in the models folder according to the given payload json schema definitions. Names will be generated according to channels etc, if you want to share a payload type between two messages, make sure to use the same "name" property in the payload. Warning: This will not check if the types of those payloads are actually the same, so make sure to use the same schema or better even, simply a ref to the schema with the name.

## Limitations

- Only json payloads are currently supported for automatic deserialization
- Only one server is currently supported and only nats protocol is supported
- Only one message is currently supported per channel, payloads can be choosen freely including anyOf/oneOf/allOf
- The generated rust types are required by default, if you want to use optional types, please modify the generated types after generation or use oneOf/anyOf/allOf to represent optional types
- references in the specification are only suppported inside the same file, external references are not supported

## Contribute

Contributions are always welcome! Please read the [contribution guidelines](CONTRIBUTING.md) first.

## License

This project is licensed under the MIT License. See the `LICENSE` file for more information.
