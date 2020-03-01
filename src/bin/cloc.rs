use structopt::StructOpt;

use cloc::engine::Engine;
use cloc::options::Options;

fn main() {
    let opt: Options = Options::from_args();

    let Options { entry } = opt;

    let engine = Engine::new(entry);
    let detail = engine.calculate();

    println!("{:?}", detail);
}
