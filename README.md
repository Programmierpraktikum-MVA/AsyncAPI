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

```sh
git clone https://github.com/yourusername/crustagen.git
cd crustagen
cargo build --release
```

## Usage

To generate Rust code from an AsyncAPI specification, use the `-s` flag to specify the path to the spec file and the `-o` flag to specify the output directory.

Example:

```sh
cargo run -- -s ./example/specs/basic.yaml -o ./output
```

This will generate a Rust project in the specified output directory. 

To run the server, navigate to the output directory (replace `{project-id}` with the actual project directory name, the title of the spec) and use `cargo run`:

```sh
cd output/{project-id}
cargo run
```

To view the auto-generated documentation, use the following command:

```sh
cargo doc --no-deps --open
```

## Contribute

Contributions are always welcome! Please read the [contribution guidelines](CONTRIBUTING.md) first.

## License

[MIT](LICENSE)
