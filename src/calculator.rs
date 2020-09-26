use std::fs;
use std::iter;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::sync::RwLock;

use crate::config::Info;
use crate::config::MANAGER;
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

        let mut counter = 0;
        let detail_senders = RwLock::new(iter::repeat(detail_sender).take(num_cpus::get()).collect::<Vec<_>>());

        for filename in filename_receiver {
            executor.submit(move || {
                if let Some(ext) = filename.extension() {
                    if let Some(info) = MANAGER.get_by_extension(ext) {
                        if let Ok(detail) = Self::statistical_detail(filename, info) {
                            println!("detail: {:?}", detail);
                        }
                    }
                }
            });
        }
    }

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

        Ok(Detail::new(language, bytes, blank, comment, code))
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Detail {
    language: &'static str,
    size: u64,
    blank: usize,
    comment: usize,
    code: usize,
}

impl Detail {
    pub fn new(language: &'static str, size: u64, blank: usize, comment: usize, code: usize) -> Self {
        Self {
            language,
            size,
            blank,
            comment,
            code,
        }
    }
}
