mod asyncapi_model;
mod cli;
mod generator;
mod parser;
mod template_context;
mod utils;

use crate::{
    asyncapi_model::AsyncAPI,
    generator::{
        check_for_overwrite, render_write_all_embedded_templates, render_write_all_fs_templates,
    },
};
use clap::Parser;
use rust_embed::RustEmbed;
use std::{path::Path, process::Command};

#[derive(RustEmbed)]
#[folder = "./templates"]
struct Templates;

fn main() {
    let args = cli::Args::parse();

    let specfile_path = Path::new(&args.specification);
    println!("📄 Using specification file {:?}", specfile_path);

    let template_dir = Path::new("./user_templates/");

    let spec: AsyncAPI = match parser::asyncapi_model_parser::parse_spec_to_model(specfile_path) {
        Ok(spec) => spec,
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
    // simplify async api spec to template context
    let async_config = match template_context::create_template_context(&spec) {
        Ok(async_config) => async_config,
        Err(e) => {
            eprintln!("❌ Error parsing the specification: {}", e);
            std::process::exit(1);
        }
    };
    check_for_overwrite(output_path, title);

    // make output a compilable project in output_path
    cargo_command!("init", "--bin", output_path);

    render_write_all_embedded_templates(&async_config, output_path);
    render_write_all_fs_templates(template_dir, &async_config, output_path);
    println!("🚀 File generation finished, formatting generated files...");

    // runs cargo format on path
    cargo_command!("fmt", "--", output_path.join("src/main.rs"));
    // cargo fix, mostly for cleaning unused imports
    cargo_command!(
        "fix",
        "--manifest-path",
        output_path.join("Cargo.toml"),
        "--allow-dirty"
    );

    if args.doc {
        println!("📚 Generating docs...");
        cargo_command!(output_path, "doc", "--no-deps");
    }

    println!(
        "🎉 Generation finished!\n\n   Run the service using:\n     cd {} && cargo run\n\n   If you are in the generator root, start the service using:\n     just start-service {}\n",
        output_path.to_string_lossy(), title.replace(' ', "_").to_lowercase()
    );
}
