#![allow(dead_code)]
#![allow(unused_variables)]

mod calculator;
mod detail;
mod error;
mod executor;
mod explorer;
mod machine;
mod macros;
mod options;
mod pprint;
mod prettyprinter;
mod reporter;
mod spinner;
mod util;

use std::env::current_dir;
use std::fs;

use structopt::StructOpt;

use crate::machine::AutomaticMachinery;
use crate::options::Options;

type Result<T> = std::result::Result<T, crate::error::Error>;

fn main() {
    let opt: Options = Options::from_args();
    let Options {
        output,
        sort_by,
        order_by,
        entry,
        ignore_file,
    } = opt;

    let entry = entry.and_then(|entry| fs::canonicalize(entry).ok()).unwrap_or_else(|| {
        eprintln!("No directory specified, so use current directory as entry.\n");
        current_dir().expect("current directory does not exist")
    });

    let machine = AutomaticMachinery::new(entry, ignore_file.unwrap_or_default());
    machine.startup();
}
