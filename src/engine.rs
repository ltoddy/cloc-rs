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
use crate::ClocResult;

// TODO: implement
#[derive(Debug)]
pub(crate) struct Report {
    details: Vec<LanguageDetail>,
    sum: SumDetail,
    total_files: usize,
}

impl Report {
    fn new(details: Vec<LanguageDetail>, sum: SumDetail, total_files: usize) -> Self {
        Self {
            details,
            sum,
            total_files,
        }
    }
}

#[derive(Debug)]
pub struct Engine {
    config: Config,
    entry: PathBuf,
    total_files: usize,
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
            total_files: 0,
            text_files: AtomicUsize::new(0),
            ignored_files: AtomicUsize::new(0),
            unrecognized_files: AtomicUsize::new(0),
        }
    }

    // TODO: 这个函数应该返回`Report`结构体, 这样, pprint.rs中的输出函数只需要知道`Report`结构体就可以了, `LanguageDetail`与`SumDetail`便可以是crate private的了.
    pub(crate) fn calculate(&mut self) -> (Vec<LanguageDetail>, SumDetail) {
        let executor = ThreadPoolExecutor::new();
        let (sender, receiver) = sync_channel::<Message>(1024);

        let config = Arc::new(RwLock::new(self.config.clone()));
        let receiver = Arc::new(Mutex::new(receiver));
        let details = Arc::new(Mutex::new(Vec::new()));

        for _ in 0..executor.capacity() {
            let receiver = Arc::clone(&receiver);
            let config = Arc::clone(&config);
            let details = Arc::clone(&details);

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
        self.explore(self.entry.clone(), &sender);
        for _ in 0..executor.capacity() {
            sender.send(Message::End).unwrap();
        }
        mem::drop(executor);

        aggregate_details(Arc::try_unwrap(details).unwrap().into_inner().unwrap())
    }

    fn explore(&mut self, dir: PathBuf, sender: &SyncSender<Message>) {
        if dir.is_file() {
            self.total_files += 1;
            sender.send(Message::Content(dir)).unwrap();
        } else if dir.is_dir() {
            let entries = fs::read_dir(dir).unwrap();
            for entry in entries {
                let entry = entry.unwrap();
                self.explore(entry.path(), sender);
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
