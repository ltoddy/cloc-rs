pub mod config;
pub mod detail;
pub mod engine;
pub mod error;
pub mod executor;
pub mod options;
pub mod pprint;
pub mod util;

// TODO: consider removing this enum, instead of use &'static str.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd)]
pub enum Language {
    C,
    CHeader,
    Cpp,
    CppHeader,
    Css,
    Go,
    Html,
    Haskell,
    JavaScript,
    Json,
    Julia,
    Java,
    Markdown,
    Python,
    Rust,
    Ruby,
    Scala,
    Shell,
    TypeScript,

    Illegal,
}

impl From<&str> for Language {
    fn from(ext_or_name: &str) -> Self {
        match ext_or_name {
            "C" | "c" => Language::C,
            "C header" | "h" => Language::CHeader,
            "Cpp" | "cpp" => Language::Cpp,
            "Cpp header" | "hpp" => Language::CppHeader,
            "Css" | "css" => Language::Css,
            "Go" | "go" => Language::Go,
            "Html" | "html" | "xhtml" | "htm" => Language::Html,
            "Haskell" | "hs" => Language::Haskell,
            "JavaScript" | "js" => Language::JavaScript,
            "Json" | "json" => Language::Json,
            "Julia" | "jl" => Language::Julia,
            "Java" | "java" => Language::Java,
            "Markdown" | "md" => Language::Markdown,
            "Python" | "py" => Language::Python,
            "Rust" | "rs" => Language::Rust,
            "Ruby" | "rb" => Language::Ruby,
            "Scala" | "scala" => Language::Scala,
            "Shell" | "sh" => Language::Shell,
            "TypeScript" | "ts" => Language::TypeScript,
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
            Language::Css => "Css",
            Language::Go => "Go",
            Language::Html => "Html",
            Language::Haskell => "Haskell",
            Language::JavaScript => "JavaScript",
            Language::Json => "Json",
            Language::Julia => "Julia",
            Language::Java => "Java",
            Language::Markdown => "Markdown",
            Language::Python => "Python",
            Language::Rust => "Rust",
            Language::Ruby => "Ruby",
            Language::Scala => "Scala",
            Language::Shell => "Shell",
            Language::TypeScript => "TypeScript",

            Language::Illegal => "Illegal", // TODO
        }
    }
}

pub type ClocResult<T> = std::result::Result<T, crate::error::ClocError>;
