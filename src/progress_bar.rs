use std::fmt::Display;
use std::io::{self, Write};

struct Begin;
struct Progressing;
struct Done;


struct ProgressBar<State = Begin> {
    index: usize,
    bar: Vec<char>,
    _state: std::marker::PhantomData<State>
}

impl ProgressBar<Begin> {
    fn new(size: usize) -> ProgressBar<Progressing> {
        let bar = {
            let mut bar: Vec<char> = vec![' '; size];
            bar[0] = '[';
            bar[1] = '>';
            bar[size - 1] = ']';
            bar
        };

        ProgressBar::<Progressing> { 
            index: 0, 
            bar,
            _state: std::marker::PhantomData
        }
    }
}

impl ProgressBar<Progressing> {
    fn update(mut self) -> Step<'static> {
        self.bar[self.index] = '=';
        self.bar[self.index + 1] = '>';
        self.index += 1;

        if self.index == self.bar.len() - 2 {
            Step::Last(ProgressBar::<Last> {
                index: self.index,
                bar: self.bar,
                _state: std::marker::PhantomData,
            })
        } else {
            Step::Progressing(&self)
        }
    }
}



impl Display for ProgressBar<Begin> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\r{}", self.bar.iter().collect::<String>())
    }
}

impl Display for ProgressBar<Progressing> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\r{}", self.bar.iter().collect::<String>())
    }
}

impl Display for ProgressBar<Done> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\r{}\n", self.bar.iter().collect::<String>())
    }
}

pub fn update_progress_bar(index: usize, bar: &mut [char]) {
    let last = bar.len() - 1;
    bar[index] = '=';
    if index != last - 1 {
        bar[index + 1] = '>';
    }
}

pub fn progress_index(cnt: usize, file_count: usize, bar: &[char]) -> usize {
    let width = bar.len() - 2;
    (cnt * width) / file_count.max(1)
}

pub fn flush_stdout() {
    if io::stdout().flush().is_ok() {}
}
