use structopt::StructOpt;

use cloc::detail::TotalDetail;
use cloc::engine::Engine;
use cloc::options::{Options, Output};
use cloc::pprint::PrettyPrinter;

fn main() {
    let opt: Options = Options::from_args();
    let Options { entry, output } = opt;

    let engine = Engine::new(entry);
    let (details, total_text_files, ignored_files) = engine.calculate();
    let total = TotalDetail::from_details(details);

    match output {
        Output::Terminal => PrettyPrinter::terminal(total, total_text_files, ignored_files),
    }
}
