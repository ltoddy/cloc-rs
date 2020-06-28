use std::fs;
use std::path::Path;

use crate::config::Info;
use crate::detail::Detail;
use crate::ClocResult;

pub(crate) fn calculate<P: AsRef<Path>>(path: P, info: &Info) -> ClocResult<Detail> {
    calculate_impl(path.as_ref(), info)
}

fn calculate_impl(path: &Path, info: &Info) -> ClocResult<Detail> {
    let Info {
        language,
        single,
        multi,
        ..
    } = info;

    let content = fs::read_to_string(&path)?;
    let metadata = path.metadata()?;
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

    Ok(Detail::new(language, bytes, blank, comment, code))
}
