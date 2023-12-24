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

fn main() {

    let mut width : usize = 0;
    let mut height : usize = 0;
    const DIM : usize = 110;
    type Map = [[char; DIM]; DIM];
    let mut map: Map = [['.'; DIM]; DIM];

    type XY =
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(contents) = line {
                if width == 0 {
                    width = contents.len();
                }
                for (x, c) in contents.chars().enumerate() {
                    map[height][x] = c;
                }
            }
            height += 1;
        }
    }

    for y in 0..height {
        for x in 0..width {
            print!("{}", map[y][x])
        }
        println!("");
    }

    let mut turns = 0;
    let mut moved = true;
    while moved {
        moved = false;

        for y in 1..height {
            for x in 0..width {
                if map[y][x] == 'O' && map[y-1][x] == '.' {
                    map[y][x] = '.';
                    map[y-1][x] = 'O';
                    if !moved {
                        moved = true;
                        turns += 1;
                    }
                }
            }
        }
    }

    println!("\nturns {}", turns);
    let mut total = 0;
    for y in 0..height {
        for x in 0..width {
            if map[y][x] == 'O' {
                let weight = height - y;
                total += weight;
            }
        }
    }

    println!("\ntotal {}", total);
    if false {
        for y in 0..height {
            for x in 0..width {
                print!("{}", map[y][x])
            }
            println!("");
        }
    }
}
