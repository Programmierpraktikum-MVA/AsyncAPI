use gtmpl::Context;

use crate::{template_context::TemplateContext, utils};
use std::{
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
    vec,
};

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

fn key_exists(args: &[gtmpl_value::Value]) -> Result<gtmpl_value::Value, gtmpl_value::FuncError> {
    if args.is_empty() {
        return Err(gtmpl_value::FuncError::AtLeastXArgs(
            "Need at least 1 arg for key exists".to_string(),
            1,
        ));
    }
    let map = args[0].clone();
    if args.len() == 1 {
        return Ok(gtmpl_value::Value::Bool(true));
    }
    let keys = args[1..].to_vec();
    // check if keys is empty
    let rest_keys: Vec<gtmpl_value::Value> = match keys.len() > 1 {
        false => vec![],
        _ => keys[1..].to_vec(),
    };

    // extract first key
    if !keys.is_empty() {
        let key = keys[0].clone();
        match key {
            gtmpl_value::Value::String(s) => {
                let res: Result<gtmpl_value::Value, gtmpl_value::FuncError> = match map {
                    gtmpl_value::Value::Object(o) => {
                        // call again with rest of keys
                        key_exists(
                            vec![vec![o.get(&s).unwrap().clone()], rest_keys]
                                .concat()
                                .as_slice(),
                        )
                    }
                    _ => Ok(gtmpl_value::Value::Bool(false)),
                };
                return res;
            }
            _ => {
                return Err(gtmpl_value::FuncError::Generic(
                    "keys need to be string".to_string(),
                ));
            }
        }
    }
    Ok(gtmpl::Value::Bool(true))
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
    let mut base_template = gtmpl::Template::default();
    base_template.add_func("key_exists", key_exists);
    base_template
        .parse(&template)
        .expect("failed to parse template");
    let render = match base_template.render(&Context::from(context_ref.into())) {
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

pub fn write_multiple_templates(
    template_path: &Path,
    context_ref: &TemplateContext,
    output_path: &Path,
    endings: &[&str],
) {
    for ending in endings {
        if ending.ends_with(".go") {
            template_render_write(
                &template_path.join(ending),
                context_ref,
                &output_path.join(ending).with_extension("rs"),
            );
        } else {
            template_render_write(
                &template_path.join(ending),
                context_ref,
                &output_path.join(ending),
            );
        }
    }
}

pub fn cargo_generate_rustdoc(path: &Path) {
    Command::new("cargo")
        .current_dir(path)
        .arg("doc")
        .arg("--no-deps")
        .output()
        .expect("failed to generate rustdoc");
}
