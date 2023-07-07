mod asyncapi_model;
mod cli;
mod generator;
mod parser;
mod template_context;
mod utils;

use crate::{
    asyncapi_model::AsyncAPI,
    generator::{check_for_overwrite, generate_models_folder, render_write_all_embedded_templates},
    utils::append_file_to_file,
};
use clap::Parser;
use rust_embed::RustEmbed;
use std::{path::Path, process::Command};

#[derive(RustEmbed)]
#[folder = "./templates"]
struct Templates;

impl Templates {
    pub fn get_str(file_path: &str) -> Option<String> {
        let file = match Self::get(file_path) {
            Some(file) => file,
            None => return None,
        };

        let result = match std::str::from_utf8(file.data.as_ref()) {
            Ok(file) => file,
            Err(_) => return None,
        };
        Some(result.to_string())
    }
}

fn main() {
    let args = cli::Args::parse();

    let specfile_path = Path::new(&args.specification);
    println!("📄 Using specification file {:?}", specfile_path);

    let template_path = Path::new("./templates/");

    let spec: AsyncAPI = match parser::asyncapi_model_parser::parse_spec_to_model(specfile_path) {
        Ok(spec) => {
            println!("🎉 Specification was parsed successfully!");
            spec
        }
        Err(e) => {
            eprintln!("❌ Error parsing the specification: {}", e);
            std::process::exit(1);
        }
    };

    let title: &str = match &args.title {
        Some(t) => t,
        None => &spec.info.title,
    };
    let output = args.output;
    let output_path = &Path::new(&output).join(title.replace(' ', "_").to_lowercase());
    println!("📂 Output path: {:?}", output_path);

    let async_config = match template_context::create_template_context(&spec) {
        Ok(async_config) => async_config,
        Err(e) => {
            eprintln!("❌ Error parsing the specification: {}", e);
            std::process::exit(1);
        }
    };

    //check_for_overwrite(output_path, title);

    render_write_all_embedded_templates(&async_config, output_path);

    println!("🚀 File generation finished, adding dependencies...");

    // make output a compilable project in output_path
    cargo_command!("init", "--bin", output_path);
    // runs cargo format on path
    cargo_command!("fmt", "--", output_path.join("src/main.rs"));

    // add dependencies
    // append_file_to_file(
    //     template_path.join("dependencies.toml"),
    //     output_path.join("Cargo.toml"),
    // )
    // .unwrap();
    // cargo fix, mostly for cleaning unused imports
    cargo_command!("fix", "--bin", output_path, "--allow-dirty");

    if args.doc {
        println!("📚 Generating docs...");
        cargo_command!(output_path, "doc", "--no-deps");
    }
}
