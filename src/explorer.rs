use std::fs;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};

pub struct Explorer {
    ignore_list: Option<Vec<PathBuf>>,
    sender: SyncSender<PathBuf>,
}

impl Explorer {
    pub fn new(ignore_list: Option<Vec<PathBuf>>) -> (Self, Receiver<PathBuf>) {
        let (sender, receiver) = sync_channel::<PathBuf>(1024);
        let explorer = Self { ignore_list, sender };
        (explorer, receiver)
    }

    pub fn walk_directory<P: AsRef<Path>>(&mut self, entry: P) {
        self.walk_dir_impl(entry.as_ref());
    }

    fn walk_dir_impl(&mut self, path: &Path) {
        if path.is_file() && self.is_not_ignore_file(path) {
            let _ = self.sender.send(PathBuf::from(path));
        } else if path.is_dir() {
            if let Ok(entries) = fs::read_dir(path) {
                entries
                    .filter_map(|entry| entry.ok())
                    .for_each(|entry| self.walk_dir_impl(&entry.path()));
            }
        }
    }

    #[inline]
    fn is_not_ignore_file<P: AsRef<Path>>(&self, filename: P) -> bool {
        self.is_not_ignore_file_impl(filename.as_ref())
    }

    #[inline]
    fn is_not_ignore_file_impl(&self, filename: &Path) -> bool {
        if let Some(ignore_list) = &self.ignore_list {
            return ignore_list.iter().all(|path| !filename.starts_with(path));
        }

        true
    }
}
