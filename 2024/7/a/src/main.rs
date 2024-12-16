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
        let stop_value = 1 << (operators + 1);
        // println!(" operators {}", operators);

        let valid : bool = {
            let mut valid = false;
            let mut permutation = 0;
            while !valid && permutation < stop_value {
                let mut value = numbers[0];
                for i in 0..operators {
                    let bit = 1 << i;
                    if permutation & bit == bit {
                        let next_value = value * numbers[i+1];
                        value = next_value;
                    } else {
                        let next_value = value + numbers[i+1];
                        value = next_value;
                    }
                }
                if value == result {
                    valid = true;
                } else {
                    permutation += 1;
                }
            }
            valid
        };
        if valid { total_valid += result; }
        println!("{} {} {}", valid, total_valid, whole_line);
    }
}
