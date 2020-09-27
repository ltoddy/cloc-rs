use std::collections::HashMap;

#[derive(Debug)]
pub struct Report {
    pub languages: Vec<LanguageDetail>,
    pub sum: SumDetail,
}

impl Report {
    pub fn new(languages: Vec<LanguageDetail>, sum: SumDetail) -> Self {
        Self { languages, sum }
    }
}

/// 读取单个文件, 分析后得出的详情
#[derive(Copy, Clone, Debug)]
pub struct Detail {
    pub language: &'static str,
    pub bytes: u64,
    pub blank: usize,
    pub comment: usize,
    pub code: usize,
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
            .fold((0, 0, 0, 0), |acc, x| {
                (acc.0 + x.0, acc.1 + x.1, acc.2 + x.2, acc.3 + x.3)
            });

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
pub struct SumDetail {
    pub files: usize,
    pub bytes: u64,
    pub blank: usize,
    pub comment: usize,
    pub code: usize,
}

impl SumDetail {
    #[inline]
    pub fn zero() -> Self {
        Self {
            files: 0,
            bytes: 0,
            blank: 0,
            comment: 0,
            code: 0,
        }
    }
}

pub fn aggregate_details(details: Vec<Detail>) -> Report {
    let mut kinds = HashMap::<&str, Vec<Detail>>::new();
    let mut sum = SumDetail::zero();

    for detail in details {
        let Detail {
            language,
            bytes,
            blank,
            comment,
            code,
        } = detail;
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

    Report::new(
        kinds
            .iter()
            .map(|(language, details)| LanguageDetail::from_details(language, details))
            .collect(),
        sum,
    )
}
