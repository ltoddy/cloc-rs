use std::cmp::Ordering;
use std::time;

use structopt::StructOpt;

use cloc::detail::aggregate_details;
use cloc::engine::Engine;
use cloc::options::{Options, Output, SortBy};
use cloc::pprint::PrettyPrinter;

fn main() {
    let opt: Options = Options::from_args();
    let Options { output, sort_by, entry } = opt;

    let engine = Engine::new(entry);
    let now = time::Instant::now();
    let (details, total_text_files, ignored_files) = engine.calculate();
    let (mut languages, sum) = aggregate_details(details);

    languages.sort_by(|prev, next| match sort_by {
        SortBy::Name => compare(prev.language, next.language),
        SortBy::Size => compare(prev.bytes, next.bytes),
        SortBy::Blank => compare(prev.blank, next.blank),
        SortBy::Comment => compare(prev.comment, next.comment),
        SortBy::Code => compare(prev.code, next.code),
    });
    let elapsed = now.elapsed();

    match output {
        Output::Terminal => PrettyPrinter::terminal(languages, sum, total_text_files, ignored_files, elapsed),
        Output::Markdown => PrettyPrinter::markdown(languages, sum, total_text_files, ignored_files, elapsed),
    }
}

// TODO: move to order file
fn compare<T: PartialOrd>(t1: T, t2: T) -> Ordering {
    if t1 < t2 {
        Ordering::Less
    } else if t1 > t2 {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}
