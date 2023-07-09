use crate::{generator::template_functions::TEMPLATE_FUNCTIONS, Templates};
use crate::{template_context::TemplateContext, utils};
use gtmpl::Context;
use rust_embed::EmbeddedFile;
use std::path::Path;

/// runs cargo command with options
/// Example: ` cargo_command!("init","--bin","path"); `
#[macro_export]
macro_rules! cargo_command {
    ( $( $x:expr ),* ) => {
        {
            let mut command = Command::new("cargo");
            $(
                command.arg($x);
            )*
            command.output().expect("failed cargo_command")
        }
    };
}
/// checks if project with name already exists, if yes asks for permission to overwrite
pub fn check_for_overwrite(output_path: &Path, project_title: &str) {
    if output_path.exists() {
        println!("A project with the name {} already exists in the current directory, do you want to overwrite the existing project? \nWARNING: This will delete all files in the directory and all applied. \nType 'y' to continue or anything else to exit.", project_title);
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {
                if input.trim() != "y" {
                    println!("Aborting generation...");
                    std::process::exit(0);
                }
                std::fs::remove_dir_all(output_path).unwrap();
            }
            Err(err) => {
                println!("❌ Error reading input: {}", err);
                std::process::exit(1);
            }
        }
    }
}

/// takes an embedded `template_path`, renders it with context reference and writes to output file
pub fn embedded_template_render_write(
    template_path: &str,
    context_ref: impl Into<gtmpl::Value>,
    output_path: &Path,
) {
    let template: EmbeddedFile = match Templates::get(template_path) {
        Some(template) => template,
        None => {
            eprintln!("❌ Error reading template");
            std::process::exit(1);
        }
    };
    let template = std::str::from_utf8(template.data.as_ref()).unwrap();
    template_render_write(template, context_ref, output_path)
}

/// takes a `template`, renders it with context reference and writes to output file
pub fn template_render_write(
    template: impl Into<String>,
    context_ref: impl Into<gtmpl::Value>,
    output_path: &Path,
) {
    let mut render = match render_template(template, context_ref, TEMPLATE_FUNCTIONS) {
        Ok(render) => render,
        Err(e) => {
            eprintln!("❌ Error rendering template: {}", e);
            std::process::exit(1);
        }
    };
    if output_path.ends_with(".env") {
        let mut lines: Vec<&str> = render.split('\n').collect();
        lines.retain(|&x| x.trim() != "");
        render = lines.join("\n");
    }

    match utils::write_to_path_create_dir(&render, output_path) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("❌ Error writing template: {}", e);
            std::process::exit(1);
        }
    }
}

/// parses templates, adds funcs so they can be executed from inside the template and renders templatey
/// just like `gtmpl::render` but supports adding template functions
fn render_template<T: Into<String>, C: Into<gtmpl::Value>, F: Into<String> + Clone>(
    template_str: T,
    context: C,
    template_functions: &[(F, gtmpl::Func)],
) -> Result<String, gtmpl::TemplateError> {
    let mut tmpl = gtmpl::Template::default();
    tmpl.add_funcs(template_functions);
    tmpl.parse(template_str)?;
    tmpl.render(&Context::from(context)).map_err(Into::into)
}
/// renders and writes all templates in `template_file_paths` to `output_path`
/// if file has `.go` extension it will be changed to `.rs`
pub fn write_multiple_embedded_templates<'a>(
    context_ref: &TemplateContext,
    output_path: &Path,
    template_file_paths: impl Iterator<Item = &'a str>,
) {
    for template_file_path in template_file_paths {
        if template_file_path.ends_with(".go") {
            embedded_template_render_write(
                template_file_path,
                context_ref,
                &output_path.join(template_file_path).with_extension("rs"),
            );
        } else {
            embedded_template_render_write(
                template_file_path,
                context_ref,
                &output_path.join(template_file_path),
            );
        }
    }
}
