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

struct Map {
    width : usize,
    height : usize,
    bytes : [[u8; 64]; 64]
}

impl Map {
    fn is_valid(&self, yx : YX) -> bool {
        yx.y >= 0 && yx.x >= 0 && (yx.y as usize) < self.height && (yx.x as usize) < self.width
    }

    fn cell(&self, yx : YX) -> u8 {
        if self.is_valid(yx) {
            self.bytes[yx.y as usize][yx.x as usize]
        } else {
            assert!(false, "map out of bounds");
            b' '
        }
    }

    fn push(&mut self, object: u8, start : YX, delta : YX) -> bool {
        let dest = start.add(delta);
        let pushed = match self.cell(dest) {
            b'#' => false,
            b'.' => true,
            b'O' => self.push(b'O', dest, delta),
            _ => {
                assert!(false, "bad push");
                false
            }
        };
        if pushed {
            self.bytes[dest.y as usize][dest.x as usize] = object;
            self.bytes[start.y as usize][start.x as usize] = b'.';
        }
        pushed
    }
}

fn main() {
    let use_example = 0;
    let width : usize;
    let height : usize;
    let filename;
    if use_example != 0 {
        filename = "/home/jgrills/adventofcode/2024/15/example.txt";
        width = 10;
        height = 10;
    } else {
        filename = "/home/jgrills/adventofcode/2024/15/input.txt";
        width = 50;
        height = 50;
    };
    let file_string = read_to_string(filename).unwrap();
    let mut map = Map{width, height, bytes:[[b' ';64];64]};

    let mut lines = file_string.lines();

    for y in 0..height {
        let line = lines.next().unwrap();
        let mut x = 0;
        for b in line.bytes() {
            map.bytes[y][x] = b;
            x += 1;
        }
    }

    let mut robot = YX{y:0,x:0};
    for y in 0..height as i32 {
        for x in 0..width as i32 {
            let yx = YX{y:y,x:x};
            if map.cell(yx) == b'@' {
                robot = yx;
                println!("start at {} {}", y, x);
                break;
            }
        }
    }

    for step_line in lines {
        for step in step_line.chars() {
            let dir = match step {
                '^' => UP,
                'v' => DOWN,
                '<' => LEFT,
                '>' => RIGHT,
                _ => {
                    assert!(false, "bad step");
                    DOWN
                }
            };
            if map.push(b'@', robot, dir) {
                robot = robot.add(dir);
            }
        }
    }

    let mut gps : usize = 0;
    for y in 0..height {
        for x in 0..width {
            print!("{}", map.bytes[y][x] as char);
            if map.bytes[y][x] == b'O' {
                gps += y * 100 + x;
            }
        }
        println!();
    }
    println!("gps {}", gps);
}
