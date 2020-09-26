use std::fs;
use std::path::PathBuf;

use crate::calculator::Calculator;
use crate::explorer::Explorer;
use std::thread::spawn;

pub struct AutomaticMachinery {
    entry: PathBuf,

    explorer: Explorer,
    calculator: Calculator,
}

impl AutomaticMachinery {
    pub fn new(entry: PathBuf, ignore_file: PathBuf) -> Self {
        let ignore_list = Self::read_ignore_list(ignore_file);

        let (explorer, filename_receiver) = Explorer::new(ignore_list);
        let (calculator, detail_receiver) = Calculator::new(filename_receiver);

        Self {
            entry,
            explorer,
            calculator,
        }
    }

    pub fn startup(self) {
        let Self {
            entry,
            mut explorer,
            calculator,
        } = self;

        spawn(move || explorer.walk_directory(entry));
        calculator.serve();
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
