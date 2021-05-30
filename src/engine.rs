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
    pub fn new(entry: PathBuf, ignore_file: Option<PathBuf>) -> Self {
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
        #[rustfmt::skip]
        let Self { entry, mut explorer, calculator, reporter } = self;
        let spinner = Spinner::new();

        spinner.start();
        spawn(move || explorer.walk_directory(entry));
        spawn(|| calculator.calculate());
        let report = reporter.research();
        spinner.stop();

        report
    }

    #[rustfmt::skip]
    fn read_ignore_list(filename: Option<PathBuf>) -> Option<Vec<PathBuf>> {
        filename
            .and_then(|filename| fs::read_to_string(filename).ok())
            .map(|content| content.lines().map(|line| line.trim_start_matches("/")).filter_map(|path| fs::canonicalize(path).ok()).collect())
    }
}
