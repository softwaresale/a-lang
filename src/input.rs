use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;
use crate::error::internal::InternalError;

pub enum SourceInputKind {
    /// The input came from a file
    File {
        /// the path of the file we are compiling
        path: PathBuf,
    },
    /// The input came from a raw string
    Raw
}

pub struct SourceInput {
    /// the source buffer input
    buffer: String,
    /// the kind of input
    kind: SourceInputKind,
}

impl SourceInput {
    pub fn open<PathT: Into<PathBuf>>(path: PathT) -> Result<Self, InternalError> {
        let path = path.into();
        let file = File::open(&path)
            .map_err(|err| InternalError::new(format!("Failed to open input file {}", path.display()))
                .with_cause(Box::new(err) as Box<dyn Error>)
            )?;

        let mut reader = BufReader::new(file);
        let mut buffer_string = String::default();
        reader.read_to_string(&mut buffer_string)
            .map_err(|io_err| InternalError::new("Error while reading input file")
                .with_cause(Box::new(io_err) as Box<dyn Error>)
            )?;

        Ok(Self {
            buffer: buffer_string,
            kind: SourceInputKind::File {
                path
            }
        })
    }

    pub fn raw<ContentT: Into<String>>(content: ContentT) -> Self {
        Self {
            buffer: content.into(),
            kind: SourceInputKind::Raw,
        }
    }

    pub fn contains_non_ascii(&self) -> bool {
        for ch in self.buffer.chars() {
            if !ch.is_ascii() {
                return true;
            }
        }

        false
    }

    pub fn source(&self) -> &str {
        &self.buffer
    }
}
