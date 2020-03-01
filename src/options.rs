use std::path::PathBuf;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "cloc - Count, or compute differences of, lines of source code and comments.")]
pub struct Options {
    #[structopt(name = "path", parse(from_os_str))]
    pub entry: PathBuf,
}
