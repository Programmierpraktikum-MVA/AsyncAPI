mod asyncapi_model;
mod cli;
mod generator;
mod parser;
mod template_model;
mod utils;

use crate::generator::{cargo_generate_rustdoc, template_render_write};
use clap::Parser;
use generator::{cargo_add, cargo_fmt, cargo_init_project};
use std::path::Path;

fn main() {
    let args = cli::Args::parse();

    let specfile_path = Path::new(&args.specification_file);
    println!("specfile_path: {:?}", specfile_path);

    let template_path = Path::new("./templates/");
    let validator_schema_path = Path::new("./validator_schema/2.1.0.json");

    let spec = parser::parse_spec_to_model(specfile_path, validator_schema_path).unwrap();
    println!("{:?}", spec);

    let title: &str = match &args.project_title {
        Some(t) => t,
        None => &spec.info.title,
    };
    let output = args.output_directory;
    let output_path = &Path::new(&output).join(title.replace(' ', "_").to_lowercase());
    println!("output_path: {:?}", output_path);

    let async_config = parser::spec_to_pubsub_template_type(&spec).unwrap();

    // render template and write
    template_render_write(
        &template_path.join("main.rs"),
        &async_config,
        &output_path.join("src/main.rs"),
    );
    template_render_write(
        &template_path.join("handler.rs"),
        &async_config,
        &output_path.join("src/handler.rs"),
    );
    template_render_write(
        &template_path.join("Readme.md"),
        &async_config,
        &output_path.join("Readme.md"),
    );
    println!("file generation finished, adding dependencies...");

    // make output a compilable project
    cargo_init_project(output_path);
    cargo_fmt(&output_path.join("src/main.rs"));
    cargo_add(output_path, "tokio", Some("rt-multi-thread")); // when there are more crates move to generator.rs
    cargo_add(output_path, "async_nats", None);
    cargo_add(output_path, "futures", None);
    cargo_add(output_path, "serde", None);
    println!("generating docs...");
    cargo_generate_rustdoc(output_path);
}
