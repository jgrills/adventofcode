use std::fs::read_to_string;
use std::collections::BTreeMap;

fn split_with_char(input: &str, with: char) -> (&str, &str) {
    match input.find(with) {
        Some(index) => (&input[0..index], &input[index+2..]),
        None => (input, "")
    }
}

fn fixable(pattern_order : &Vec<&str>, patterns : &BTreeMap<&str, usize>, text : &str) -> usize {
    if text.is_empty() {
        1
    } else {
        for prefix in pattern_order {
            if !patterns.contains_key(prefix) { continue; }
            if let Some(remain) = text.strip_prefix(prefix) {
                let local = fixable(pattern_order, patterns, remain); 
                let count = patterns[prefix];
                let result = local * count;
                if result !=0 { println!(" {} {} {} {} {} {}", result, local, count, text, prefix, remain); return result; }
            }
        }
        0
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

    let mut patterns : [Vec<&str>; 16] = Default::default();
    let mut patterns_line = lines.next().unwrap();
    while !patterns_line.is_empty() {
        let pat;
        (pat, patterns_line) = split_with_char(patterns_line, ',');
        patterns[pat.len()].push(pat);
    }

    let mut pattern_order : Vec<&str> = Vec::new();
    for i in (0..16).rev() {
        for p in &patterns[i] {
            pattern_order.push(*p);
            println!("pattern {} len {}", p, i);
        }
    }

    let mut pattern_map : BTreeMap<&str,usize> = BTreeMap::new();
    for p in pattern_order.iter().rev() {
        let cnt = fixable(&pattern_order, &pattern_map, p) + 1;
        pattern_map.insert(p, cnt);
        println!("pattern {} cnt {}", p, cnt);
    }

    lines.next();
    let mut works = 0;
    while let Some(line) = lines.next() {
        let this = fixable(&pattern_order, &pattern_map, line);
        works += this;
        println!("test: {} {} {}", this, works, line);
    }
    println!("total: {}", works);

}
