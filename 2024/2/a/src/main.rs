use std::fs::read_to_string;
use std::str::FromStr;

fn main() {

    let use_example = 0;
    let filename =
        if use_example != 0 {
            "/home/jgrills/adventofcode/2024/2/example.txt"
        } else {
            "/home/jgrills/adventofcode/2024/2/input.txt"
        };
    let file_string = read_to_string(filename).unwrap();

    let mut total_safe = 0;
    for line in file_string.lines() {
        let mut tokens = line.split_ascii_whitespace();
        let first = i64::from_str(tokens.next().unwrap()).unwrap();
        let second = i64::from_str(tokens.next().unwrap()).unwrap();
        let mut safe = (first != second) && (first - second).abs() <= 3;
        print!("{} {}", first, second);
        if safe {
            let direction = if first < second { 1 } else { -1 };
            let mut current = second;
            for token in tokens {
                let tok = i64::from_str(token).unwrap();
                print!(" {}", tok);
                let delta = if direction == 1 { tok - current } else { current - tok };
                if delta <= 0 || delta > 3 {
                    print!(" UNSAFE");
                    safe = false;
                    break;
                }
                current = tok;
            }
        }
        println!("\n{} {}", safe, line);
        if safe { total_safe +=1 }
    }

    println!("total safe {}", total_safe);
}
