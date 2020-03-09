use std::time;

use structopt::StructOpt;

use cloc::detail::aggregate_details;
use cloc::engine::Engine;
use cloc::options::{Options, Output, SortBy};
use cloc::pprint::PrettyPrinter;
use cloc::util::compare;

fn main() {
    let opt: Options = Options::from_args();
    let Options { output, sort_by, entry } = opt;

    let engine = Engine::new(entry);
    let now = time::Instant::now();
    let details = engine.calculate();
    let (mut languages, sum) = aggregate_details(&details);

    languages.sort_by(|prev, next| match sort_by {
        SortBy::Name => compare(prev.language, next.language),
        SortBy::Size => compare(prev.bytes, next.bytes),
        SortBy::Blank => compare(prev.blank, next.blank),
        SortBy::Comment => compare(prev.comment, next.comment),
        SortBy::Code => compare(prev.code, next.code),
    });
    let elapsed = now.elapsed();

    match output {
        Output::Terminal => PrettyPrinter::terminal(languages, sum, elapsed),
        Output::Markdown => PrettyPrinter::markdown(languages, sum, elapsed),
    }
}
