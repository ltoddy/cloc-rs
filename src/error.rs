#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),

    InvalidArg(String),
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(err) => write!(f, "{}", err),
            Error::InvalidArg(s) => write!(f, "invalid argument: {}", s),
        }
    }
}
