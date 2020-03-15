use std::io::prelude::*;
use std::io::stdout;
use std::sync::{Arc, Condvar, Mutex};
use std::thread::{sleep, spawn};
use std::time::Duration;

#[derive(Debug, Default)]
pub(crate) struct Spinner {
    cvar: Arc<Condvar>,
    lock: Arc<Mutex<bool>>,
}

impl Spinner {
    pub(crate) fn new() -> Self {
        Self {
            cvar: Arc::new(Condvar::new()),
            lock: Arc::new(Mutex::new(false)),
        }
    }

    pub(crate) fn start(&self) {
        let Self { cvar, lock } = self;
        let pair = (Arc::clone(&lock), Arc::clone(&cvar));
        spawn(move || {
            let mut out = stdout();
            for c in vec!['|', '/', '-', '\\'].iter().cycle() {
                let status = format!("{} computing", c);
                let _ = out.write_all(status.as_bytes());
                let _ = out.flush();
                let _ = out.write_all("\x08".repeat(status.len()).as_bytes());
                sleep(Duration::from_millis(50));

                let (lock, cvar) = &pair;

                if let Ok(mut started) = lock.lock() {
                    if let Ok(result) = cvar.wait_timeout(started, Duration::from_millis(100)) {
                        started = result.0;
                        if *started {
                            break;
                        }
                    }
                }
            }
        });
    }

    pub(crate) fn stop(&self) {
        let Self { cvar, lock } = self;
        if let Ok(mut started) = lock.lock() {
            *started = true;
            cvar.notify_one();
        }
    }
}
