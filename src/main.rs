mod calculate;
mod config;
mod detail;
mod engine;
mod error;
mod executor;
mod macros;
mod options;
mod pprint;
mod spinner;
mod util;

use std::env::current_dir;
use std::fs;
use std::time;

use structopt::StructOpt;

use crate::engine::Engine;
use crate::options::{Options, Output, SortBy};
use crate::pprint::PrettyPrinter;
use crate::spinner::Spinner;
use crate::util::compare;

type Result<T> = std::result::Result<T, crate::error::Error>;

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e)
    }
}

fn run() -> Result<()> {
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

    let spinner = Spinner::new();
    let mut engine = Engine::new(entry, ignore_file);
    let now = time::Instant::now();
    spinner.start();
    let mut report = engine.calculate();

    report.languages.sort_by(|prev, next| match sort_by {
        SortBy::Language => compare(prev.language, next.language, order_by),
        SortBy::Files => compare(prev.files, next.files, order_by),
        SortBy::Size => compare(prev.bytes, next.bytes, order_by),
        SortBy::Blank => compare(prev.blank, next.blank, order_by),
        SortBy::Comment => compare(prev.comment, next.comment, order_by),
        SortBy::Code => compare(prev.code, next.code, order_by),
    });
    let elapsed = now.elapsed();
    spinner.stop();

    match output {
        Output::Terminal => PrettyPrinter::terminal(report, elapsed),
        Output::Markdown => PrettyPrinter::markdown(report, elapsed),
    }

    Ok(())
}
