use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs;
use std::ops::{Add, AddAssign};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::sync::Arc;

use lazy_static::lazy_static;

use crate::executor::ThreadPoolExecutor;
use crate::Result;

pub struct Calculator {
    filename_receiver: Receiver<PathBuf>,
    detail_sender: SyncSender<Detail>,
    executor: ThreadPoolExecutor,
}

impl Calculator {
    pub fn new(filename_receiver: Receiver<PathBuf>) -> (Self, Receiver<Detail>) {
        let (detail_sender, detail_receiver) = sync_channel::<Detail>(32);

        let calculator = Self {
            filename_receiver,
            detail_sender,
            executor: ThreadPoolExecutor::new(),
        };

        (calculator, detail_receiver)
    }

    pub fn serve(self) {
        let Calculator {
            filename_receiver,
            detail_sender,
            executor,
        } = self;

        let sender = Arc::new(detail_sender);

        for filename in filename_receiver {
            let sender = Arc::clone(&sender);
            executor.submit(move || {
                filename
                    .extension()
                    .and_then(|ext| MANAGER.get_by_extension(ext))
                    .and_then(|info| Self::statistical_detail(filename, info).ok())
                    .and_then(|detail| sender.send(detail).ok());
            });
        }
    }

    #[inline]
    fn statistical_detail<P: AsRef<Path> + Sync + Send>(filename: P, info: &Info) -> Result<Detail> {
        Self::statistical_detail_impl(filename.as_ref(), info)
    }

    fn statistical_detail_impl(filename: &Path, info: &Info) -> Result<Detail> {
        let Info {
            language,
            single,
            multi,
            ..
        } = info;

        let content = fs::read_to_string(&filename)?;
        let metadata = filename.metadata()?;
        let bytes = metadata.len();
        let mut blank = 0;
        let mut comment = 0;
        let mut code = 0;
        let mut in_comment: Option<(&str, &str)> = None;

        'here: for line in content.lines() {
            let line = line.trim();

            // empty line
            if line.is_empty() {
                blank += 1;
                continue;
            }

            // match single line comments
            for single in single {
                if line.starts_with(single) {
                    comment += 1;
                    continue 'here;
                }
            }

            // match multi line comments
            for (start, end) in multi {
                if let Some(d) = in_comment {
                    if d != (*start, *end) {
                        continue;
                    }
                }

                // multi line comments maybe in one line
                let mut same_line = false;
                if line.starts_with(start) {
                    in_comment = match in_comment {
                        Some(_) => {
                            comment += 1;
                            in_comment = None;
                            continue 'here;
                        }
                        None => {
                            same_line = true;
                            Some((start, end))
                        }
                    }
                }

                // This line is in comments
                if in_comment.is_some() {
                    comment += 1;
                    if line.ends_with(end) {
                        if same_line {
                            if line.len() >= (start.len() + end.len()) {
                                in_comment = None;
                            }
                        } else {
                            in_comment = None;
                        }
                    }
                    continue 'here;
                }
            }

            code += 1;
        }

        Ok(Detail::new(language, 1, bytes, blank, comment, code))
    }
}

#[derive(Debug, Clone)]
struct Info {
    language: &'static str,
    file_ext: Vec<&'static str>,
    single: Vec<&'static str>,
    multi: Vec<(&'static str, &'static str)>,
}

impl Info {
    #[inline]
    fn new(
        language: &'static str,
        file_ext: Vec<&'static str>,
        single: Vec<&'static str>,
        multi: Vec<(&'static str, &'static str)>,
    ) -> Self {
        Self {
            language,
            file_ext,
            single,
            multi,
        }
    }
}

struct Manager {
    languages: HashMap<&'static str, Info>,
    ext_to_language: HashMap<&'static str, &'static str>,
}

impl Manager {
    #[inline]
    fn get_by_extension(&self, ext: &OsStr) -> Option<&Info> {
        ext.to_str()
            .and_then(|ext| self.ext_to_language.get(ext))
            .and_then(|language| self.languages.get(language))
    }
}

lazy_static! {
    static ref MANAGER: Manager = {
        let mut languages = HashMap::<&'static str, Info>::new();
        let mut ext_to_language = HashMap::new();

        macro_rules! language {
            ($language: expr, $ext: expr, $single: expr, $multi: expr) => {{
                languages.insert($language, Info::new($language, $ext, $single, $multi));
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

        Manager {
            languages,
            ext_to_language,
        }
    };
}

#[derive(Debug, Copy, Clone)]
pub struct Detail {
    pub language: &'static str,
    pub files: usize,
    pub bytes: u64,
    pub blank: usize,
    pub comment: usize,
    pub code: usize,
}

impl Detail {
    pub fn new(language: &'static str, files: usize, bytes: u64, blank: usize, comment: usize, code: usize) -> Self {
        Self {
            language,
            files,
            bytes,
            blank,
            comment,
            code,
        }
    }
}

impl Add for Detail {
    type Output = Detail;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            language: self.language,
            files: self.files + rhs.files,
            bytes: self.bytes + rhs.bytes,
            blank: self.blank + rhs.blank,
            comment: self.comment + rhs.comment,
            code: self.code + rhs.comment,
        }
    }
}

impl AddAssign for Detail {
    fn add_assign(&mut self, rhs: Self) {
        self.files += rhs.files;
        self.bytes += rhs.bytes;
        self.blank += rhs.blank;
        self.comment += rhs.comment;
        self.code += rhs.code;
    }
}
