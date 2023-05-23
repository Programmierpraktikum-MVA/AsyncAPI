use std::{
    fs::{self, File},
    io::{Error, Write},
    path::PathBuf,
};
pub fn write_to_path_create_dir(content: &str, path: &PathBuf) -> Result<(), Error> {
    fs::create_dir_all(path.parent().unwrap())?;
    let mut out_file = File::create(path)?;
    out_file.write_all(content.as_bytes())
}
