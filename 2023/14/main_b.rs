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
    type YX = (i32, i32);

    // map storage
    const DIM : usize = 110;
    type Map = [[char; DIM]; DIM];
    let mut map: Map = [['.'; DIM]; DIM];
    let mut width : usize = 0;
    let mut height : usize = 0;

    // Known rock locations
    const ROCKS : usize = 3000;
    let mut rock_yx : [YX; ROCKS] = [(0,0); ROCKS];
    let mut rocks = 0;

    // read the input
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
                    if c == 'O' {
                        rock_yx[rocks] = (height as i32, x as i32);
                        rocks += 1;
                    }
                }
            }
            height += 1;
        }
    }

    // lambda to print the map
    let print_map = | m: &Map, w : usize, h : usize | {
        for y in 0..h {
            for x in 0..w {
                print!("{}", m[y][x])
            }
            println!("");
        }
        println!("");
    };

    // print the starting map
    print_map(&map, width, height);

    // run the simulation
    const CYCLES : usize = 1000000000;
    const STEPS : [YX; 4] = [ (-1, 0), (0, -1), (1, 0), (0, 1)];
    for steps in 0 .. CYCLES * 4 {
        let delta : YX = STEPS[steps % 4];
        let mut moved = true;
        while moved {
            moved = false;
            for r in 0..rocks {
                let old : YX = rock_yx[r];
                let next : YX = (old.0 + delta.0, old.1 + delta.1);
                if next.0 >= 0 && next.1 >= 0 && next.0 < height as i32 && next.1 < width as i32 {
                    if map[next.0 as usize][next.1 as usize] == '.' {
                        map[next.0 as usize][next.1 as usize] = 'O';
                        map[old.0 as usize][old.1 as usize] = '.';
                        rock_yx[r] = next;
                        moved = true;
                    }
                }
            }
        }

        // give a progress report
        if steps % 100000 == 0 {
            println!("After step {}", steps);
        }
    }

    // show the final map
    print_map(&map, width, height);

    // lambda to print the map
    let score_map = | m: &Map, w : usize, h : usize | -> i64 {
        let mut total = 0;
        for y in 0..h {
            for x in 0..w {
                if map[y][x] == 'O' {
                    let weight = height - y;
                    total += weight;
                }
            }
        }
    }

    let total = score_map(&map, width, height);
    println!("Total {}", total);
}
