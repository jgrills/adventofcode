use std::env;
use std::fs;

fn split_with_char(input: &str, with: char) -> (&str, &str) {
    match input.find(with) {
        Some(index) => (&input[0..index], &input[index+1..]),
        None => (input, "")
    }
}

fn main() {
    const DIM : usize = 140;
    let mut width : usize = 0;
    let mut height : usize = 0;
    type Map = [[u8; DIM]; DIM];
    let mut map : Map = [[0; DIM]; DIM];

    // grab the first command line argument, use it as a filename and load that file into a string
    let file_path : String = env::args().nth(1).unwrap();
    let file_contents : String = match fs::read_to_string(file_path.clone()) {
        Ok(fc) => fc,
        Err(..) => panic!("couldn't read {}", file_path)
    };

    // get str for the rest of the file contents left to process
    let mut rest : &str = file_contents.as_str();
    while !rest.is_empty() {
        let line : &str;
        (line, rest) = split_with_char(rest, '\n');
        for (x, c) in line.chars().enumerate() {
            map[height as usize][x] = match c {
                '.' => 0b0010,
                'S' => 0b0110,
                '#' => 0b0001,
                _ => panic!("unknown map element")
            }
        }
        if width == 0 { width = line.len(); }
        height += 1;
    }

    let print_map = | m: &Map, w : usize, h : usize | {
        for y in 0..h {
            for x in 0..w  {
                let v = m[y][x];
                let out = match v {
                    1 => '#',
                    2 => '.',
                    6 => 'O',
                    _ => panic!("unexpected value {}", v)
                };
                print!("{}", out);
            }
            println!("");
        }
        println!("");
    };


    print_map(&map, width, height);

    let from = 0b0110;
    let to =   0b1000;
    for steps in 0..64 {
        for y in 0..height {
            for x in 0..width {
                let cell = map[y][x];
                let current = cell == from;
                if current {
                    if y > 0 { map[y-1][x] |= to; }
                    if x > 0 { map[y][x-1] |= to; }
                    if y < height-1 { map[y+1][x] |= to; }
                    if x < width-1  { map[y][x+1] |= to; }
                }
            }
        }


        let mut spots = 0;
        for y in 0..height {
            for x in 0..width {
                let cell = map[y][x];
                let result = match cell {
                    0b0001 | 0b1001 => 0b0001,
                    0b1010 | 0b1110 => {
                        spots += 1;
                        0b0110
                    },
                    0b0010 | 0b0110 => 0b0010,
                    _ => panic!("unexpected cell {}", cell)
                };
                map[y][x] = result;
            }
        }

        println!("\nturn {} spots {}", steps, spots);
        // print_map(&map, width, height);
    }
}
