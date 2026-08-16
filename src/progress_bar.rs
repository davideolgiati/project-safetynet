use std::fmt::Display;
use std::io::{self, Write};

pub struct ProgressBar {
    bar: Vec<char>,
    seen: usize,
    ratio: f64,
    refresh: bool
}

impl ProgressBar {
    pub fn new(cols: usize, population: usize) -> ProgressBar {
        let bar = {
            let mut bar: Vec<char> = vec![' '; cols];
            bar[0] = '[';
            bar[1] = '>';
            bar[cols - 1] = ']';
            bar
        };

        ProgressBar { 
            bar,
            seen: 0,
            ratio: (cols - 2) as f64 / population as f64,
            refresh: true
        }
    }

    fn index(&self) -> usize {
        ((self.seen as f64 * self.ratio) as usize).min(self.bar.len() - 2)
    }

    pub fn progress(&mut self) {
        let current_idx = self.index();

        self.seen += 1;

        let index = self.index();

        if index == current_idx {
            return
        }

        self.bar[index] = '=';

        if index != self.bar.len() - 2 {
            self.bar[index + 1] = '>';
        }

        self.refresh = true
    }

    pub fn show(&mut self) {
        if !self.refresh {
            return
        }

        let index = self.index();

        if index != self.bar.len() - 2 {
            print!("\r{}", self)
        } else {
            println!("\r{}", self)
        }

        let _ = io::stdout().flush().is_ok();

        self.refresh = false;
    }
}

impl Display for ProgressBar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.bar.iter().collect::<String>())
    }
}
