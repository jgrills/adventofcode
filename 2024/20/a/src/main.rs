use std::fs::read_to_string;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
struct YX { y:i32, x:i32 }
impl YX {
    fn add(&self, rhs : Self) -> Self { Self{ y:self.y + rhs.y, x: self.x + rhs.x } }
    fn us(&self) -> (usize, usize) { (self.y as usize, self.x as usize) }
}

const UP : YX = YX{y:-1,x:0};
const DOWN : YX = YX{y:1,x:0};
const RIGHT : YX = YX{y:0,x:1};
const LEFT : YX = YX{y:0,x:-1};
const DIRECTIONS : usize = 4;
const DIRECTION : [YX; DIRECTIONS] = [ UP, RIGHT, DOWN, LEFT ];

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: i32,
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

struct Map {
    height : i32,
    width : i32,
    bytes : String
}

impl Map {
    fn is_valid(&self, yx : YX) -> bool {
        yx.y >= 0 && yx.x >= 0 && yx.y < self.height && yx.x < self.width
    }

    fn cell(&self, yx : YX) -> u8 {
        if self.is_valid(yx) {
            let index = ((yx.y * (self.width as i32 + 1)) + yx.x) as usize;
            self.bytes.bytes().nth(index).unwrap()
        } else {
            b'#'
        }
    }

    fn can_traverse(&self, yx : YX) -> bool {
        self.is_valid(yx) && self.cell(yx) != b'#'
    }

}

struct Range {
    range : [[i32;160]; 160]
}

impl Range {
    fn is_valid(&self, yx : YX) -> bool {
        yx.y >= 0 && yx.x >= 0 && yx.y < 160 && yx.x < 160
    }

    fn assert_valid(&self, yx : YX) {
        assert!(self.is_valid(yx), "out of map");
    }

    fn cell(&self, yx : YX) -> i32 {
        self.assert_valid(yx);
        let (y,x) = yx.us();
        self.range[y][x]
    }

    fn set_cell(&mut self, yx : YX, value : i32) {
        self.assert_valid(yx);
        let (y,x) = yx.us();
        self.range[y][x] = value;
    }
}

fn main() {
    let use_example = 0;
    let map : Map;
    if use_example == 1 {
        map = Map{height:15, width:15, bytes:read_to_string("/home/jgrills/adventofcode/2024/20/example.txt").unwrap()};
    } else {
        map = Map{height:141, width:141, bytes:read_to_string("/home/jgrills/adventofcode/2024/20/input.txt").unwrap()};
    };

    let mut start = YX{y:-1,x:-1};
    let mut end = YX{y:-1,x:-1};

    for y in 0..map.height {
        for x in 0..map.width {
            let pos = YX{y,x};
            match map.cell(pos) {
                b'S' => start = pos,
                b'E' => end = pos,
                _ => (),
            };
        }
    }

    let mut pq = BinaryHeap::new();
    let mut range = Range{range:[[-1;160];160]};
    pq.push(State{ cost:0, pos:end });
    while let Some(State{cost,pos}) = pq.pop() {
        if range.cell(pos) < 0 { 
            range.set_cell(pos, cost);
            for i in 0..DIRECTIONS {
                let cost = cost + 1;
                let pos = pos.add(DIRECTION[i]);
                if map.can_traverse(pos) { pq.push(State{cost,pos}); }
            }
        }
    }

    let mut hun = 0;
    for y in 0..map.height {
        for x in 0..map.width {
            let pos = YX{y,x};
            if map.cell(pos) == b'#' {
                let up = pos.add(UP);
                let down = pos.add(DOWN);
                let right = pos.add(RIGHT);
                let left = pos.add(LEFT);
                if map.cell(up) != b'#' && map.cell(down) != b'#' {
                    let from = std::cmp::max(range.cell(up), range.cell(down));
                    let to = std::cmp::min(range.cell(up), range.cell(down));
                    let cost = range.cell(start) - from + 1 + 1 + to;
                    let savings = range.cell(start) - cost;
                    if savings >= 100 { hun += 1; }
                    println!("{} {} skip {} {} vertically {}", savings, hun, pos.y, pos.x, cost);
                }
                if map.cell(right) != b'#' && map.cell(left) != b'#' {
                    let from = std::cmp::max(range.cell(right), range.cell(left));
                    let to = std::cmp::min(range.cell(right), range.cell(left));
                    let cost = range.cell(start) - from + 1 + 1 + to;
                    let savings = range.cell(start) - cost;
                    if savings >= 100 { hun += 1; }
                    println!("{} {} skip {} {} horizontal {}", savings, hun, pos.y, pos.x, cost);
                }
            };
        }
    }
    println!("{}", hun);
}
