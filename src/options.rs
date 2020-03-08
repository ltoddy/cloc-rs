use std::path::PathBuf;
use std::str::FromStr;

use structopt::StructOpt;

use crate::error::ClocError;

#[derive(Debug)]
pub enum Output {
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
            _ => Err(ClocError::InvalidCommandArgs),
        }
    }
}

#[derive(Debug)]
pub enum SortBy {
    Name,
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
            "name" => Ok(SortBy::Name),
            "size" => Ok(SortBy::Size),
            "blank" => Ok(SortBy::Blank),
            "comment" => Ok(SortBy::Comment),
            "code" => Ok(SortBy::Code),
            _ => Err(ClocError::InvalidCommandArgs),
        }
    }
}

#[derive(StructOpt, Debug)]
#[structopt(
    name = "cloc - Count, or compute differences of, lines of source code and comments.",
    author = "ltoddy - toddy.liu@outlook.com"
)]
pub struct Options {
    #[structopt(
        short = "o",
        long = "output",
        default_value = "Terminal",
        help = "alternative parameters(ignore case): Terminal, Markdown"
    )]
    pub output: Output,

    #[structopt(
        long = "sort-by",
        default_value = "name",
        help = "alternative parameters(ignore case): name, size, blank, comment, code"
    )]
    pub sort_by: SortBy,

    #[structopt(name = "path", parse(from_os_str))]
    pub entry: PathBuf,
}
