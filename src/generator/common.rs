use crate::generator::template_functions::TEMPLATE_FUNCTIONS;
use crate::utils;
use gtmpl::Context;
use std::{
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
};

/// initialize a cargo project in path
pub fn cargo_init_project(path: impl AsRef<OsStr>) -> Output {
    Command::new("cargo")
        .arg("init")
        .arg("--bin")
        .arg(path)
        .output()
        .expect("failed create new cargo project")
}
/// runs cargo format on path
pub fn cargo_fmt(path: impl AsRef<OsStr>) -> Output {
    Command::new("cargo")
        .arg("fmt")
        .arg("--")
        .arg(path)
        .output()
        .expect("failed to format")
}

/// cargo fix, mostly for cleaning unused imports
pub fn cargo_fix(path: &PathBuf) -> Output {
    Command::new("cargo")
        .arg("fix")
        .arg("--bin")
        .arg(path)
        .arg("--allow-dirty")
        .output()
        .expect("failed to cargo fix")
}

/// reads template from path renders it with context reference and writes to output file
pub fn template_render_write(
    template_path: &PathBuf,
    context_ref: impl Into<gtmpl::Value>,
    output_path: &PathBuf,
) {
    let template = match fs::read_to_string(template_path) {
        Ok(template) => template,
        Err(e) => {
            eprintln!("❌ Error reading template: {}", e);
            std::process::exit(1);
        }
    };
    let render = match render_template(&template, context_ref, TEMPLATE_FUNCTIONS) {
        Ok(render) => render,
        Err(e) => {
            eprintln!("❌ Error rendering template: {}", e);
            std::process::exit(1);
        }
    };
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
fn render_template<C: Into<gtmpl::Value>, F: Into<String> + Clone>(
    template_str: &str,
    context: C,
    template_functions: &[(F, gtmpl::Func)],
) -> Result<String, gtmpl::TemplateError> {
    let mut tmpl = gtmpl::Template::default();
    tmpl.add_funcs(template_functions);
    tmpl.parse(template_str)?;
    tmpl.render(&Context::from(context)).map_err(Into::into)
}

pub fn cargo_generate_rustdoc(path: &Path) {
    Command::new("cargo")
        .current_dir(path)
        .arg("doc")
        .arg("--no-deps")
        .output()
        .expect("failed to generate rustdoc");
}
