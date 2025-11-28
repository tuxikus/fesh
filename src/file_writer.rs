use std::{fs, io::Write, path::Path};

#[derive(Debug)]
pub enum FileWriterError {
    Io(std::io::Error),
}

pub struct FileWriter {}

impl FileWriter {
    pub fn new() -> FileWriter {
        FileWriter {}
    }

    pub fn overwrite_file(&self, path: &Path, content: &str) -> Result<usize, FileWriterError> {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(path)
            .map_err(FileWriterError::Io)?;

        file.write(content.as_bytes()).map_err(FileWriterError::Io)
    }

    pub fn append_to_file(&self, path: &Path, content: &str) -> Result<usize, FileWriterError> {
        let mut file = fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(path)
            .map_err(FileWriterError::Io)?;

        file.write(content.as_bytes()).map_err(FileWriterError::Io)
    }
}
