use std::fs;
use std::path::PathBuf;

use crate::detail::Detail;

#[derive(Debug)]
pub struct Engine {
    entry: PathBuf,
}

impl Engine {
    pub fn new(entry: PathBuf) -> Self {
        Self { entry }
    }

    pub fn calculate(self) -> Detail {
        let Engine { entry } = self;

        calculate(entry)
    }
}

fn calculate(path: PathBuf) -> Detail {
    // hard code, temporarily
    let language = "Rust";
    let single_comment = vec!["//", "///", "///!"];
    let multi_comment = vec![("/*", "*/")];

    // --------------------------------------

    let content = fs::read_to_string(path).unwrap(); // TODO: remove unwrap
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
        for single in &single_comment {
            if line.starts_with(single) {
                comment += 1;
                continue 'here;
            }
        }

        // match multi line comments
        for (start, end) in &multi_comment {
            if let Some(d) = in_comment {
                if d != (start, end) {
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
            if let Some(_) = in_comment {
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

    Detail::new(language, blank, comment, code)
}
