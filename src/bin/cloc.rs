use std::time;

use structopt::StructOpt;

use cloc::detail::aggregate_details;
use cloc::engine::Engine;
use cloc::options::{Options, Output};
use cloc::pprint::PrettyPrinter;

fn main() {
    let opt: Options = Options::from_args();
    let Options { entry, output, .. } = opt;

    let engine = Engine::new(entry);
    let now = time::Instant::now();
    let (details, total_text_files, ignored_files) = engine.calculate();
    let (languages, sum) = aggregate_details(details);
    let elapsed = now.elapsed();

    match output {
        Output::Terminal => PrettyPrinter::terminal(languages, sum, total_text_files, ignored_files, elapsed),
        Output::Markdown => PrettyPrinter::markdown(languages, sum, total_text_files, ignored_files, elapsed),
    }
}
