mod asyncapi_model;
mod cli;
mod generator;
mod parser;
mod template_model;
mod utils;

use generator::{cargo_add, cargo_fmt, cargo_init_project};

use quicli::prelude::*;
use std::path::Path;
use structopt::StructOpt;

use crate::generator::template_render_write;

fn main() -> CliResult {
    let args = cli::Cli::from_args();
    // specfile_path
    let specfile_path = Path::new(&args.specification_file);
    println!("specfile_path: {:?}", specfile_path);

    let template_path = Path::new("./templates/");

    let spec = parser::parse_asyncapi_yaml_file(specfile_path).unwrap();
    println!("{:?}", spec);

    let title = match args.project_title {
        Some(t) => t,
        None => spec.info.title.clone(),
    };
    // output_path
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
    // make output a compilable project
    cargo_init_project(output_path);
    cargo_fmt(&output_path.join("src/main.rs"));
    cargo_add(output_path, "tokio", Some("rt-multi-thread")); // when there are more crates move to generator.rs
    cargo_add(output_path, "async_nats", None);
    cargo_add(output_path, "futures", None);
    cargo_add(output_path, "serde", None);

    Ok(())
}
