use std::fs::read_to_string;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
struct YX { y:i32, x:i32 }
impl YX {
    fn add(&self, rhs : Self) -> Self { Self{ y:self.y + rhs.y, x: self.x + rhs.x } }
}

const DIRECTIONS : usize = 4;
const DIRECTION : [YX; DIRECTIONS] = [ YX{y:-1,x:0}, YX{y:0,x:1}, YX{y:1,x:0}, YX{y:0,x:-1} ];
const LEFT : [usize; DIRECTIONS] = [ 3, 0, 1, 2 ];
const RIGHT : [usize; DIRECTIONS] = [ 1, 2, 3, 0 ];
const REVERSE : [usize; DIRECTIONS] = [ 2, 3, 0, 1 ];

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: YX,
    direction_index: usize
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
            .then_with(|| self.position.cmp(&other.position))
            .then_with(|| self.direction_index.cmp(&other.direction_index))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


struct Map {
    height : usize,
    width : usize,
    bytes : String
}

impl Map {
    fn is_valid(&self, yx : YX) -> bool {
        yx.y >= 0 && yx.x >= 0 && (yx.y as usize) < self.height && (yx.x as usize) < self.width
    }

    fn assert_valid(&self, yx : YX) {
        assert!(self.is_valid(yx), "out of map");
    }

    fn cell(&self, yx : YX) -> u8 {
        self.assert_valid(yx);
        let index = ((yx.y * (self.width as i32 + 1)) + yx.x) as usize;
        self.bytes.bytes().nth(index).unwrap()
    }
}

fn main() {
    let use_example = 0;
    let map : Map;
    if use_example == 1 {
        map = Map{height:15, width:15, bytes:read_to_string("/home/jgrills/adventofcode/2024/16/example_a.txt").unwrap()};
    } else if use_example == 2 {
        map = Map{height:17, width:17, bytes:read_to_string("/home/jgrills/adventofcode/2024/16/example_b.txt").unwrap()};
    } else {
        assert!(use_example == 0, "unexpected example value {}", use_example);
        map = Map{height:141, width:141, bytes:read_to_string("/home/jgrills/adventofcode/2024/16/input.txt").unwrap()};
    };
    let start = YX{y:map.height as i32 - 2, x:1};
    let end = YX{y:1, x:map.width as i32 - 2};
    assert!(map.cell(start) == b'S');
    assert!(map.cell(end) == b'E');

    let mut visited : [[[Option<usize>; 4]; 256]; 256] = [[[None; 4]; 256]; 256];

    let mut pq = BinaryHeap::new();
    pq.push(State{ cost:0, position:start, direction_index:1});
    while let Some(state) = pq.pop() {
        if state.position == end {
            println!("expanding path out of end {}", state.cost);
            break;
        }

        let y = state.position.y as usize;
        let x = state.position.x as usize;
        println!("cost {} at {} {} dir {}", state.cost, state.position.y, state.position.x, state.direction_index);

        let update = match visited[y][x][state.direction_index] {
            Some(x) => state.cost < x,
            None => true
        };

        if update {
            visited[y][x][state.direction_index] = Some(state.cost);
            let mut consider = | cost : usize, from : YX, direction_index : usize | {
                let position = from.add(DIRECTION[direction_index]);
                if map.cell(position) != b'#' { pq.push(State{cost, position, direction_index}); }
            };
            consider(state.cost + 1,   state.position,  state.direction_index);
            consider(state.cost + 1001, state.position, LEFT[state.direction_index]);
            consider(state.cost + 1001, state.position, RIGHT[state.direction_index]);
            consider(state.cost + 2001, state.position, REVERSE[state.direction_index]);
        }
    }
}
