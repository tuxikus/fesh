use std::{fs, io::Write, path::Path};

use crate::logger;

#[derive(Debug)]
pub enum FileWriterError {
    Io(std::io::Error),
}

pub struct FileWriter {
    pub logger: logger::Logger,
}

impl FileWriter {
    pub fn new() -> FileWriter {
        FileWriter {
            logger: logger::Logger::new(false),
        }
    }

    pub fn overwrite_file(&self, path: &Path, content: &str) -> Result<usize, FileWriterError> {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(path)
            .map_err(FileWriterError::Io)?;

        self.logger.print_debug(String::from("FileWriter"), format!("overwriting file <{}>", path.display()));

        file.write(content.as_bytes()).map_err(FileWriterError::Io)
    }

    pub fn append_to_file(&self, path: &Path, content: &str) -> Result<usize, FileWriterError> {
        let mut file = fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(path)
            .map_err(FileWriterError::Io)?;

        self.logger.print_debug(String::from("FileWriter"), format!("appending to file <{}>", path.display()));

        file.write(content.as_bytes()).map_err(FileWriterError::Io)
    }
}
