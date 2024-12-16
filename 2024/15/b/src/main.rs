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
    bytes : [[u8; 128]; 64]
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
        self.bytes[yx.y as usize][yx.x as usize]
    }

    fn set_cell(&mut self, yx : YX, val : u8) {
        self.assert_valid(yx);
        self.bytes[yx.y as usize][yx.x as usize] = val;
    }

    fn can_push_left(&self, start : YX) -> bool {
        let dest = start.add(LEFT);
        match self.cell(dest) {
            b'#' => false,
            b'.' => true,
            b']' => {
                let other_half = dest.add(LEFT);
                assert!(self.cell(other_half) == b'[', "broken box");
                self.can_push_left(other_half)
            },
            _ => { assert!(false, "bad push"); false }
        }
    }

    fn can_push_right(&self, start : YX) -> bool {
        let dest = start.add(RIGHT);
        match self.cell(dest) {
            b'#' => false,
            b'.' => true,
            b'[' => {
                let other_half = dest.add(RIGHT);
                assert!(self.cell(other_half) == b']', "broken box");
                self.can_push_right(other_half)
            },
            _ => { assert!(false, "bad push"); false }
        }
    }

    fn do_push_horizontal(&mut self, start : YX, dir : YX) {
        let o = self.cell(start);
        let dest = start.add(dir);
        match self.cell(dest) {
            b'#' => { assert!(false, "wall in push"); },
            b'.' => self.set_cell(dest, o),
            b'[' | b']' => {
                self.do_push_horizontal(dest, dir);
                self.set_cell(dest, o);
            },
            _ => { assert!(false, "bad cell"); }
        }
        self.set_cell(start, b'.');
    }

    fn can_push_vertical(&self, start : YX, delta : YX) -> bool {
        let dest = start.add(delta);
        match self.cell(dest) {
            b'#' => false,
            b'.' => true,
            b'[' => self.can_push_vertical(dest, delta) && self.can_push_vertical(dest.add(RIGHT), delta),
            b']' => self.can_push_vertical(dest, delta) && self.can_push_vertical(dest.add(LEFT), delta),
            _ => { assert!(false, "bad push"); false }
        }
    }

    fn do_push_vertical(&mut self, start : YX, delta : YX) {
        let o = self.cell(start);
        let dest = start.add(delta);
        match self.cell(dest) {
            b'#' => assert!(false, "wall in push"),
            b'.' => self.set_cell(dest, o),
            b'[' => {
                self.do_push_vertical(dest, delta);
                self.do_push_vertical(dest.add(RIGHT), delta);
                self.set_cell(dest, o);
            },
            b']' => {
                self.do_push_vertical(dest, delta);
                self.do_push_vertical(dest.add(LEFT), delta);
                self.set_cell(dest, o);
            },
            _ => { assert!(false, "bad cell"); }
        }
        self.set_cell(start, b'.');
    }
}

fn main() {
    let use_example = 0;
    let width : usize;
    let height : usize;
    let filename;
    if use_example != 0 {
        filename = "/home/jgrills/adventofcode/2024/15/example.txt";
        width = 10 * 2;
        height = 10;
    } else {
        filename = "/home/jgrills/adventofcode/2024/15/input.txt";
        width = 50 * 2;
        height = 50;
    };
    let file_string = read_to_string(filename).unwrap();
    let mut map = Map{width, height, bytes:[[b' ';128];64]};

    let mut lines = file_string.lines();

    for y in 0..height {
        let line = lines.next().unwrap();
        let mut x = 0;
        for b in line.bytes() {
            match b {
                b'#' | b'.' => { map.bytes[y][x+0] = b; map.bytes[y][x+1] = b; },
                b'O' => { map.bytes[y][x+0] = b'['; map.bytes[y][x+1] = b']'; },
                b'@' => { map.bytes[y][x+0] = b; map.bytes[y][x+1] = b'.'; },
                _ => { assert!(false, "bad input"); }
            };
            x += 2;
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
            match step {
                '^' =>
                    if map.can_push_vertical(robot, UP) {
                        map.do_push_vertical(robot, UP);
                        robot = robot.add(UP);
                    },
                'v' =>
                    if map.can_push_vertical(robot, DOWN) {
                        map.do_push_vertical(robot, DOWN);
                        robot = robot.add(DOWN);
                    },
                '<' =>
                    if map.can_push_left(robot) {
                        map.do_push_horizontal(robot, LEFT);
                        robot = robot.add(LEFT);
                    },
                '>' =>
                    if map.can_push_right(robot) {
                        map.do_push_horizontal(robot, RIGHT);
                        robot = robot.add(RIGHT);
                    },
                _ => {
                    assert!(false, "bad step");
                }
            };
        }
    }

    let mut gps : usize = 0;
    for y in 0..height {
        for x in 0..width {
            print!("{}", map.bytes[y][x] as char);
            if map.bytes[y][x] == b'[' {
                gps += y * 100 + x;
            }
        }
        println!();
    }
    println!("gps {}", gps);
}
