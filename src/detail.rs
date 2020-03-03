use std::collections::HashMap;
use std::ops::AddAssign;

use crate::Language;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Detail {
    pub language: &'static str,
    pub blank: i32,
    pub comment: i32,
    pub code: i32,
}

impl Detail {
    pub fn new(language: &'static str, blank: i32, comment: i32, code: i32) -> Self {
        Self {
            language,
            blank,
            comment,
            code,
        }
    }

    pub fn from_other(other: &Detail) -> Self {
        Self {
            language: other.language,
            blank: 0,
            comment: 0,
            code: 0,
        }
    }
}

impl AddAssign for Detail {
    fn add_assign(&mut self, rhs: Self) {
        self.blank += rhs.blank;
        self.comment += rhs.comment;
        self.code += rhs.code;
    }
}

#[derive(Debug, Default)]
pub struct SumDetail {
    pub blank: i32,
    pub comment: i32,
    pub code: i32,
}

#[derive(Debug)]
pub struct TotalDetail {
    pub inner: HashMap<Language, Detail>,
    pub sum: SumDetail,
}

impl TotalDetail {
    pub fn from_details(details: Vec<Detail>) -> Self {
        let mut total = Self {
            inner: HashMap::new(),
            sum: SumDetail::default(),
        };

        for detail in details {
            total.add(detail);
        }

        total
    }

    fn add(&mut self, detail: Detail) {
        let language = Language::from(detail.language);
        self.sum.blank += detail.blank;
        self.sum.comment += detail.comment;
        self.sum.code += detail.code;

        *self.inner.entry(language).or_insert(Detail::from_other(&detail)) += detail;
    }
}
