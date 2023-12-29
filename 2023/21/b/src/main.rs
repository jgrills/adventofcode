use std::env;
use std::fs;
use std::collections::HashSet;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
struct YX { y:i64, x:i64}

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
    let mut start =  YX{y:0,x:0};
    let mut rest : &str = file_contents.as_str();
    while !rest.is_empty() {
        let line : &str;
        (line, rest) = split_with_char(rest, '\n');
        for (x, c) in line.chars().enumerate() {
            map[height][x] = match c {
                '.' => 2,
                'S' => {
                    start = YX{ y:height as i64, x:x as i64};
                    2
                },
                '#' => 1,
                _ => panic!("unknown map element")
            }
        }
        if width == 0 { width = line.len(); }
        height += 1;
    }

    let _print_map = | dests : &HashSet<YX> | {
        for y in 0..height {
            let y64 = y as i64;
            for x in 0..width  {
                let x64 = x as i64;
                let v = map[y][x];
                let out = match v {
                    1 => if dests.contains(&YX{y:y64,x:x64}) { panic!(); } else { '#' },
                    2 => if dests.contains(&YX{y:y64,x:x64}) { 'O' } else { '.' },
                    _ => panic!("unexpected value {}", v)
                };
                print!("{}", out);
            }
            println!();
        }
    };

    let mut active = HashSet::new();
    let mut new = HashSet::new();
    let mut frontier = HashSet::new();
    let w64 = width as i64;
    let h64 = height as i64;
    frontier.insert(start.clone());
    let num_steps : i32 = 26501365;
    for steps in 0..num_steps {
        let mut new_frontier = HashSet::new();
        for yx in &frontier {
            let y = yx.y;
            let x = yx.x;

            let mut step = | y : i64, x : i64 | {
                let my = ((if y < 0 { y + ((-y / h64) + 1) * h64 } else { y }) % h64) as usize;
                let mx = ((if x < 0 { x + ((-x / w64) + 1) * w64 } else { x }) % w64) as usize;
                if map[my][mx] == 2 {
                    if new.insert(YX{y,x}) {
                        new_frontier.insert(YX{y,x});
                    }; 
                }
            };

            step(y-1, x  );
            step(y  , x-1);
            step(y+1, x  );
            step(y  , x+1);
        }

        let finished = (((steps+1)*100) as f64) / num_steps as f64; 
        if (steps+1) % 100 == 0 { println!("{:.3} turn {} spots {} added {}", finished, steps+1, new.len(), new_frontier.len()); }
        //print_map(&active);
        std::mem::swap(&mut active, &mut new);
        std::mem::swap(&mut frontier, &mut new_frontier);
    }
    println!("spots {}", active.len());
}
