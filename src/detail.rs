use std::collections::HashMap;
use std::ops::AddAssign;

use crate::Language;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Detail {
    pub language: &'static str,
    pub bytes: u64,
    pub blank: usize,
    pub comment: usize,
    pub code: usize,
}

impl Detail {
    pub fn new(language: &'static str, bytes: u64, blank: usize, comment: usize, code: usize) -> Self {
        Self {
            language,
            bytes,
            blank,
            comment,
            code,
        }
    }

    pub fn from_other(other: &Detail) -> Self {
        Self {
            language: other.language,
            bytes: 0,
            blank: 0,
            comment: 0,
            code: 0,
        }
    }
}

impl AddAssign for Detail {
    fn add_assign(&mut self, rhs: Self) {
        self.bytes += rhs.bytes;
        self.blank += rhs.blank;
        self.comment += rhs.comment;
        self.code += rhs.code;
    }
}

#[derive(Debug, Default)]
pub struct SumDetail {
    pub bytes: u64,
    pub blank: usize,
    pub comment: usize,
    pub code: usize,
}

#[derive(Debug)]
pub struct TotalDetail {
    pub kinds: HashMap<Language, Detail>,
    pub sum: SumDetail,
}

impl TotalDetail {
    pub fn from_details(details: Vec<Detail>) -> Self {
        let mut total = Self {
            kinds: HashMap::new(),
            sum: SumDetail::default(),
        };

        for detail in details {
            total.add(detail);
        }

        total
    }

    fn add(&mut self, detail: Detail) {
        let language = Language::from(detail.language);
        self.sum.bytes += detail.bytes;
        self.sum.blank += detail.blank;
        self.sum.comment += detail.comment;
        self.sum.code += detail.code;

        *self
            .kinds
            .entry(language)
            .or_insert_with(|| Detail::from_other(&detail)) += detail;
    }
}
