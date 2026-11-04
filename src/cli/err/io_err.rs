// src/cli/err/io_err.rs

#[derive(Debug, Clone, PartialEq)]
pub enum IOError {
    FailedToCreateFile,
    FailedToOpenFile,
    FailedToReadLines,
    FailedToWrite,
    FileAlreadyExists,
    FailedToFlushBufWriter,
    NoGivenFile,
}

impl std::error::Error for IOError {}
impl std::fmt::Display for IOError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}