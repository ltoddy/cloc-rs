use structopt::StructOpt;

use cloc::detail::TotalDetail;
use cloc::engine::Engine;
use cloc::options::Options;
use cloc::pprint::PrettyPrinter;

fn main() {
    let opt: Options = Options::from_args();
    let Options { entry, output } = opt;

    let engine = Engine::new(entry);
    let details = engine.calculate();
    let total = TotalDetail::from_details(details);

    PrettyPrinter::terminal(total);
}
