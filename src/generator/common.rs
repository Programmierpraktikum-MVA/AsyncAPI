use crate::template_context::SimplifiedOperation;
use crate::utils::write_to_path_create_dir;
use crate::{generator::template_functions::TEMPLATE_FUNCTIONS, Templates};
use crate::{template_context::TemplateContext, utils};
use gtmpl::Context;
use std::fs::read_to_string;
use std::io::{self, Error};
use std::path::Path;
use walkdir::WalkDir;

use super::{generate_models_folder, generate_schemas_folder};

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
        println!("\nA project with the name {} already exists in the current directory: {}. Do you want to overwrite it? \n\n❗ WARNING: Existing files within the folder will be permanently replaced by newly generated files. \n\nType 'y' to continue or anything else to exit.", project_title, output_path.to_string_lossy());
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {
                if input.trim() != "y" {
                    println!("Aborting generation...");
                    std::process::exit(0);
                }
            }
            Err(err) => {
                println!("❌ Error reading input: {}", err);
                std::process::exit(1);
            }
        }
    }
}

/// takes a `template` as **NOT THE template PATH**, renders it with context reference and writes to output file
pub fn render_write_template(
    template: impl Into<String>,
    context: impl Into<gtmpl::Value>,
    output_path: &Path,
) {
    let mut render = match render_template(template, context, TEMPLATE_FUNCTIONS) {
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

pub fn render_write_all_fs_templates(
    template_dir: &Path,
    context: &TemplateContext,
    output_path: &Path,
) {
    for template_dir_entry in WalkDir::new(template_dir)
        .into_iter()
        .filter_map(|x| x.ok())
    {
        let template_path = &template_dir_entry.path();
        if template_path.is_file() {
            let template = read_to_string(template_path).unwrap();
            render_write_dependant(
                template_path.strip_prefix(template_dir).unwrap(),
                template,
                context,
                output_path,
            );
        }
    }
}

/// renders and writes all templates in `template_file_paths` to `output_path`/`template_file_path`
/// if file has `.go` extension it will be changed to `.rs`
pub fn render_write_all_embedded_templates(context: &TemplateContext, output_path: &Path) {
    for template_path in Templates::iter() {
        if let Some(template) = Templates::get_str(&template_path) {
            let template_path = template_path.as_ref();
            render_write_dependant(Path::new(template_path), template, context, output_path);
        }
    }
}

/// renders and writes template and checks if it should be rendered seperatly or not
fn render_write_dependant(
    template_path: &Path,
    template: impl Into<String> + Clone,
    context: &TemplateContext,
    output_path: &Path,
) {
    let seperated_files = separate_files(
        template_path,
        template.clone(),
        context,
        output_path
            .join(template_path)
            .parent()
            .unwrap_or(Path::new("")),
    );
    if seperated_files.unwrap() {
        return;
    }
    render_write_template(
        template,
        context,
        &output_path.join(template_path).with_extension(""),
    );
}

/// checks if files should be seperatly rendered
/// if yes -> renders them
/// else returns false
fn separate_files(
    template_path: &Path,
    template_str: impl Into<String> + Clone,
    context: &TemplateContext,
    output_dir: &Path,
) -> Result<bool, Error> {
    let template_path = template_path.to_str().unwrap();
    if template_path.contains("$$handler$$") {
        render_write_separate_handler(&context.subscribe_channels, template_str, output_dir)?;
        return Ok(true);
    }
    if template_path.contains("$$producer$$") {
        render_write_separate_handler(&context.publish_channels, template_str, output_dir)?;
        return Ok(true);
    }
    if template_path.contains("$$model$$") {
        generate_models_folder(template_str, context, output_dir);
        return Ok(true);
    }
    if template_path.contains("$$schemas$$") {
        generate_schemas_folder(template_str, context, output_dir);
        return Ok(true);
    }
    Ok(false)
}

/// renders `template_str` and writes it to different file for each `contexts` iteration
fn render_write_separate_handler(
    contexts: &Vec<(&String, SimplifiedOperation)>,
    template_str: impl Into<String> + Clone,
    output_dir: &Path,
) -> Result<(), Error> {
    // render separate files
    for (_, context) in contexts {
        let output_path = output_dir
            .join(context.unique_id.clone())
            .with_extension("rs");
        let render = render_template(template_str.clone(), context, TEMPLATE_FUNCTIONS)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        write_to_path_create_dir(&render, &output_path)?;
    }
    Ok(())
}
