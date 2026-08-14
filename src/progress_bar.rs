use std::io::{self, Write};

pub fn new_progress_bar(size: usize) -> Vec<char> {
    let mut bar: Vec<char> = vec![' '; size];
    bar[0] = '[';
    bar[1] = '>';
    bar[size - 1] = ']';
    bar
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

pub fn display_progress_bar(bar: &[char]) {
    let last = bar.len() - 1;
    
    if bar[last - 1] == '=' {
        println!("\r{}", bar.iter().collect::<String>());
    } else {
        print!("\r{}", bar.iter().collect::<String>());
    }

    flush_stdout();
}
