use crate::utils;
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

pub fn cargo_add(path: &Path, crate_name: &str, features: Option<&str>) {
    let mut command_builder = Command::new("cargo");
    command_builder.arg("add").arg(crate_name);
    if let Some(features) = features {
        command_builder.arg("--features").arg(features);
    }
    command_builder
        .arg(String::from("--manifest-path=") + path.to_str().unwrap() + "/Cargo.toml")
        .output()
        .expect("failed to add crate");
}
/// reads template from path renders it with context reference and writes to output file
pub fn template_render_write(
    template_path: &PathBuf,
    context_ref: impl Into<gtmpl::Value>,
    output_path: &PathBuf,
) {
    let template = fs::read_to_string(template_path).expect("file could not be read");
    let render = gtmpl::template(&template, context_ref).expect("Could not inject template");
    utils::write_to_path_create_dir(&render, output_path).unwrap();
}

pub fn cargo_generate_rustdoc(path: &Path) {
    Command::new("cargo")
        .current_dir(path)
        .arg("doc")
        .arg("--no-deps")
        .output()
        .expect("failed to generate rustdoc");
}
