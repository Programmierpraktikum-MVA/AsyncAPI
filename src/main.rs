mod asyncapi_model;
mod cli;
mod generator;
mod parser;
mod template_context;
mod utils;

use crate::{
    asyncapi_model::AsyncAPI,
    generator::{cargo_fix, cargo_generate_rustdoc, generate_models_folder, template_render_write},
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

    template_render_write(
        &template_path.join("main.go"),
        &async_config,
        &output_path.join("src/main.rs"),
    );
    template_render_write(
        &template_path.join("handler.go"),
        &async_config,
        &output_path.join("src/handler.rs"),
    );

    template_render_write(
        &template_path.join("Readme.md"),
        &async_config,
        &output_path.join("Readme.md"),
    );

    template_render_write(
        &template_path.join("utils/mod.go"),
        &async_config,
        &output_path.join("src/utils/mod.rs"),
    );

    template_render_write(
        &template_path.join("utils/streams.go"),
        &async_config,
        &output_path.join("src/utils/streams.rs"),
    );
    template_render_write(
        &template_path.join("utils/common.go"),
        &async_config,
        &output_path.join("src/utils/common.rs"),
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
