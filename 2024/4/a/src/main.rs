use std::fs::read_to_string;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
struct YX { y:i32, x:i32 }
impl YX {
    fn add(&self, rhs : Self) -> Self { Self{ y:self.y + rhs.y, x: self.x + rhs.x } }
}

const DIRECTIONS : usize = 8;
const DIRECTION : [YX; DIRECTIONS] = [
    YX{y:-1,x:0}, YX{y:1,x:0}, YX{y:0,x:-1}, YX{y:0,x:1},
    YX{y:1,x:1},  YX{y:1,x:-1}, YX{y:-1,x:1}, YX{y:-1,x:-1} ];

fn main() {
    let use_example = 0;
    let width : i32;
    let height : i32;
    let filename;
    if use_example != 0 {
        filename = "/home/jgrills/adventofcode/2024/4/example.txt";
        width = 10;
        height = 10;
    } else {
        filename = "/home/jgrills/adventofcode/2024/4/input.txt";
        width = 140;
        height = 140;
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
            ' '
        }
    };

    let xmas = | yx: YX, step: YX| -> bool {
        if map(yx) !='X' { return false; }
        let yx1 = yx.add(step);
        if map(yx1) != 'M' { return false; }
        let yx2 = yx1.add(step);
        if map(yx2) != 'A' { return false; }
        let yx3 = yx2.add(step);
        if map(yx3) != 'S' { return false; }
        true
    };

    let mut total : i32 = 0;
    for y in 0..height {
        for x in 0..width {
            for d in 0..DIRECTIONS {
                if xmas(YX{y,x}, DIRECTION[d]) {
                    // println!("xmas {} {} {} {}", y, x, DIRECTION[d].y, DIRECTION[d].x);
                    total += 1;
                }
            }
        }
        println!("line {}", y)
    }
    println!("total {}", total);
}
