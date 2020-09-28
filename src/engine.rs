use std::fs;
use std::path::PathBuf;
use std::thread::spawn;

use crate::calculator::Calculator;
use crate::explorer::Explorer;
use crate::reporter::{Report, Reporter};
use crate::spinner::Spinner;

pub struct Engine {
    entry: PathBuf,

    explorer: Explorer,
    calculator: Calculator,
    reporter: Reporter,
}

impl Engine {
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

    pub fn serve(self) -> Report {
        let Self {
            entry,
            mut explorer,
            calculator,
            reporter,
        } = self;
        let spinner = Spinner::new();

        spinner.start();
        spawn(move || explorer.walk_directory(entry));
        spawn(|| calculator.calculate());
        let report = reporter.research();
        spinner.stop();

        report
    }

    fn read_ignore_list(filename: PathBuf) -> Vec<PathBuf> {
        fs::read_to_string(filename)
            .map(|content| {
                content
                    .lines()
                    .filter_map(|path| fs::canonicalize(path).ok())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default()
    }
}
