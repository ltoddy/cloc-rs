use std::io::prelude::*;
use std::io::stdout;
use std::sync::{Arc, Condvar, Mutex};
use std::thread::{sleep, spawn};
use std::time::Duration;

pub struct Spinner {
    cvar: Arc<Condvar>,
    lock: Arc<Mutex<bool>>,
}

impl Spinner {
    pub fn new() -> Self {
        // TODO: consider using other type
        Self {
            cvar: Arc::new(Condvar::new()),
            lock: Arc::new(Mutex::new(false)),
        }
    }

    pub fn start(&self) {
        let Self { cvar, lock } = self;
        let pair = Arc::new((lock.clone(), cvar.clone()));
        spawn(move || {
            let mut out = stdout();
            for c in vec!['|', '/', '-', '\\'].iter().cycle() {
                let status = format!("{} computing", c);
                out.write_all(status.as_bytes()).unwrap();
                out.flush().unwrap();
                out.write_all("\x08".repeat(status.len()).as_bytes()).unwrap();
                sleep(Duration::from_millis(50));

                let (lock, cvar) = &*pair;
                let mut started = lock.lock().unwrap();
                let result = cvar.wait_timeout(started, Duration::from_millis(100)).unwrap();
                started = result.0;
                if *started {
                    break;
                }
            }
        });
    }

    pub fn stop(&self) {
        let Self { cvar, lock } = self;
        let mut started = lock.lock().unwrap();
        *started = true;
        cvar.notify_one();
    }
}
