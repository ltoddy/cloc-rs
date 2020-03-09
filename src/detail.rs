use std::collections::HashMap;
use std::ops::AddAssign;

/// 读取单个文件, 分析后得出的详情
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Detail {
    // TODO: rename  language -> name
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

    // TODO: rename
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

/// 基于语言分类之后的详情
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LanguageDetail {
    pub language: &'static str,
    pub bytes: u64,
    pub code: usize,
    pub comment: usize,
    pub blank: usize,
}

impl LanguageDetail {
    fn from_detail_by_default(detail: Detail) -> Self {
        Self {
            language: detail.language,
            bytes: detail.bytes,
            code: detail.code,
            comment: detail.comment,
            blank: detail.blank,
        }
    }

    fn add_detail(&mut self, detail: Detail) {
        assert_eq!(self.language, detail.language);

        self.bytes += detail.bytes;
        self.code += detail.code;
        self.comment += detail.comment;
        self.blank += detail.blank;
    }
}

#[derive(Debug, Default)]
pub struct SumDetail {
    pub bytes: u64,
    pub blank: usize,
    pub comment: usize,
    pub code: usize,
}

pub fn aggregate_details(details: &[Detail]) -> (Vec<LanguageDetail>, SumDetail) {
    let mut kinds = HashMap::new();

    let mut sum = SumDetail::default();

    for detail in details {
        let &Detail {
            language,
            bytes,
            blank,
            comment,
            code,
        } = detail;
        sum.bytes += bytes;
        sum.blank += blank;
        sum.comment += comment;
        sum.code += code;

        kinds
            .entry(language)
            .and_modify(|d: &mut LanguageDetail| d.add_detail(*detail))
            .or_insert_with(|| LanguageDetail::from_detail_by_default(*detail));
    }

    (kinds.values().cloned().collect(), sum)
}
