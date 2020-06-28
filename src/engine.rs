use std::fs;
use std::mem;
use std::path::{Path, PathBuf};
use std::env::current_dir;
use std::sync::atomic::AtomicUsize;
use std::sync::mpsc::{sync_channel, SyncSender};
use std::sync::{Arc, Mutex, RwLock};

use crate::calculate::calculate;
use crate::config::Config;
use crate::detail::{aggregate_details, LanguageDetail, SumDetail};
use crate::executor::ThreadPoolExecutor;

// TODO: implement
#[derive(Debug)]
pub(crate) struct Report {
    pub(crate) languages: Vec<LanguageDetail>,
    pub(crate) sum: SumDetail,
}

impl Report {
    pub(crate) fn new(languages: Vec<LanguageDetail>, sum: SumDetail) -> Self {
        Self { languages, sum }
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
    pub(crate) fn new(mut entry: PathBuf) -> Self {
        if !entry.exists() {
            println!("can't find path: {:?}, so use current directory as entry.", entry);
            entry = current_dir().unwrap();
        }

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
    pub(crate) fn calculate(&mut self) -> Report {
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
