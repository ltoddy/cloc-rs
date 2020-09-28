use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::sync::{Arc, Mutex};
use std::thread;

enum Message {
    Job(Box<dyn FnOnce() + Send + 'static>),
    Terminate,
}

struct Worker {
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(receiver: Arc<Mutex<Receiver<Message>>>) -> Self {
        let thread = thread::spawn(move || Self::work(receiver));

        Self { thread: Some(thread) }
    }

    fn work(receiver: Arc<Mutex<Receiver<Message>>>) {
        loop {
            let message = receiver
                .lock()
                .expect("Poisoned thread")
                .recv()
                .expect("ThreadPoolExecutor sender has disconnected");

            match message {
                Message::Job(job) => job(),
                Message::Terminate => break,
            }
        }
    }
}

pub struct ThreadPoolExecutor {
    workers: Vec<Worker>,
    sender: SyncSender<Message>,
}

impl ThreadPoolExecutor {
    pub fn new() -> Self {
        Self::with_capacity(num_cpus::get() * 5)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        assert!(capacity > 0);

        let (sender, receiver) = sync_channel::<Message>(1024);
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::<Worker>::with_capacity(capacity);

        for _ in 0..capacity {
            workers.push(Worker::new(Arc::clone(&receiver)));
        }

        Self { workers, sender }
    }

    pub fn submit<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Message::Job(Box::new(f));

        self.sender
            .send(job)
            .expect("ThreadPoolExecutor receiver has disconnected");
    }
}

impl Default for ThreadPoolExecutor {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for ThreadPoolExecutor {
    fn drop(&mut self) {
        for _ in 0..self.workers.len() {
            self.sender
                .send(Message::Terminate)
                .expect("ThreadPoolExecutor receiver has disconnected");
        }

        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().expect("thread::spawn failed");
            }
        }
    }
}
