use std::{env, ffi::OsStr, fs};

use getopts::{Matches, Options};

fn parse_opts<C: IntoIterator>(args: C) -> Matches
where
    C::Item: AsRef<OsStr>,
{
    let mut opts = Options::new();
    opts.optflag("c", "--bytes", "counts bytes");
    opts.optflag("l", "--lines", "counts lines");
    opts.optflag("w", "--words", "counts words");
    opts.optflag("m", "--chars", "counts characters");

    let matches = match opts.parse(args) {
        Ok(m) => m,
        Err(f) => {
            panic!("{}", f.to_string())
        }
    };
    matches
}

fn print_output(counts: Vec<usize>, file_path: &str) {
    for count in counts {
        print!("{:<8}", count)
    }
    println!("{}", file_path);
}

fn count_bytes(content: &str) -> usize {
    content.as_bytes().len()
}

fn count_lines(content: &str) -> usize {
    content.lines().count()
}

fn count_words(content: &str) -> usize {
    content
        .lines()
        .fold(0, |acc, line| acc + line.split_whitespace().count())
}

fn main() {
    // argument parsing
    let args: Vec<String> = env::args().collect();

    let matches = parse_opts(&args[1..]);
    let file_path = matches.free.first().ok_or("No file path").unwrap();

    let content = fs::read_to_string(file_path).unwrap();

    let mut counts: Vec<usize> = Vec::new();

    if matches.opt_present("c") {
        counts.push(count_bytes(&content));
    }
    if matches.opt_present("l") {
        counts.push(count_lines(&content));
    }
    if matches.opt_present("w") {
        counts.push(count_words(&content));
    }
    if matches.opt_present("m") {
        counts.push(content.chars().count());
    }

    if counts.is_empty() {
        counts.push(count_bytes(&content));
        counts.push(count_lines(&content));
        counts.push(count_words(&content));
    }

    print_output(counts, file_path);
}
