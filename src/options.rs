use std::path::PathBuf;
use std::str::FromStr;

use structopt::StructOpt;

use crate::error::ClocError;

#[derive(Debug)]
pub(crate) enum Output {
    Terminal,
    Markdown,
}

impl FromStr for Output {
    type Err = ClocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        match s.as_str() {
            "terminal" => Ok(Output::Terminal),
            "markdown" => Ok(Output::Markdown),
            _ => Err(ClocError::InvalidArg(s)),
        }
    }
}

#[derive(Debug)]
pub(crate) enum SortBy {
    Language,
    Files,
    Size,
    Blank,
    Comment,
    Code,
}

impl FromStr for SortBy {
    type Err = ClocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        match s.as_str() {
            "language" => Ok(SortBy::Language),
            "files" => Ok(SortBy::Files),
            "size" => Ok(SortBy::Size),
            "blank" => Ok(SortBy::Blank),
            "comment" => Ok(SortBy::Comment),
            "code" => Ok(SortBy::Code),
            _ => Err(ClocError::InvalidArg(s)),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum OrderBy {
    Asc,
    Desc,
}

impl FromStr for OrderBy {
    type Err = ClocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        match s.as_str() {
            "asc" => Ok(OrderBy::Asc),
            "desc" => Ok(OrderBy::Desc),
            _ => Err(ClocError::InvalidArg(s)),
        }
    }
}

#[derive(StructOpt, Debug)]
#[structopt(
    name = "cloc - Count, or compute differences of, lines of source code and comments",
    author = "ltoddy - toddy.liu@outlook.com"
)]
pub(crate) struct Options {
    #[structopt(
        long = "output",
        default_value = "Terminal",
        help = "alternative parameters: Terminal, Markdown\n"
    )]
    pub(crate) output: Output,

    #[structopt(
        long = "sort-by",
        default_value = "language",
        help = "alternative parameters: language, files, size, blank, comment, code\n"
    )]
    pub(crate) sort_by: SortBy,

    #[structopt(
        long = "order-by",
        default_value = "asc",
        help = "alternative parameters: asc, desc\n"
    )]
    pub(crate) order_by: OrderBy,

    #[structopt(name = "path", parse(from_os_str))]
    pub(crate) entry: Option<PathBuf>,
}
