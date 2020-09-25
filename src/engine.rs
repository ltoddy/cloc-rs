use std::env::current_dir;
use std::fs;
use std::mem;
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicUsize;
use std::sync::mpsc::{sync_channel, SyncSender};
use std::sync::{Arc, Mutex, RwLock};

use crate::calculate::calculate;
use crate::config::Config;
use crate::detail::{aggregate_details, LanguageDetail, SumDetail};
use crate::executor::ThreadPoolExecutor;

// TODO: implement
#[derive(Debug)]
pub struct Report {
    pub languages: Vec<LanguageDetail>,
    pub sum: SumDetail,
}

impl Report {
    pub fn new(languages: Vec<LanguageDetail>, sum: SumDetail) -> Self {
        Self { languages, sum }
    }
}

#[derive(Debug)]
pub struct Engine {
    ignore_list: Vec<PathBuf>,
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
    pub fn new(mut entry: PathBuf, ignore_file: Option<PathBuf>) -> Self {
        if !entry.exists() {
            println!("can't find path: {:?}, so use current directory as entry.", entry);
            entry = current_dir().unwrap();
        }

        let ignore_list = Self::read_ignore_list(ignore_file).unwrap_or(Vec::<PathBuf>::new());

        Self {
            ignore_list,
            config: Config::default(),
            entry,
            total_files: 0,
            text_files: AtomicUsize::new(0),
            ignored_files: AtomicUsize::new(0),
            unrecognized_files: AtomicUsize::new(0),
        }
    }

    // TODO: 这个函数应该返回`Report`结构体, 这样, pprint.rs中的输出函数只需要知道`Report`结构体就可以了, `LanguageDetail`与`SumDetail`便可以是crate private的了.
    pub fn calculate(&mut self) -> Report {
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
                        Message::Content(ref path) => {
                            let info = match config
                                .read()
                                .expect("the RwLock is poisoned")
                                .get_by_extension(path.extension())
                            {
                                Some(info) => info.clone(),
                                None => continue,
                            };

                            if let Ok(detail) = calculate(path, &info) {
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
        if dir.is_file() && !self.is_ignored_file(&dir) {
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

    fn read_ignore_list(filename: Option<PathBuf>) -> Option<Vec<PathBuf>> {
        filename.and_then(|filename| {
            fs::read_to_string(filename)
                .map(|content| {
                    content
                        .lines()
                        .filter_map(|path| fs::canonicalize(path).ok())
                        .collect::<Vec<_>>()
                })
                .ok()
        })
    }

    #[inline]
    fn is_ignored_file<P: AsRef<Path>>(&self, filename: P) -> bool {
        self.is_ignored_file_impl(filename.as_ref())
    }

    fn is_ignored_file_impl(&self, filename: &Path) -> bool {
        if filename.is_dir() {
            return false;
        }
        self.ignore_list
            .iter()
            .any(|ignored_path| filename.starts_with(ignored_path))
    }
}
