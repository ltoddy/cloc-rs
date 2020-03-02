use structopt::StructOpt;

use cloc::detail::TotalDetail;
use cloc::engine::Engine;
use cloc::options::Options;
use cloc::pprint::PrettyPrinter;

fn main() {
    let opt: Options = Options::from_args();
    let Options { entry, output } = opt;

    let engine = Engine::new(entry);
    let (details, total_text_files, ignored_files) = engine.calculate();
    let total = TotalDetail::from_details(details);

    // TODO
    println!("total_text_files ==> {}", total_text_files);
    println!("ignored_files ==> {}", ignored_files);
    PrettyPrinter::terminal(total);
}
