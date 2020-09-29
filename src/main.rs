mod calculator;
mod engine;
mod error;
mod executor;
mod explorer;
mod options;
mod pretty_printer;
mod reporter;
mod spinner;
mod util;

use std::env::current_dir;
use std::fs;
use std::time::Instant;

use structopt::StructOpt;

use crate::engine::Engine;
use crate::options::{Options, SortBy};
use crate::pretty_printer::pretty_print;
use crate::util::compare;

type Result<T> = std::result::Result<T, crate::error::Error>;

fn main() {
    let opt: Options = Options::from_args();
    let Options {
        sort_by,
        order_by,
        entry,
        ignore_file,
        ..
    } = opt;

    let entry = entry.and_then(|entry| fs::canonicalize(entry).ok()).unwrap_or_else(|| {
        eprintln!("No directory specified, so use current directory as entry.\n");
        current_dir().expect("current directory does not exist")
    });

    let now = Instant::now();
    let machine = Engine::new(entry, ignore_file);
    let mut report = machine.serve();
    report.sections.sort_by(|prev, next| match sort_by {
        SortBy::Language => compare(prev.language, next.language, order_by),
        SortBy::Files => compare(prev.files, next.files, order_by),
        SortBy::Size => compare(prev.bytes, next.bytes, order_by),
        SortBy::Blank => compare(prev.blank, next.blank, order_by),
        SortBy::Comment => compare(prev.comment, next.comment, order_by),
        SortBy::Code => compare(prev.code, next.code, order_by),
    });

    let elapsed = now.elapsed();

    pretty_print(report, elapsed);
}
