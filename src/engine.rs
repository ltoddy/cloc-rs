use std::fs;
use std::mem;
use std::path::PathBuf;
use std::sync::atomic::AtomicUsize;
use std::sync::mpsc::{sync_channel, SyncSender};
use std::sync::{Arc, Mutex, RwLock};

use crate::config::{Config, Info};
use crate::detail::Detail;
use crate::detail::{aggregate_details, LanguageDetail, SumDetail};
use crate::executor::ThreadPoolExecutor;
use crate::wrap;
use crate::ClocResult;

// TODO: implement
#[derive(Debug)]
pub(crate) struct Report {
    details: Vec<LanguageDetail>,
    sum: SumDetail,
    total_files: usize,
}

#[derive(Debug)]
pub struct Engine {
    config: Config,
    entry: PathBuf,
    total_files: AtomicUsize,
    text_files: AtomicUsize,
    ignored_files: AtomicUsize,
    unrecognized_files: AtomicUsize,
}

enum Message {
    Content(PathBuf),
    End,
}

impl Engine {
    #[inline]
    pub(crate) fn new(entry: PathBuf) -> Self {
        Self {
            config: Config::default(),
            entry,
            total_files: AtomicUsize::new(0),
            text_files: AtomicUsize::new(0),
            ignored_files: AtomicUsize::new(0),
            unrecognized_files: AtomicUsize::new(0),
        }
    }

    pub(crate) fn calculate(self) -> (Vec<LanguageDetail>, SumDetail) {
        let executor = ThreadPoolExecutor::new();
        let Engine {
            config,
            entry,
            total_files,
            ..
        } = self;

        let (config, _total_files) = wrap!(Arc, RwLock::new(config), total_files);
        let (sender, receiver) = sync_channel::<Message>(1024);
        let receiver = Arc::new(Mutex::new(receiver));

        let details = Arc::new(Mutex::new(Vec::new()));
        for _ in 0..executor.capacity() {
            let (receiver, config, details) = wrap!(Arc::clone, &receiver, &config, &details);

            executor.submit(move || {
                while let Ok(message) = receiver
                    .lock()
                    .expect("another user of this mutex panicked while holding the mutex")
                    .recv()
                {
                    match message {
                        Message::End => return,
                        Message::Content(path) => {
                            let info = match config
                                .read()
                                .expect("the RwLock is poisoned")
                                .get_by_extension(path.extension())
                            {
                                Some(info) => info.clone(),
                                None => continue,
                            };

                            if let Ok(detail) = calculate(path, info) {
                                details
                                    .lock()
                                    .expect("another user of this mutex panicked while holding the mutex")
                                    .push(detail);
                            }
                        }
                    }
                }
            });
        }
        explore(entry, &sender);
        for _ in 0..executor.capacity() {
            sender.send(Message::End).unwrap();
        }
        mem::drop(executor);

        aggregate_details(Arc::try_unwrap(details).unwrap().into_inner().unwrap())
    }
}

fn explore(dir: PathBuf, sender: &SyncSender<Message>) {
    // TODO: refactor
    if dir.is_file() {
        sender.send(Message::Content(dir)).unwrap();
    } else if dir.is_dir() {
        let entries = fs::read_dir(dir).unwrap();
        for entry in entries {
            let entry = entry.unwrap();

            let path = entry.path();
            if path.is_file() {
                // TODO: remove unwrap
                sender.send(Message::Content(path)).unwrap();
            } else if path.is_dir() {
                explore(path, sender);
            }
        }
    }
}

fn calculate(path: PathBuf, info: Info) -> ClocResult<Detail> {
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
        for single in &single {
            if line.starts_with(single) {
                comment += 1;
                continue 'here;
            }
        }

        // match multi line comments
        for (start, end) in &multi {
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
