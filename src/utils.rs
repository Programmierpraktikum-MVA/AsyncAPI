use crate::Templates;
use std::{
    fs::{self, File},
    io::{Error, Write},
    path::Path,
};

/// writes a file and recursivly creates the directory it is in
pub fn write_to_path_create_dir(content: &str, path: &Path) -> Result<(), Error> {
    fs::create_dir_all(path.parent().unwrap())?;
    let mut out_file = File::create(path)?;
    out_file.write_all(content.as_bytes())
}

impl Templates {
    /// like `Templates::get(file_name)` but returns `Option<String>`
    pub fn get_str(file_path: &str) -> Option<String> {
        let file = match Self::get(file_path) {
            Some(file) => file,
            None => return None,
        };

        let result = match std::str::from_utf8(file.data.as_ref()) {
            Ok(file) => file,
            Err(_) => return None,
        };
        Some(result.to_string())
    }
}
