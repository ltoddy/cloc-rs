use std::ops::Add;

#[derive(Debug, PartialEq)]
pub struct Detail {
    language: &'static str,
    blank: i32,
    comment: i32,
    code: i32,
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
            code: self.code + rhs.comment,
        }
    }
}
