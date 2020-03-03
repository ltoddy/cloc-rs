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
        match s {
            "Terminal" => Ok(Output::Terminal),
            "Markdown" => Ok(Output::Markdown),
            _ => todo!(),
        }
    }
}

impl ToString for Output {
    fn to_string(&self) -> String {
        match self {
            Output::Terminal => String::from("Terminal"),
            Output::Markdown => String::from("Markdown"),
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
        help = "alternative parameters: Terminal, Markdown"
    )]
    pub output: Output,

    #[structopt(name = "path", parse(from_os_str))]
    pub entry: PathBuf,
}
