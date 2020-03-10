pub mod config;
pub mod detail;
pub mod engine;
pub mod error;
pub mod executor;
pub mod options;
pub mod pprint;
pub mod spinner;
pub mod util;

pub type ClocResult<T> = std::result::Result<T, crate::error::ClocError>;
