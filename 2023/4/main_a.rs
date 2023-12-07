use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// Card   1:  5 27 94 20 50  7 98 41 67 34 | 34  9 20 90  7 77 44 71 27 12 98  1 79 96 24 51 25 84 67 41  5 13 78 31 26

fn main() {

    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let mut sum : i64 = 0;

    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines(file_path) {

        for line in lines {
            if let Ok(contents) = line {
                const NUMBERS : usize = 100;
                let mut number_flags : [u8; NUMBERS] = [0; NUMBERS];
                let mut bit : u8 = 1;
                let mut line_value : i32 = 0;

                println!("Line {}", contents);
                println!("  starting with winnners");
                let mut tokens = contents.split_whitespace();
                tokens.next();
                tokens.next();
                for token in tokens {
                    println!("  token {}", token);
                    if token == "|" {
                        println!("  | switching to have");
                        bit = 2;
                    } else {
                        let index = token.parse::<usize>().unwrap();
                        println!("  index {}", index);
                        number_flags[index] |= bit;
                        println!("  index {} {}", index, number_flags[index]);
                        if number_flags[index] == 3 {
                            line_value = if line_value == 0 { 1 } else { line_value << 1 };
                        }
                    }
                }
                println!("  score {}", line_value);
                sum += line_value as i64;
            }
        }
    }
    println!("final sum {}", sum);
}
