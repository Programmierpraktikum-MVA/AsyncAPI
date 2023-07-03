use crate::utils;
use crate::{generator::template_functions::TEMPLATE_FUNCTIONS, Templates};
use gtmpl::Context;
use std::path::PathBuf;

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
    let render = match render_template(
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
