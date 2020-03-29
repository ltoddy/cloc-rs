use std::time;

use structopt::StructOpt;

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

use crate::engine::Engine;
use crate::options::{Options, Output, SortBy};
use crate::pprint::PrettyPrinter;
use crate::spinner::Spinner;
use crate::util::compare;

pub(crate) type ClocResult<T> = std::result::Result<T, crate::error::ClocError>;

fn main() {
    let opt: Options = Options::from_args();
    let Options {
        output,
        sort_by,
        order_by,
        entry,
    } = opt;

    let spinner = Spinner::new();
    let mut engine = Engine::new(entry);
    let now = time::Instant::now();
    spinner.start();
    let (mut languages, sum) = engine.calculate();

    languages.sort_by(|prev, next| match sort_by {
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
        Output::Terminal => PrettyPrinter::terminal(languages, sum, elapsed),
        Output::Markdown => PrettyPrinter::markdown(languages, sum, elapsed),
    }
}
