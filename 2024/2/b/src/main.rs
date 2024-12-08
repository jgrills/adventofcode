use std::fs::read_to_string;
use std::str::FromStr;

fn is_safe(tokens: &[i64]) -> bool {
    let first = tokens[0];
    let second = tokens[1];
    let mut safe = (first != second) && (first - second).abs() <= 3;
    print!("{} {}", first, second);
    if safe {
        let direction = if first < second { 1 } else { -1 };
        let mut current = second;
        for token in &tokens[2..] {
            let tok = *token;
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
    println!("\n{}\n", safe);
    safe
}

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
        let tokens : Vec<_> = line.split_ascii_whitespace().map(|tok| i64::from_str(tok).unwrap()).collect();
        let mut safe = is_safe(&tokens);
        if !safe {
            for n in 0..tokens.len() {
                let mut skipone = tokens.clone();
                skipone.splice(n..n+1, []);
                println!("tokens: {:?}", tokens);
                println!("skipone {}: {:?}", n, skipone);
                if is_safe(&skipone) {
                    safe = true;
                    break;
                }
            }
        }
        if safe {
            total_safe +=1;
        }
    }

    println!("total safe {}", total_safe);
}
