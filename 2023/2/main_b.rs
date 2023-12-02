use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::process;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {

    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines(file_path) {
        let mut sum : i64 = 0;
        for line in lines {
            if let Ok(contents) = line {
                let v: Vec<&str> = contents.split(':').collect();
                let games: Vec<&str> = v[1].split(';').collect();
                let mut red : i32 = 0;
                let mut green : i32 = 0;
                let mut blue : i32 = 0;
                for game in games {
                    let tokens: Vec<&str> = game.split(' ').collect();
                    let mut count : i32 = 0;
                    for token in tokens {
                        if token.len() == 0 { continue; }

                        let value = token.parse::<i32>();
                        if value.is_ok() {
                            count = value.unwrap();
                        }
                        else {
                            if token.starts_with("red") { if count > red { red = count; }}
                            else if token.starts_with("green") { if count > green { green = count; }}
                            else if token.starts_with("blue") { if count > blue { blue = count; }}
                            else { println!("bad \"{}\"", token); process::abort(); }
                        }
                    }
                }
                let power = red * green * blue;
                sum += power as i64;
            }
        }
        println!("power sum {}", sum);
    }
}
