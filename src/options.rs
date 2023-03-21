use std::path::PathBuf;
use std::str::FromStr;

use structopt::StructOpt;

use crate::error::Error;

#[derive(Debug)]
pub enum Output {
    Terminal,
    Markdown,
}

impl FromStr for Output {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        match s.as_str() {
            "terminal" => Ok(Output::Terminal),
            "markdown" => Ok(Output::Markdown),
            _ => Err(Error::InvalidArg(s)),
        }
    }
}

#[derive(Debug)]
pub enum SortBy {
    Language,
    Files,
    Size,
    Blank,
    Comment,
    Code,
}

impl FromStr for SortBy {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        match s.as_str() {
            "language" => Ok(SortBy::Language),
            "files" => Ok(SortBy::Files),
            "size" => Ok(SortBy::Size),
            "blank" => Ok(SortBy::Blank),
            "comment" => Ok(SortBy::Comment),
            "code" => Ok(SortBy::Code),
            _ => Err(Error::InvalidArg(s)),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum OrderBy {
    Asc,
    Desc,
}

impl FromStr for OrderBy {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        match s.as_str() {
            "asc" => Ok(OrderBy::Asc),
            "desc" => Ok(OrderBy::Desc),
            _ => Err(Error::InvalidArg(s)),
        }
    }
}

#[derive(StructOpt, Debug)]
#[structopt(
    name = "rcloc",
    author = "ltoddy - taoliu0509@gmail.com",
    about = r#"
        Count, or compute differences of, lines of source code and comments
        Contribute this project on Github: https://github.com/ltoddy/cloc-rs
    "#
)]
pub struct Options {
    #[structopt(
        long = "output",
        default_value = "Terminal",
        help = "alternative parameters: Terminal, Markdown\n"
    )]
    pub output: Output,

    #[structopt(
        long = "sort-by",
        default_value = "language",
        help = "alternative parameters: language, files, size, blank, comment, code\n"
    )]
    pub sort_by: SortBy,

    #[structopt(
        long = "order-by",
        default_value = "asc",
        help = "alternative parameters: asc, desc\n"
    )]
    pub order_by: OrderBy,

    #[structopt(name = "path", parse(from_os_str))]
    pub entry: Option<PathBuf>,

    // #[structopt(long = "ignore")]
    // pub ignore_path: Option<PathBuf>,
    #[structopt(long = "ignore-file")]
    pub ignore_file: Option<PathBuf>,
}
