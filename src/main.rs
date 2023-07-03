mod asyncapi_model;
mod cli;
mod generator;
mod parser;
mod template_context;
mod utils;

use crate::{
    asyncapi_model::AsyncAPI,
    generator::{
        cargo_fix, cargo_generate_rustdoc, check_for_overwrite, generate_models_folder,
        write_multiple_templates,
    },
    utils::append_file_to_file,
};

use clap::Parser;
use generator::{cargo_fmt, cargo_init_project};
use std::path::Path;

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

    check_for_overwrite(output_path, title);

    write_multiple_templates(
        template_path,
        &async_config,
        output_path,
        &[
            "src/main.go",
            "src/handler.go",
            "src/cli.go",
            "Readme.md",
            ".env",
            "src/utils/mod.go",
            "src/utils/streams.go",
            "src/utils/common.go",
            "src/config/mod.go",
        ],
    );

    generate_models_folder(&async_config, template_path, output_path);
    println!("🚀 File generation finished, adding dependencies...");

    // make output a compilable project
    cargo_init_project(output_path);

    cargo_fmt(output_path.join("src/main.rs"));
    // add dependencies
    append_file_to_file(
        template_path.join("dependencies.toml"),
        output_path.join("Cargo.toml"),
    )
    .unwrap();

    cargo_fix(output_path);

    if args.doc {
        println!("📚 Generating docs...");
        cargo_generate_rustdoc(output_path);
    }
}
