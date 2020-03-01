use std::collections::HashMap;
use std::iter::FromIterator;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Language {
    Rust,
}

impl From<&str> for Language {
    fn from(source: &str) -> Self {
        match source {
            "rs" => Language::Rust,
            _ => unimplemented!(),
        }
    }
}

impl Language {
    pub fn as_str(&self) -> &'static str {
        match self {
            Language::Rust => "Rust",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Info {
    // TODO: remove?
    pub name: Language,
    pub file_ext: Vec<&'static str>,
    pub single: Vec<&'static str>,
    pub multi: Vec<(&'static str, &'static str)>,
}

macro_rules! language {
    ($name: expr, $ext: expr, $single: expr, $multi: expr) => {
        (
            Language::from($name),
            Info {
                name: $name,
                file_ext: $ext,
                single: $single,
                multi: $multi,
            },
        )
    };
}

#[derive(Debug)]
pub struct Config {
    pub languages: HashMap<Language, Info>,
}

impl Config {
    pub fn new() -> Self {
        use self::Language::*;

        Self {
            languages: HashMap::from_iter(vec![language!(
                Rust,
                vec!["rs"],
                vec!["//", "///", "///!"],
                vec![("/*", "*/")]
            )]),
        }
    }

    pub fn get(&self, ext: &str) -> Option<&Info> {
        self.languages.get(&Language::from(ext))
    }
}
