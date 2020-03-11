use std::time;

use structopt::StructOpt;

use cloc::detail::aggregate_details;
use cloc::engine::Engine;
use cloc::options::{Options, Output, SortBy};
use cloc::pprint::PrettyPrinter;
use cloc::spinner::Spinner;
use cloc::util::compare;

fn main() {
    let opt: Options = Options::from_args();
    let Options { output, sort_by, order_by, entry } = opt;

    let spinner = Spinner::new();
    let engine = Engine::new(entry);
    let now = time::Instant::now();
    spinner.start();
    let details = engine.calculate();
    let (mut languages, sum) = aggregate_details(&details);

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
