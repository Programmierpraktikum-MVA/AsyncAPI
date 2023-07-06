use crate::{generator::template_functions::TEMPLATE_FUNCTIONS, Templates};
use crate::{template_context::TemplateContext, utils};
use gtmpl::Context;
use std::path::{Path, PathBuf};

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

pub fn check_for_overwrite(output_path: &Path, project_title: &str) {
    //check if project with name already exists, if yes ask for permission to overwrite
    if output_path.exists() {
        let warn_message = format!("A project with the name {} already exists in the current directory, do you want to overwrite the existing project? \nWARNING: This will delete all files in the directory and all applied. \nType 'y' to continue or anything else to exit.",project_title);
        println!("{}", warn_message);
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

/// reads template from path renders it with context reference and writes to output file
pub fn template_render_write(
    template_path: &str,
    context_ref: impl Into<gtmpl::Value>,
    output_path: &PathBuf,
) {
    let template = match Templates::get(template_path) {
        Some(template) => template,
        None => {
            eprintln!("❌ Error reading template");
            std::process::exit(1);
        }
    };
    let template = template.data.as_ref();
    let mut render = match render_template(
        std::str::from_utf8(template).unwrap(),
        context_ref,
        TEMPLATE_FUNCTIONS,
    ) {
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

pub fn write_multiple_templates(
    context_ref: &TemplateContext,
    output_path: &Path,
    template_file_paths: &[&str],
) {
    for template_file_path in template_file_paths {
        if template_file_path.ends_with(".go") {
            template_render_write(
                template_file_path,
                context_ref,
                &output_path.join(template_file_path).with_extension("rs"),
            );
        } else {
            template_render_write(
                template_file_path,
                context_ref,
                &output_path.join(template_file_path),
            );
        }
    }
}