use std::fs::read_to_string;
use std::str::FromStr;

fn main() {

    let use_example = false;
    let filename =
        if use_example {
            "/home/jgrills/adventofcode/2024/1/example.txt"
        } else {
            "/home/jgrills/adventofcode/2024/1/input.txt"
        };

    let file_string = read_to_string(filename).unwrap();

    let mut column0 = Vec::new();
    let mut column1 = Vec::new();

    for line in file_string.lines() {
        let mut tokens = line.split_ascii_whitespace();
        let tok0 = i64::from_str(tokens.next().unwrap()).unwrap();
        let tok1 = i64::from_str(tokens.next().unwrap()).unwrap();
        println!("{} : {} {}", line, tok0, tok1);
        column0.push(tok0);
        column1.push(tok1);
    }

    column0.sort();
    column1.sort();

    println!("sorted pairs:");

    let mut total_sim = 0;
    for i0 in column0.iter() {
        let v0 = *i0;
        let mut right_count = 0;
        for i1 in column1.iter() {
            let v1 = *i1;
            if v0 == v1 {
                right_count += 1;
            }
        }
        let left_value = v0 * right_count;
        total_sim += left_value;
        println!(" {} * {} = {} >> {}", v0, right_count, left_value, total_sim);
    }
}
