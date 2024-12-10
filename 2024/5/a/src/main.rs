use std::fs::read_to_string;
use std::str::FromStr;

fn split_with_char(input: &str, with: char) -> (&str, &str) {
    match input.find(with) {
        Some(index) => (&input[0..index], &input[index+1..]),
        None => (input, "")
    }
}

fn main() {
    let use_example = 0;
    let filename;
    if use_example != 0 {
        filename = "/home/jgrills/adventofcode/2024/5/example.txt";
    } else {
        filename = "/home/jgrills/adventofcode/2024/5/input.txt";
    };
    let file_string = read_to_string(filename).unwrap();

    let mut ordering = true;
    let mut depends = [[false; 100]; 100];

    let mut numbers = Vec::<usize>::new();
    let mut total = 0;
    for line in file_string.lines() {
        if line.is_empty() {
            ordering = false;
            continue;
        }
        if ordering {
            let first = usize::from_str(&line[0..2]).unwrap();
            let second = usize::from_str(&line[3..5]).unwrap();
            depends[second][first] = true;
            println!("O: {} | {}", first, second);
        }
        else {
            numbers.clear();
            let depends_on = | before:usize, after:usize | -> bool {
                depends[after][before]
            };

            println!("T: {}", line);
            let mut remain : &str = line;
            let mut valid = true;
            'line_valid: while !remain.is_empty() {
                let (item, rest) = split_with_char(remain, ',');
                let i = usize::from_str(item).unwrap();
                print!(" {}", i);
                for n in &numbers {
                    if depends_on(i, *n) {
                        valid = false;
                        print!(" invalid with {}", n);
                        break 'line_valid;
                    }
                }
                numbers.extend([i]);
                remain = rest;
            }
            if valid {
                let mid = numbers[numbers.len() / 2];
                total += mid;
                print!(" valid {} >> {}", mid, total);
            }

            println!()
        }
    }
    println!("final total {}", total);
}
