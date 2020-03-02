pub mod config;
pub mod detail;
pub mod engine;
pub mod error;
pub mod executor;
pub mod options;
pub mod pprint;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Language {
    Go,
    Rust,

    Illegal,
}

impl From<&str> for Language {
    fn from(ext_or_name: &str) -> Self {
        match ext_or_name {
            "Go" | "go" => Language::Go,
            "Rust" | "rs" => Language::Rust,
            _ => Language::Illegal,
        }
    }
}

impl Language {
    pub fn as_str(&self) -> &'static str {
        match self {
            Language::Rust => "Rust",
            Language::Go => "Go",

            Language::Illegal => "Illegal", // TODO
        }
    }
}

type Result<T> = std::result::Result<T, crate::error::ClocError>;
