use std::{env, fs};
use std::fs::File;
use std::io::{BufRead, BufReader};

use getopts::Options;

fn main() {
    // argument parsing
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optflag("c", "", "counts bytes");
    opts.optflag("l", "", "counts lines");
    opts.optflag("w", "", "counts words");
    opts.optflag("m", "", "counts characters");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            panic!("{}", f.to_string())
        }
    };

    let file_path = matches.free.first().ok_or("No file path").unwrap();

    let content = fs::read_to_string(file_path).unwrap();

    let mut opts_present = false;
    if matches.opt_present("c") {
        opts_present = true;
        print!("{} ", count_bytes(&content));
    }
    if matches.opt_present("l") {
        opts_present = true;
        print!("{} ", count_lines(&content));
    }
    if matches.opt_present("w") {
        opts_present = true;
        print!("{} ", count_words(&content));
    }
    if matches.opt_present("m") {
        opts_present = true;
        print!("{} ", count_chars(&content));
    }
    if !opts_present {
        print!(
            "{} {} {} ",
            count_bytes(&content),
            count_lines(&content),
            count_words(&content)
        );
    }

    print!("{}\n", file_path);
}

fn count_bytes(content: &str) -> usize {
    content.as_bytes().iter().count()
}

fn count_lines(content: &str) -> usize {
    return 0;
}

fn count_words(content: &str) -> usize {
    return 0;
}

fn count_chars(content: &str) -> usize {
    content.chars().count()
}
