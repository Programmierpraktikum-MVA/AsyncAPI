mod asyncapi_model;
mod cli;
mod generator;
mod parser;
mod template_model;
mod utils;

use generator::{cargo_add, cargo_fmt, cargo_init_project};
use quicli::prelude::*;
use std::{
    fs::{self, create_dir_all},
    path::Path,
};
use structopt::StructOpt;

fn main() -> CliResult {
    let args = cli::Cli::from_args();

    let specfile_path = Path::new(&args.specification_file);
    println!("specfile_path: {:?}", specfile_path);

    let spec = parser::parse_spec_to_model(specfile_path).unwrap();
    println!("{:?}", spec);

    let title = match args.project_title {
        Some(t) => t,
        None => spec.info.title.clone(),
    };

    let output = args.output_directory;

    let output_path = &Path::new(&output).join(title.replace(' ', "_").to_lowercase());

    println!("output_path: {:?}", output_path);

    let template_path = Path::new("./templates/pubsub.tmpl");
    let template = fs::read_to_string(template_path).expect("file could not be read");
    let async_config = parser::spec_to_pubsub_template_type(spec).unwrap();

    let template_result =
        gtmpl::template(&template, async_config).expect("Could not inject template");

    create_dir_all(output_path.join("src")).unwrap();
    utils::write_to_file(&template_result, &output_path.join("src/main.rs")).unwrap();

    cargo_init_project(output_path);
    cargo_fmt(&output_path.join("src/main.rs"));
    cargo_add(output_path, "tokio", Some("rt-multi-thread")); // when there are more crates move to generator.rs
    cargo_add(output_path, "async_nats", None);
    cargo_add(output_path, "futures", None);
    cargo_add(output_path, "serde", None);

    Ok(())
}
