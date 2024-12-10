use std::fs::read_to_string;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
struct YX { y:i32, x:i32 }
impl YX {
    fn add(&self, rhs : Self) -> Self { Self{ y:self.y + rhs.y, x: self.x + rhs.x } }
}

const UP : YX = YX{y:-1,x:0};
const DOWN : YX = YX{y:1,x:0};
const LEFT : YX = YX{y:0,x:-1};
const RIGHT : YX = YX{y:0,x:1};

struct Reached {
    reached : [[bool; 60]; 60]
}

struct Map {
    width : usize,
    height : usize,
    bytes : String
}
impl Map {
    fn is_valid(&self, yx : YX) -> bool {
        yx.y >= 0 && yx.x >= 0 && (yx.y as usize) < self.height && (yx.x as usize) < self.width
    }

    fn map(&self, yx : YX) -> u8 {
        if self.is_valid(yx) {
            let index = ((yx.y * (self.width as i32 + 1)) + yx.x) as usize;
            self.bytes.bytes().nth(index).unwrap()
        } else {
            b' '
        }
    }

    fn count_sub_trees(&self, pos : YX, d : u8, r : &mut Reached) -> usize {
        if self.map(pos) != d {
            0
        } else if d == b'9' {
            1
        } else {
            let d1 = d + 1;
            self.count_sub_trees(pos.add(UP), d1, r) +
            self.count_sub_trees(pos.add(DOWN), d1, r) +
            self.count_sub_trees(pos.add(LEFT), d1, r) +
            self.count_sub_trees(pos.add(RIGHT), d1, r)
        }
    }

    fn count_trees(&self, pos : YX) -> usize {
        let mut r = Reached{reached:[[false; 60]; 60]};
        self.count_sub_trees(pos, b'0', &mut r)
    }
}

fn main() {
    let use_example = 0;
    let width : usize;
    let height : usize;
    let filename;
    if use_example != 0 {
        filename = "/home/jgrills/adventofcode/2024/10/example.txt";
        width = 8;
        height = 8;
    } else {
        filename = "/home/jgrills/adventofcode/2024/10/input.txt";
        width = 53;
        height = 53;
    };
    let m = Map{ width:width, height:height, bytes:read_to_string(filename).unwrap() };

    let mut heads = Vec::new();
    for y in 0..height as i32 {
        for x in 0..width as i32 {
            let yx = YX{y:y,x:x};
            if m.map(yx) == b'0' {
                println!("0 trailhead at {} {}", y, x);
                heads.extend([yx]);
            }
        }
    }

    println!("found {} trailheads", heads.len());

    let mut total = 0;
    for head in heads {
        print!("trailhead at {} {}", head.y, head.x);
        let trees = m.count_trees(head);
        total += trees;
        println!(" trees {} total {}", trees, total);
    }
}
