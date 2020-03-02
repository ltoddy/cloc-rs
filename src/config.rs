use std::collections::HashMap;
use std::ffi::OsStr;
use std::iter::FromIterator;

use crate::Language;

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

impl Default for Config {
    fn default() -> Self {
        use self::Language::*;

        Self {
            languages: HashMap::from_iter(vec![
                language!(Go, vec!["go"], vec!["//"], vec![("/*", "*/"), ("/**", "*/")]),
                language!(Rust, vec!["rs"], vec!["//", "///", "///!"], vec![("/*", "*/")]),
            ]),
        }
    }
}

impl Config {
    pub fn get_by_extension(&self, ext: Option<&OsStr>) -> Option<&Info> {
        ext.and_then(|ext| ext.to_str())
            .and_then(|ext| self.languages.get(&Language::from(ext)))
    }
}
