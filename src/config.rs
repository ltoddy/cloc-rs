use std::collections::HashMap;
use std::ffi::OsStr;

#[derive(Debug, Clone)]
pub struct Info {
    pub name: &'static str,
    pub file_ext: Vec<&'static str>,
    pub single: Vec<&'static str>,
    pub multi: Vec<(&'static str, &'static str)>,
}

#[derive(Debug)]
pub struct Config {
    // TODO: rename languages -> names
    pub languages: HashMap<&'static str, Info>,
    ext_to_name: HashMap<&'static str, &'static str>,
}

impl Default for Config {
    fn default() -> Self {
        let mut languages = HashMap::new();
        let mut ext_to_name = HashMap::new();

        macro_rules! language {
            ($name: expr, $ext: expr, $single: expr, $multi: expr) => {{
                languages.insert(
                    $name,
                    Info {
                        name: $name,
                        file_ext: $ext,
                        single: $single,
                        multi: $multi,
                    },
                );
                for e in $ext {
                    ext_to_name.insert(e, $name);
                }
            }};
            ($name: expr, $ext: expr, $single: expr) => {
                language!($name, $ext, $single, vec![])
            };
            ($name: expr, $ext: expr) => {
                language!($name, $ext, vec![], vec![])
            };
        }

        language!("C", vec!["c"], vec!["//"], vec![("/*", "*/")]);
        language!("CHeader", vec!["h"], vec!["//"], vec![("/*", "*/")]);
        language!("Cpp", vec!["cpp"], vec!["//"], vec![("/*", "*/")]);
        language!("CppHeader", vec!["hpp"], vec!["//"], vec![("/*", "*/")]);
        language!(
            "CSS",
            vec!["css", "sass", "less", "scss"],
            vec!["//"],
            vec![("/*", "*/")]
        );
        language!("Go", vec!["go"], vec!["//"], vec![("/*", "*/"), ("/**", "*/")]);
        language!("Html", vec!["html", "xhtml", "hml"]);
        language!("Haskell", vec!["hs"], vec!["--"], vec![("{-", "-}")]);
        language!("JavaScript", vec!["js", "ejs"], vec!["//"], vec![("/*", "*/")]);
        language!("Json", vec!["json"]);
        language!("Julia", vec!["jl"], vec!["#"], vec![("#=", "=#")]);
        language!("Java", vec!["java"], vec!["//"], vec![("/*", "*/")]);
        language!("Markdown", vec!["md"]);
        language!("Python", vec!["py"], vec!["#"], vec![("'''", "'''"), (r#"""#, r#"""#)]);
        language!("Rust", vec!["rs"], vec!["//", "///", "///!"], vec![("/*", "*/")]);
        language!("Ruby", vec!["rb"], vec!["#"], vec![("=", "=")]);
        language!("Scala", vec!["scala"], vec!["//"], vec![("/*", "*/")]);
        language!("Shell", vec!["sh"], vec!["#"]);
        language!("TypeScript", vec!["ts"], vec!["//"], vec![("/*", "*/")]);

        Self { languages, ext_to_name }
    }
}

impl Config {
    pub fn get_by_extension(&self, ext: Option<&OsStr>) -> Option<&Info> {
        ext.and_then(|ext| ext.to_str())
            .and_then(|ext| self.ext_to_name.get(ext))
            .and_then(|name| self.languages.get(name))
    }
}
