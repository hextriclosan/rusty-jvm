use crate::error::{IoSnafu, JarError, Result};
use snafu::ResultExt;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use zip::ZipArchive;

/// Represents a JAR (Java ARchive) file, which is essentially a ZIP file with a specific structure.
#[derive(Debug)]
pub struct JarFile {
    zip_file: ZipArchive<BufReader<File>>,
    file_name_index: HashMap<String, usize>,
}

impl JarFile {
    /// Opens a JAR file from the specified path.
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = File::open(path.as_ref()).context(IoSnafu {
            path: path.as_ref().to_path_buf(),
        })?;
        let reader = BufReader::new(file);
        let mut zip_file = ZipArchive::new(reader)?;

        let mut file_name_index = HashMap::with_capacity(zip_file.len());
        for i in 0..zip_file.len() {
            let file = zip_file.by_index(i)?;
            if file.is_file() {
                let name = file.name().to_string();
                file_name_index.insert(name, i);
            }
        }

        Ok(JarFile {
            zip_file,
            file_name_index,
        })
    }

    /// Returns a vector of the names of the files contained in the JAR file.
    pub fn file_names(&self) -> Vec<String> {
        self.file_name_index.keys().cloned().collect()
    }

    /// Retrieves the content of a file within the JAR by its name.
    pub fn content_by_name(&mut self, name: &str) -> Result<Vec<u8>> {
        let index = self
            .file_name_index
            .get(name)
            .ok_or_else(|| JarError::EntryNotFound {
                entry_name: name.to_string(),
            })?;
        let mut file = self.zip_file.by_index(*index)?;
        let mut buffer = Vec::with_capacity(file.size() as usize);
        file.read_to_end(&mut buffer).map_err(|e| JarError::Io {
            source: e,
            path: name.into(),
        })?;
        Ok(buffer)
    }
}
