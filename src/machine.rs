use std::fs;
use std::path::PathBuf;
use std::thread::spawn;

use crate::calculator::Calculator;
use crate::explorer::Explorer;
use crate::prettyprinter::pretty_print;
use crate::reporter::Reporter;

pub struct AutomaticMachinery {
    entry: PathBuf,

    explorer: Explorer,
    calculator: Calculator,
    reporter: Reporter,
}

impl AutomaticMachinery {
    pub fn new(entry: PathBuf, ignore_file: PathBuf) -> Self {
        let ignore_list = Self::read_ignore_list(ignore_file);

        let (explorer, filename_receiver) = Explorer::new(ignore_list);
        let (calculator, detail_receiver) = Calculator::new(filename_receiver);
        let reporter = Reporter::new(detail_receiver);

        Self {
            entry,
            explorer,
            calculator,
            reporter,
        }
    }

    pub fn startup(self) {
        let Self {
            entry,
            mut explorer,
            calculator,
            reporter,
        } = self;

        spawn(move || explorer.walk_directory(entry));
        spawn(|| calculator.serve());
        let report = reporter.research();
        pretty_print(report);
    }

    fn read_ignore_list(filename: PathBuf) -> Vec<PathBuf> {
        fs::read_to_string(filename)
            .map(|content| {
                content
                    .lines()
                    .filter_map(|path| fs::canonicalize(path).ok())
                    .collect::<Vec<_>>()
            })
            .unwrap_or(Vec::new())
    }
}
