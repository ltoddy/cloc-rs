use std::fmt::Formatter;

#[derive(Debug)]
pub(crate) enum ClocError {
    Io(std::io::Error),

    InvalidArg(String),
}

impl std::error::Error for ClocError {}

impl From<std::io::Error> for ClocError {
    fn from(err: std::io::Error) -> Self {
        ClocError::Io(err)
    }
}

impl std::fmt::Display for ClocError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ClocError::Io(err) => write!(f, "{}", err),
            ClocError::InvalidArg(s) => write!(f, "invalid argument: {}", s),
        }
    }
}
