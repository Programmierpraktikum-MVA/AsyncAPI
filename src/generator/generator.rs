

use std::{
    fs::File,
    io::{Error, Write},
    path::{Path, PathBuf},
    process::{Command, Output},
};

pub fn write_to_file(content: &str, path: &PathBuf) -> Result<(), Error> {
    let mut out_file = File::create(path)?;
    out_file.write_all(content.as_bytes())
}

pub fn cargo_init_project(path: &PathBuf) -> Output {
    Command::new("cargo")
        .arg("init")
        .arg("--bin")
        .arg(path)
        .output()
        .expect("failed create new cargo project")
}

pub fn cargo_fmt(path: &PathBuf) -> Output {
    Command::new("cargo")
        .arg("fmt")
        .arg("--")
        .arg(path)
        .output()
        .expect("failed to format")
}

pub fn cargo_add(path: &Path, crate_name: &str) {
    Command::new("cargo")
        .arg("add")
        .arg(crate_name)
        .arg(String::from("--manifest-path=") + path.to_str().unwrap() + "/Cargo.toml")
        .output()
        .expect("failed to add crate");
}