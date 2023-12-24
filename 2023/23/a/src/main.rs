use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

type YXC = (i8, i8, char);
const STEPS : [YXC; 4] = [(-1, 0, '^'), (1, 0, 'v'), (0, -1, '<'), (0, 1, '>')];

#[derive(Copy, Clone, Eq, PartialEq)]
struct YX {
    y : usize,
    x : usize
}

const WALK : usize = 1000;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Walk {
    steps: usize,
    step: [YX; WALK]
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for Walk {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.steps.cmp(&self.steps)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for Walk {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

const DIM : usize = 150;
type Map = [[char; DIM]; DIM];

fn a_star(map: &Map, width: usize, height: usize, start: YX, goal: YX) -> Option<Walk> {

    let mut heap = BinaryHeap::new();

    // start of the search
    heap.push(Walk{step: [start; WALK], steps:1 });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(Walk mut walk) = heap.pop() {

        let position = walk.step[walk.steps-1];
        println!("exploring {}=len {}=steps at {},{}", heap.len(), walk.steps, position.y, position.x);

        if position == goal {
            return Some(walk);
        }

        // Examine each step out of this cell
        'steploop: for (dir, step) in STEPS.iter().enumerate() {
            let p : YX = YX{ y: position.y + step.0, x: position.x + step.1 };
            if p.y >= 0 && p.x >= 0 && p.x < width && p.y < height {

                let mut found : bool = false;
                for k in 0..walk.steps {
                    if p == walk.step[k] { break 'steploop; }
                }

                walk[walk.steps] = p;
                heap.push(Walk{ step: walk.step, steps: walk.steps+1});
            }
        }
    }

    // Goal not reachable
    None
}

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
    let mut map: Map = [[' '; DIM]; DIM];

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

    let print_map = | m: &Map, w : usize, h : usize | {
        for y in 0..h {
            for x in 0..w {
                print!("{}", m[y][x])
            }
            println!("");
        }
        println!("");
    };

    print_map(&map, width, height);
}
