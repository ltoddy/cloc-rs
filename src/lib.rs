pub mod config;
pub mod detail;
pub mod engine;
pub mod error;
pub mod executor;
pub mod options;
pub mod pprint;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Language {
    C,
    CHeader,
    Cpp,
    CppHeader,
    Go,
    Rust,

    Illegal,
}

impl From<&str> for Language {
    fn from(ext_or_name: &str) -> Self {
        match ext_or_name {
            "C" | "c" => Language::C,
            "C header" | "h" => Language::CHeader,
            "Cpp" | "cpp" => Language::Cpp,
            "Cpp header" | "hpp" => Language::CppHeader,
            "Go" | "go" => Language::Go,
            "Rust" | "rs" => Language::Rust,
            _ => Language::Illegal,
        }
    }
}

impl Language {
    pub fn as_str(&self) -> &'static str {
        match self {
            Language::C => "C",
            Language::CHeader => "C header",
            Language::Cpp => "Cpp",
            Language::CppHeader => "Cpp header",
            Language::Rust => "Rust",
            Language::Go => "Go",

            Language::Illegal => "Illegal", // TODO
        }
    }
}

pub type ClocResult<T> = std::result::Result<T, crate::error::ClocError>;
