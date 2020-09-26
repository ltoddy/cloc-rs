use crate::detail::Detail;
use std::sync::mpsc::Receiver;

pub struct Reporter {
    receiver: Receiver<Detail>,
}

impl Reporter {
    pub fn new(receiver: Receiver<Detail>) -> Self {
        Self { receiver }
    }

    // pub fn research(&self) -> Reporter {
    //
    // }
}

pub struct Report {
    section: Detail,
    summary: Detail,
}
