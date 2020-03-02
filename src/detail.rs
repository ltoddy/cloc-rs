use std::collections::HashMap;
use std::ops::Add;

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
}

impl Add for Detail {
    type Output = Detail;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            language: self.language,
            blank: self.blank + rhs.blank,
            comment: self.comment + rhs.comment,
            code: self.code + rhs.code,
        }
    }
}

#[derive(Debug)]
pub struct TotalDetail {
    pub inner: HashMap<Language, Detail>,
}

impl TotalDetail {
    pub fn from_details(details: Vec<Detail>) -> Self {
        let mut total = Self { inner: HashMap::new() };

        for detail in details {
            total.add(detail);
        }

        total
    }

    fn add(&mut self, detail: Detail) {
        let language = Language::from(detail.language);

        if let Some(d) = self.inner.get(&language) {
            self.inner.insert(language, *d + detail);
        } else {
            self.inner.insert(language, detail);
        }
    }
}
