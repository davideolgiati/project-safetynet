//! Benchmarks `filter_files`, the hot loop behind every archive run.
//!
//! It is O(files x patterns): every file is tested against every include rule,
//! then against every exclude rule. So the two axes worth measuring are file
//! count and pattern count. If the ns/file column grows with `pat`, a
//! `RegexSet` (one pass over all patterns) is the upgrade path.
//!
//! Run with `cargo bench`.

// ponytail: the crate is a binary, so there is no lib to link against.
// Including the module source beats adding a lib target just for this.
#[path = "../src/file_filter.rs"]
#[allow(dead_code)]
mod file_filter;

use file_filter::{build_regex_registry, filter_files};
use regex::Regex;
use std::hint::black_box;
use std::time::{Duration, Instant};

const EXTS: [&str; 8] = ["rs", "pdf", "txt", "md", "json", "png", "tar.gz", "log"];

/// Paths shaped like a real tree: nested dirs, mixed extensions, and every 17th
/// name hitting the exclude rule from `config.json`.
fn sample_files(n: usize) -> Vec<String> {
    (0..n)
        .map(|i| {
            let ext = EXTS[i % EXTS.len()];
            let name = if i % 17 == 0 { "contratto" } else { "documento" };
            format!("/home/user/docs/{}/{}/{name}_{i}.{ext}", i % 13, i % 7)
        })
        .collect()
}

/// Compiles through the same path production uses, so the `(?i)` wrapper is measured too.
fn rules<S: ToString>(patterns: impl IntoIterator<Item = S>) -> Vec<Regex> {
    build_regex_registry(
        &Some(patterns.into_iter().map(|p| p.to_string()).collect()),
        vec![],
    )
}

/// The empty set: matches nothing.
fn none() -> Vec<Regex> {
    rules::<&str>([])
}

fn bench(label: &str, files: &[String], include: &[Regex], exclude: &[Regex], iters: usize) {
    let mut best = Duration::MAX;
    let mut kept = 0;

    for _ in 0..iters {
        let input = files.to_vec(); // untimed: filter_files consumes its input
        let start = Instant::now();
        let out = filter_files(input, include, exclude);
        best = best.min(start.elapsed());
        kept = black_box(&out).len(); // keeps `out` alive so its drop stays untimed
    }

    let ns_per_file = best.as_nanos() as f64 / files.len() as f64;
    println!(
        "{label:<26} n={:<9} pat={:<3} kept={kept:<8} {:>10.2?} {ns_per_file:>7.1} ns/file",
        files.len(),
        include.len() + exclude.len(),
        best
    );
}

/// Fails loudly if the generator or the filter itself breaks, so the timings
/// below are never for the wrong work.
fn self_check() {
    let files = sample_files(40);
    // .pdf is EXTS[1], so pdfs land on i = 1, 9, 17, 25, 33; i=17 is a contratto.
    let kept = filter_files(files.clone(), &rules([r".*\.pdf"]), &rules([r".*contratto.*"]));
    let want: Vec<String> = [1, 9, 25, 33].iter().map(|&i| files[i].clone()).collect();
    assert_eq!(kept, want);

    assert_eq!(
        filter_files(files.clone(), &rules([r".*\.PDF"]), &none()).len(),
        5,
        "build_regex_registry must apply (?i)"
    );
    assert!(
        filter_files(files, &rules::<&str>([]), &rules::<&str>([])).is_empty(),
        "an empty include list keeps nothing"
    );
}

fn main() {
    self_check();

    let include = rules([r".*\.pdf"]);
    let exclude = rules([r".*contratto.*\.pdf"]);

    println!("\n-- file count scaling (config.json rules) --");
    for n in [10_000, 100_000, 1_000_000] {
        let iters = if n >= 1_000_000 { 3 } else { 20 };
        bench("include + exclude", &sample_files(n), &include, &exclude, iters);
    }

    let files = sample_files(100_000);

    println!("\n-- pattern count scaling (pat = include + exclude) --");
    for k in [1usize, 4, 16] {
        // k-1 patterns that match nothing, plus the real one: worst case for the
        // per-file linear scan, since every miss costs a full regex run.
        let mut patterns: Vec<String> = (1..k).map(|j| format!(r".*\.ext{j}$")).collect();
        patterns.push(r".*\.pdf".to_string());
        bench("misses + 1 hit", &files, &rules(&patterns), &exclude, 20);
    }

    println!("\n-- production default (config omits include/exclude) --");
    bench("include=.* exclude=none", &files, &rules([".*"]), &rules::<&str>([]), 20);
}