use std::collections::HashMap;

/// 读取单个文件, 分析后得出的详情
#[derive(Copy, Clone, Debug)]
pub(crate) struct Detail {
    pub(crate) language: &'static str,
    pub(crate) bytes: u64,
    pub(crate) blank: usize,
    pub(crate) comment: usize,
    pub(crate) code: usize,
}

impl Detail {
    #[inline]
    pub fn new(language: &'static str, bytes: u64, blank: usize, comment: usize, code: usize) -> Self {
        Self {
            language,
            bytes,
            blank,
            comment,
            code,
        }
    }
}

/// 基于语言分类之后的详情
#[derive(Debug)]
pub struct LanguageDetail {
    pub language: &'static str,
    pub files: usize,
    pub bytes: u64,
    pub blank: usize,
    pub comment: usize,
    pub code: usize,
}

impl LanguageDetail {
    fn from_details(language: &'static str, details: &[Detail]) -> Self {
        let (bytes, blank, comment, code) = details
            .iter()
            .map(|detail| (detail.bytes, detail.blank, detail.comment, detail.code))
            .fold((0, 0, 0, 0), |acc, x| (acc.0 + x.0, acc.1 + x.1, acc.2 + x.2, acc.3 + x.3));

        Self {
            language,
            files: details.len(),
            bytes,
            blank,
            comment,
            code,
        }
    }
}

#[derive(Debug)]
pub(crate) struct SumDetail {
    pub(crate) files: usize,
    pub(crate) bytes: u64,
    pub(crate) blank: usize,
    pub(crate) comment: usize,
    pub(crate) code: usize,
}

impl SumDetail {
    #[inline]
    pub(crate) fn zero() -> Self {
        Self {
            files: 0,
            bytes: 0,
            blank: 0,
            comment: 0,
            code: 0,
        }
    }
}

pub(crate) fn aggregate_details(details: Vec<Detail>) -> (Vec<LanguageDetail>, SumDetail) {
    let mut kinds = HashMap::<&str, Vec<Detail>>::new();
    let mut sum = SumDetail::zero();

    for detail in details {
        let Detail { language, bytes, blank, comment, code } = detail;
        sum.files += 1;
        sum.bytes += bytes;
        sum.blank += blank;
        sum.comment += comment;
        sum.code += code;

        kinds
            .entry(language)
            .and_modify(|ds| ds.push(detail))
            .or_insert_with(|| vec![detail]);
    }

    (
        kinds
            .iter()
            .map(|(language, details)| LanguageDetail::from_details(language, details))
            .collect(),
        sum,
    )
}
