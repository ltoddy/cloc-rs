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
