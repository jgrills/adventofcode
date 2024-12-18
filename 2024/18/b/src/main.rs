use std::fs::read_to_string;
use std::str::FromStr;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
struct YX { y:i32, x:i32 }
impl YX {
    fn add(&self, rhs : Self) -> Self { Self{ y:self.y + rhs.y, x: self.x + rhs.x } }
}

const DIRECTIONS : usize = 4;
const UP : YX = YX{y:-1,x:0};
const DOWN : YX = YX{y:1,x:0};
const LEFT : YX = YX{y:0,x:-1};
const RIGHT : YX = YX{y:0,x:1};

const DIRECTION : [YX; 4] = [ UP, DOWN, LEFT, RIGHT ];

struct Map {
    width : i32,
    height : i32,
    bytes : [[u8; 96]; 96]
}

impl Map {
    fn is_valid(&self, yx : YX) -> bool {
        yx.y >= 0 && yx.x >= 0 && yx.y < self.height && yx.x < self.width
    }

    fn cell(&self, yx : YX) -> u8 {
        assert!(self.is_valid(yx));
        self.bytes[yx.y as usize][yx.x as usize]
    }
    fn set_cell(&mut self, yx : YX, ch : u8) {
        assert!(self.is_valid(yx), "bad yx {} {}", yx.y, yx.x);
        self.bytes[yx.y as usize][yx.x as usize] = ch;
    }
}

fn split_with_char(input: &str, with: char) -> (&str, &str) {
    match input.find(with) {
        Some(index) => (&input[0..index], &input[index+1..]),
        None => (input, "")
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    pos: YX
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let use_example = 0;
    let width : i32;
    let height : i32;
    let filename;
    if use_example != 0 {
        filename = "/home/jgrills/adventofcode/2024/18/example.txt";
        width = 7;
        height = 7;
    } else {
        filename = "/home/jgrills/adventofcode/2024/18/input.txt";
        width = 71;
        height = 71;
    };
    let file_string = read_to_string(filename).unwrap();
    let mut blockers = Vec::new();
    for line in file_string.lines() {
        let (x,y) = split_with_char(line, ',');
        let yx = YX{y:i32::from_str(y).unwrap(), x:i32::from_str(x).unwrap()};
        blockers.push(yx);
    }

    let mut min = 20;
    let mut max = blockers.len() - 1;
    let goal = YX{y:height-1, x:width-1};
    let mut pq = BinaryHeap::new();
    while min < max {
        let mid = (min + max) / 2;
        println!("testing {} [{}] {} : {} {}", min, mid, max, blockers[mid-1].y, blockers[mid-1].x);
        let mut map = Map{width, height, bytes:[[b'.';96];96]};
        for i in 0..mid { map.set_cell(blockers[i], b'#'); }

        let mut found = false;
        let mut expanded = [[false; 96]; 96];
        pq.clear();
        pq.push(State{cost:0, pos:YX{y:0, x:0}});
        while let Some(State{cost, pos}) = pq.pop() {
            if pos == goal {
                found = true;
                break;
            }
            let y = pos.y as usize;
            let x = pos.x as usize;
            if !expanded[y][x] {
                expanded[y][x] = true;
                // println!("Expanding {} {}", y, x);
                for i in 0..DIRECTIONS {
                    let next = pos.add(DIRECTION[i]);
                    if map.is_valid(next) && map.cell(next) == b'.' { pq.push(State{cost:cost+1, pos:next}); }
                }
            }
        }

        if found {
            println!(" found");
            min = mid + 1;
        } else {
            println!(" not");
            max = mid;
        }
    }

    println!("blocker at {} {},{}", min, blockers[min-1].x, blockers[min-1].y);
}
