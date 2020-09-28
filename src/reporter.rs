use std::collections::HashMap;
use std::ops::AddAssign;
use std::sync::mpsc::Receiver;

use crate::calculator::Detail;

pub struct Reporter {
    receiver: Receiver<Detail>,
}

impl Reporter {
    pub fn new(receiver: Receiver<Detail>) -> Self {
        Self { receiver }
    }

    pub fn research(&self) -> Report {
        let mut kinds = HashMap::<&str, Detail>::new();
        let mut summary = Detail::new("Sum", 0, 0, 0, 0, 0);

        for detail in &self.receiver {
            summary += detail;

            kinds
                .entry(detail.language)
                .and_modify(|d| d.add_assign(detail))
                .or_insert(detail);
        }

        Report {
            sections: kinds.values().cloned().collect::<Vec<_>>(),
            summary,
        }
    }
}

#[derive(Debug)]
pub struct Report {
    pub sections: Vec<Detail>,
    pub summary: Detail,
}
