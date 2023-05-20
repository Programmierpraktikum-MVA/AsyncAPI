use std::{
    fs::File,
    io::{Error, Write},
    path::PathBuf,
};

pub fn write_to_file(content: &str, path: &PathBuf) -> Result<(), Error> {
    let mut out_file = File::create(path)?;
    out_file.write_all(content.as_bytes())
}
