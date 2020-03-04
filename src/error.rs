use std::fmt;
use std::io::Error;

#[derive(Debug)]
pub enum ClocError {
    Unrecognized,
    NonTextFile,
    Io(std::io::Error),
    InvalidCommandArgs,
}

impl std::error::Error for ClocError {}

impl fmt::Display for ClocError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO
        match self {
            ClocError::Unrecognized => write!(f, "Unrecognized"),
            ClocError::NonTextFile => write!(f, "NonTextFile"),
            ClocError::Io(_) => write!(f, "Io"),
            InvalidCommandArgs => todo!()
        }
    }
}

impl From<std::io::Error> for ClocError {
    fn from(e: Error) -> Self {
        ClocError::Io(e)
    }
}
