use std::fs::read_to_string;
use std::collections::BTreeSet;

fn split_with_char(input: &str, with: char) -> (&str, &str) {
    match input.find(with) {
        Some(index) => (&input[0..index], &input[index+2..]),
        None => (input, "")
    }
}

fn fixable(patterns : &BTreeSet<&str>, text : &str, skip : &str) -> bool {
    if text.is_empty() {
        true
    } else {
        for prefix in patterns {
            if *prefix == skip { continue; }
            if let Some(remain) = text.strip_prefix(prefix) {
                // println!("  {} {} {}", text, prefix, remain);
                if fixable(patterns, remain, skip) {
                    return true;
                }
            }
        }
        false
    }
}

fn main() {
    let use_example = 0;
    let filename;
    if use_example != 0 {
        filename = "/home/jgrills/adventofcode/2024/19/example.txt";
    } else {
        filename = "/home/jgrills/adventofcode/2024/19/input.txt";
    };
    let file_string = read_to_string(filename).unwrap();
    let mut lines = file_string.lines();

    let mut patterns = BTreeSet::new();
    let mut skip = BTreeSet::new();
    let mut patterns_line = lines.next().unwrap();
    while !patterns_line.is_empty() {
        patterns = patterns.difference(&skip).cloned().collect();
        let pat;
        (pat, patterns_line) = split_with_char(patterns_line, ',');
        if fixable(&patterns, pat, "") {
            skip.insert(pat);
        } else {
            patterns.insert(pat);
            for p in patterns.iter() {
                if fixable(&patterns, p, p) {
                    skip.insert(p);
                }
            }
        }
    }

    println!("patterns {} skip {}", patterns.len(), skip.len());
    lines.next();
    let mut works = 0;
    while let Some(line) = lines.next() {
        if fixable(&patterns, line, "") {
            works += 1;
            println!("works: {} {}", works, line);
        } else {
            println!("fails: {}", line);
        }
    }
    println!("works: {}", works);

}
