use std::{
    fs::{self, File, OpenOptions},
    io::{self, Error, Read, Write},
    path::Path,
};
/// writes a file and recursivly creates the directory it is in
pub fn write_to_path_create_dir(content: &str, path: &Path) -> Result<(), Error> {
    fs::create_dir_all(path.parent().unwrap())?;
    let mut out_file = File::create(path)?;
    out_file.write_all(content.as_bytes())
}

/// reads source file and appends its contents to destination file
pub fn append_file_to_file(
    source_path: impl AsRef<Path>,
    destination_path: impl AsRef<Path>,
) -> Result<(), io::Error> {
    let mut source_file = File::open(source_path)?;
    let mut destination_file = OpenOptions::new().append(true).open(destination_path)?;
    let mut buffer = Vec::new();
    source_file.read_to_end(&mut buffer)?;
    destination_file.write_all(&buffer)?;
    Ok(())
}
