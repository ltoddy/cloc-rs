use std::collections::HashMap;
use std::ffi::OsStr;

#[derive(Debug, Clone)]
pub(crate) struct Info {
    pub(crate) language: &'static str,
    pub(crate) file_ext: Vec<&'static str>,
    pub(crate) single: Vec<&'static str>,
    pub(crate) multi: Vec<(&'static str, &'static str)>,
}

#[derive(Debug, Clone)]
pub(crate) struct Config {
    pub(crate) languages: HashMap<&'static str, Info>,
    ext_to_language: HashMap<&'static str, &'static str>,
}

#[allow(clippy::useless_vec)]
impl Default for Config {
    fn default() -> Self {
        let mut languages = HashMap::new();
        let mut ext_to_language = HashMap::new();

        macro_rules! language {
            ($language: expr, $ext: expr, $single: expr, $multi: expr) => {{
                languages.insert(
                    $language,
                    Info {
                        language: $language,
                        file_ext: $ext,
                        single: $single,
                        multi: $multi,
                    },
                );
                for e in $ext {
                    ext_to_language.insert(e, $language);
                }
            }};
            ($language: expr, $ext: expr, $single: expr) => {
                language!($language, $ext, $single, vec![])
            };
            ($language: expr, $ext: expr) => {
                language!($language, $ext, vec![], vec![])
            };
        }

        language!("Bat", vec!["bat", "cmd"], vec!["@rem"]);
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
        language!("Gradle", vec!["gradle"], vec!["//"], vec![("/*", "*/"), ("/**", "*/")]);
        language!("Html", vec!["html", "xhtml", "hml"]);
        language!("Haskell", vec!["hs"], vec!["--"], vec![("{-", "-}")]);
        language!("Java", vec!["java"], vec!["//"], vec![("/*", "*/")]);
        language!("JavaScript", vec!["js", "ejs"], vec!["//"], vec![("/*", "*/")]);
        language!("Json", vec!["json"]);
        language!("Julia", vec!["jl"], vec!["#"], vec![("#=", "=#")]);
        language!("Markdown", vec!["md"]);
        language!(
            "Php",
            vec!["php4", "php5", "php", "phtml"],
            vec!["#", "//"],
            vec![("/*", "*/"), ("/**", "*/")]
        );
        language!("Protobuf", vec!["proto"], vec!["//"]);
        language!("Python", vec!["py"], vec!["#"], vec![("'''", "'''"), (r#"""#, r#"""#)]);
        language!("Rust", vec!["rs"], vec!["//", "///", "///!"], vec![("/*", "*/")]);
        language!("Ruby", vec!["rb"], vec!["#"], vec![("=", "=")]);
        language!("Scala", vec!["scala"], vec!["//"], vec![("/*", "*/")]);
        language!("Shell", vec!["sh"], vec!["#"]);
        language!("Sql", vec!["sql"], vec!["#", "--"], vec![("/*", "*/")]);
        language!("Toml", vec!["toml"], vec!["#"]);
        language!("TypeScript", vec!["ts"], vec!["//"], vec![("/*", "*/")]);
        language!(
            "Xml",
            vec!["xml"],
            vec!["!there is no specific single line comment!"],
            vec![("<!--", "-->"), ("<![CDATA[", "]]>")]
        );
        language!("Yaml", vec!["yml", "yaml"], vec!["#"]);

        Self {
            languages,
            ext_to_language,
        }
    }
}

impl Config {
    #[inline]
    pub(crate) fn get_by_extension(&self, ext: Option<&OsStr>) -> Option<&Info> {
        ext.and_then(|ext| ext.to_str())
            .and_then(|ext| self.ext_to_language.get(ext))
            .and_then(|language| self.languages.get(language))
    }
}
