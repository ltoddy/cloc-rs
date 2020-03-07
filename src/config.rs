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
                language!(C, vec!["c"], vec!["//"], vec![("/*", "*/")]),
                language!(CHeader, vec!["h"], vec!["//"], vec![("/*", "*/")]),
                language!(Cpp, vec!["cpp"], vec!["//"], vec![("/*", "*/")]),
                language!(CppHeader, vec!["hpp"], vec!["//"], vec![("/*", "*/")]),
                language!(Css, vec!["css", "sass", "less", "scss"], vec!["//"], vec![("/*", "*/")]),
                language!(Go, vec!["go"], vec!["//"], vec![("/*", "*/"), ("/**", "*/")]),
                language!(Html, vec!["html", "xhtml", "hml"], vec![], vec![]),
                language!(Haskell, vec!["hs"], vec!["--"], vec![("{-", "-}")]),
                language!(JavaScript, vec!["js", "ejs"], vec!["//"], vec![("/*", "*/")]),
                language!(Json, vec!["json"], vec![], vec![]),
                language!(Julia, vec!["jl"], vec!["#"], vec![("#=", "=#")]),
                language!(Java, vec!["java"], vec!["//"], vec![("/*", "*/")]),
                language!(Markdown, vec!["md"], vec![], vec![]),
                language!(Python, vec!["py"], vec!["#"], vec![("'''", "'''"), (r#"""#, r#"""#)]),
                language!(Rust, vec!["rs"], vec!["//", "///", "///!"], vec![("/*", "*/")]),
                language!(Ruby, vec!["rb"], vec!["#"], vec![("=", "=")]),
                language!(Scala, vec!["scala"], vec!["//"], vec![("/*", "*/")]),
                language!(Shell, vec!["sh"], vec!["#"], vec![]),
                language!(TypeScript, vec!["ts"], vec!["//"], vec![("/*", "*/")]),
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
