use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClocError {
    #[error("{0}")]
    Io(#[from] std::io::Error),

    #[error("invalid argument: {0}")]
    InvalidArg(String),
}
