use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

type YXC = (i32, i32, char);
const STEPS : [YXC; 4] = [(-1, 0, '^'), (1, 0, 'v'), (0, -1, '<'), (0, 1, '>')];

type Spot=(u8,u8,u8);
const DIM_SPOTS : usize = 100;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Path {
    estimate: i32,
    cost: i32,
    spot: [Spot;DIM_SPOTS].
    spots: usize
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.estimate.cmp(&self.estimate)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

const DIM : usize = 150;
type Map = [[i32; DIM]; DIM];

fn a_star(map: &Map, width: i32, height: i32, start: YX, goal: YX) -> Option<i32> {

    let mut heap = BinaryHeap::new();

    // start of the search
    heap.push(Path { estimate: 0, cost: 0, spot[(0,0,0);DIM_SPOTS], spots: 0 });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(Path mut path) = heap.pop() {

        println!("exploring {}=len {}=est {}=cost {}=spots", heap.len(), path.estimate, path.cost, path.spots);

        // Alternatively we could have continued to find all shortest paths
        let cur = path.spot[path.spots]

        if position == goal {
            for i in 0..steps {
                println!("step {} {},{}", i, STEPS[prev[i] as usize].0, STEPS[prev[i] as usize].1);
            }
            return Some(cost);
        }

        // Examine each step out of this cell
        for (dir, step) in STEPS.iter().enumerate() {
            let p : YX = ( position.1 + step.1, position.0 + step.0 );
            if p.0 >= 0 && p.0 < width && p.1 >= 0 && p.1 < height {

                if p == last {
                    // println!("  skipping last");
                }
                else {
                    let c = cost + map[p.1 as usize][p.0 as usize];
                    let e = ((goal.0 - p.0) + (goal.1 - p.1)) + c;
                    let r = [dir, recent[0], recent[1]];

                    // println!("  pushing {}=est {}=cost {},{}=pos {}{}{}=recent", e, c, p.0, p.1, r[0], r[1], r[2]);
                    prev[steps] = dir as u8;
                    heap.push(Path { estimate: e, cost: c, last: position, position: p, recent: r, steps: steps+1, prev: prev});
                }
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
    let mut map: Map = [[0; DIM]; DIM];

    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(contents) = line {
                if width == 0 {
                    width = contents.len();
                }
                for (x, c) in contents.chars().enumerate() {
                    map[height][x] = (c as i32) - ('0' as i32);
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

    if let Some(result) = a_star(&map, width as i32, height as i32, (0,0), (height as i32 - 1,width as i32 - 1)) {
        println!("result {}", result);
    }
    else {
        println!("no result");
    }
}
