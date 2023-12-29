use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct YX {
    y : i16,
    x : i16
}

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
struct WYX {
    y : u8,
    x : u8
}

const WALK : usize = ((142 * 142) * 2) / 3;

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
struct Walk {
    steps: usize,
    step: [WYX; WALK]
}

const DIM : usize = 150;
type Map = [[char; DIM]; DIM];

fn a_star(map: &Map, width: i16, height: i16, start: YX, goal: YX) -> Walk {

    let mut heap = BinaryHeap::new();

    let mut result : Walk = Walk{step: [WYX{y:0,x:0}; WALK], steps:0 };

    // start of the search
    let sx : WYX = WYX{y:start.y as u8, x:start.x as u8};
    heap.push(Reverse(Walk{step: [sx; WALK], steps:1 }));

    // Expand the search frontier
    while let Some(Reverse(mut walk)) = heap.pop() {

        let wp = walk.step[walk.steps-1];
        let position : YX = YX{ y:wp.y as i16, x: wp.x as i16};

        // let under = map[position.y as usize][position.x as usize];
        // println!("exploring {}=len {}=steps at {},{}", heap.len(), walk.steps, position.y, position.x);

        if position == goal {
            result = walk;
            println!("found  {}=heap.len {}=steps", heap.len(), walk.steps);
            continue;
        }

        // Examine each step out of this cell
        'steploop: for (_dir, step) in STEPS.iter().enumerate() {

            if true {
                let p : YX = YX{ y: position.y + step.0, x: position.x + step.1};

                // make sure the destination is a valid cell and the cell is traversable
                if p.y >= 0 && p.x >= 0 && p.x < width && p.y < height && map[p.y as usize][p.x as usize] != '#' {

                    // make sure we don't retrace a previous step
                    for k in (0..walk.steps).rev() {
                        let wsk = walk.step[k];
                        if p.x == wsk.x as i16 && p.y == wsk.y as i16 { continue 'steploop; }
                    }

                    walk.step[walk.steps] = WYX{y:p.y as u8, x:p.x as u8};
                    heap.push(Reverse(Walk{ step: walk.step, steps: walk.steps+1}));
                }
            }
        }
    }

    return result;
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let mut width : i16 = 0;
    let mut height : i16 = 0;
    let mut map: Map = [[' '; DIM]; DIM];

    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(contents) = line {
                if width == 0 {
                    width = contents.len() as i16;
                }
                for (x, c) in contents.chars().enumerate() {
                    map[height as usize][x] = c;
                }
            }
            height += 1;
        }
    }

    let print_map = | m: &Map, w : i16, h : i16 | {
        for y in 0..h as usize{
            for x in 0..w as usize {
                print!("{}", m[y][x])
            }
            println!("");
        }
        println!("");
    };

    print_map(&map, width, height);

    let walk = a_star(&map, width, height, YX{y:0,x:1}, YX{y:height-1,x:width-2});
    if walk.steps > 0 {
        if false {
            for s in 0..walk.steps {
                let p = walk.step[s];
                println!("{} {},{}", s, p.x, p.y);
            }
        }
        println!("steps {}", walk.steps - 1);
    } else {
        println!("no solution");
    }
}
