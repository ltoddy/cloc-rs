use crate::calculator::Detail;
use std::sync::mpsc::Receiver;

pub struct Reporter {
    receiver: Receiver<Detail>,
}

impl Reporter {
    pub fn new(receiver: Receiver<Detail>) -> Self {
        Self { receiver }
    }

    pub fn research(&self) {
        for detail in &self.receiver {
            println!("{:?}", detail);
        }
    }
}

pub struct Report {
    section: Detail,
    summary: Detail,
}
