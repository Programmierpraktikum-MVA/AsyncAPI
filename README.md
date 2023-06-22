# Crustagen

Crustagen is a Rust code generator that takes AsyncAPI specifications as input and generates Rust code along with all the required dependencies and auto-generated documentation. It's designed to be an efficient tool to convert AsyncAPI specs into Rust projects ready to build and run.

![Build Status](https://github.com/Programmierpraktikum-MVA/AsyncAPI/actions/workflows/ci.yml/badge.svg)

## Table of Contents

- [Crustagen](#crustagen)
  - [Table of Contents](#table-of-contents)
  - [Requirements](#requirements)
  - [Installation](#installation)
  - [Usage](#usage)
  - [Contribute](#contribute)
  - [License](#license)

## Requirements

- Rust 1.70 or later.
- Cargo (comes with Rust).
- AsyncAPI Specification (version 2.1.0 supported currently).

## Installation

Clone and build the Crustagen project:

```sh
git clone https://github.com/yourusername/crustagen.git
cd crustagen
just install # Alternatively, you can use 'cargo build --release'
```

## Usage

To generate Rust code from an AsyncAPI specification, use the following `just` command:

```sh
just run-generator specfile_path="./example/specs/basic.yaml" output="./output" # Alternatively, you can use 'cargo run -- -s ./example/specs/basic.yaml -o ./output'
```

This will generate a Rust project in the specified output directory.

To run the server, navigate to the output directory (replace `{project-id}` with the actual project directory name, the title of the spec) and use the `just` command:

```sh
just start-service service_name={project-id} # Alternatively, you can use 'cd output/{project-id} && cargo run'
```

To view the auto-generated documentation, use the following command:

```sh
just generate-service-docs service_name={project-id} # Alternatively, you can use 'cd output/{project-id} && cargo doc --open'
```

Remember to replace `{project-id}` with the actual project directory name.

## Limitations

- only json payloads are currently supported for automatic deserialization
- only one server is currently supported and only nats protocol is supported
- only one message is currently supported per channel, payloads can be choosen freely including anyOf/oneOf/allOf

## Contribute

Contributions are always welcome! Please read the [contribution guidelines](CONTRIBUTING.md) first.

## License

This project is licensed under the MIT License. See the `LICENSE` file for more information.
