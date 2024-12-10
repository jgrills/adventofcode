use std::fs::read_to_string;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
struct YX { y:i32, x:i32 }
impl YX {
    fn add(&self, rhs : Self) -> Self { Self{ y:self.y + rhs.y, x: self.x + rhs.x } }
}

fn main() {
    let use_example = 1;
    let width : i32;
    let height : i32;
    let filename;
    if use_example != 0 {
        filename = "/home/jgrills/adventofcode/2024/6/example.txt";
        width = 10;
        height = 10;
    } else {
        filename = "/home/jgrills/adventofcode/2024/6/input.txt";
        width = 130;
        height = 130;
    };
    let file_string = read_to_string(filename).unwrap();
    let is_valid = | yx : YX | -> bool {
        yx.y >= 0 && yx.x >= 0 && yx.y < height && yx.x < width
    };

    let map = |yx : YX| -> char {
        if is_valid(yx) {
            let index = ((yx.y * (width + 1)) + yx.x) as usize;
            file_string.chars().nth(index).unwrap()
        } else {
            'E'
        }
    };

    let mut found = false;
    let mut pos = YX{ y:-1,x:-1 };
    for y in 0..height {
        for x in 0..width {
            let yx = YX{y:y,x:x};
            if map(yx) == '^' {
                pos = yx;
                found = true;
                break;
            }
        }
    }
    assert!(found, "start not found");
    println!("start {} {}", pos.y, pos.x);

    let mut visited = [[false; 130]; 130];
    visited[pos.y as usize][pos.x as usize] = true;
    let mut visited_count = 1;

    const DIRECTIONS : usize = 4;
    const DIRECTION : [YX; DIRECTIONS] = [YX{y:-1,x:0}, YX{y:0,x:1}, YX{y:1,x:0}, YX{y:0,x:-1}];
    let mut direction_index : usize = 0;
    while map(pos) != 'E' {
        let advance_pos = pos.add(DIRECTION[direction_index]);
        let advance = map(advance_pos);
        if advance == '#' {
            direction_index = (direction_index + 1) % DIRECTIONS;
            continue;
        }
        pos = advance_pos;
        if advance != 'E' && !visited[pos.y as usize][pos.x as usize] {
            visited[pos.y as usize][pos.x as usize] = true;
            visited_count += 1;
        }
        println!("walk {} @ {},{} = '{}' :{}", direction_index, pos.y, pos.x, map(pos), visited_count);
    }
}
