use std::{
    fs::{self, File},
    io::{Error, Write},
    path::PathBuf,
};
pub fn write_to_file(content: &str, path: &PathBuf) -> Result<(), Error> {
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)?;
        }
    }

    let mut out_file = File::create(path)?;
    out_file.write_all(content.as_bytes())
}
