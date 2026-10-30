// src/cli/sha/io_err.rs

#[derive(Debug, Clone, PartialEq)]
pub enum IOError {
    FailedToCreateFile,
    FailedToWrite,
    FileAlreadyExists,
    FailedToFlushBufWriter,
}