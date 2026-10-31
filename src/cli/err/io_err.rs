// src/cli/err/io_err.rs

#[derive(Debug, Clone, PartialEq)]
pub enum IOError {
    FailedToCreateFile,
    FailedToWrite,
    FileAlreadyExists,
    FailedToFlushBufWriter,
}