use std::fmt;

#[derive(Debug)]
pub enum ClocError {
    Unrecognized,
}

impl std::error::Error for ClocError {}

impl fmt::Display for ClocError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
