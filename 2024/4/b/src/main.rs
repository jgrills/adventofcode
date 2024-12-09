use std::fs::read_to_string;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
struct YX { y:i32, x:i32 }
impl YX {
    fn add(&self, rhs : Self) -> Self { Self{ y:self.y + rhs.y, x: self.x + rhs.x } }
}

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

    let check_sm = |yx:YX, d0:YX, d1:YX| -> bool {
        let yx_d0 = yx.add(d0);
        let yx_d1 = yx.add(d1);
        let yx_d0_ch = map(yx_d0);
        let yx_d1_ch = map(yx_d1);
        (yx_d0_ch == 'M' && yx_d1_ch == 'S') || (yx_d0_ch == 'S' && yx_d1_ch == 'M')
    };

    let xmas = |yx: YX| -> bool {
        if map(yx) !='A' { return false; }
        if !check_sm(yx, YX{y:-1,x:-1}, YX{y: 1,x: 1}) { return false; }
        if !check_sm(yx, YX{y:-1,x: 1}, YX{y: 1,x:-1}) { return false; }
        true
    };

    let mut total : i32 = 0;
    for y in 0..height {
        for x in 0..width {
            if xmas(YX{y,x}) {
                total += 1;
                // println!("xmas {} {} {}", y, x, total);
            }
        }
        println!("line {} {}", y, total)
    }
    println!("total {}", total);
}
