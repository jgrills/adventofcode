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
        filename = "/home/jgrills/adventofcode/2024/7/example.txt"
    } else {
        filename = "/home/jgrills/adventofcode/2024/7/input.txt"
    };
    let file_string = read_to_string(filename).unwrap();

    let mut total_valid = 0;
    let mut numbers = Vec::new();
    for whole_line in file_string.lines() {
        let line : &str = whole_line;
        let (result_str, rest) = split_with_char(line, ':');
        let result = i64::from_str(result_str).unwrap();
        // println!("result {} >> '{}':'{}'", whole_line, result, rest);
        numbers.clear();
        for number in rest.split_ascii_whitespace() {
            let n  = i64::from_str(number).unwrap();
            numbers.extend([n]);
            // println!(" {}", n);
        }
        let operators = numbers.len() - 1;

        let valid : bool = {
            let mut valid = false;
            let mut permutation = [0 as u8; 32];
            'outer: loop {
                let mut value = numbers[0];
                for i in 0..operators {
                    match permutation[i] {
                        0 => {
                            value = value * numbers[i+1];
                        },
                        1 => {
                            value = value + numbers[i+1];
                        },
                        2 => {
                            let low = numbers[i+1];
                            let mut n = low;
                            while n != 0 {
                                value *= 10;
                                n /= 10;
                            }
                            value += low;
                        },
                        _ => {
                            assert!(false, "bad operator");
                        }
                    }
                }
                if value == result {
                    valid = true;
                    break 'outer;
                } else {
                    let mut slot = 0;
                    loop {
                        if slot == operators {
                            break 'outer;
                        }
                        if permutation[slot] == 2 {
                            permutation[slot] = 0;
                            slot += 1;
                        } else {
                            permutation[slot] += 1;
                            break;
                        }
                    }
                }
            }
            valid
        };
        if valid { total_valid += result; }
        println!("{} {} {}", valid, total_valid, whole_line);
    }
}
