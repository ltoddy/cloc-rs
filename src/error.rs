use std::fmt;
use std::io::Error;

#[derive(Debug)]
pub enum ClocError {
    Unrecognized,
    NonTextFile,
    Io,
}

impl std::error::Error for ClocError {}

impl fmt::Display for ClocError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO
        match self {
            ClocError::Unrecognized => write!(f, "Unrecognized"),
            ClocError::NonTextFile => write!(f, "NonTextFile"),
            ClocError::Io => write!(f, "Io"),
        }
    }
}

impl From<std::io::Error> for ClocError {
    fn from(e: Error) -> Self {
        ClocError::Io
    }
}
