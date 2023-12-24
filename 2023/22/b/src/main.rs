use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops;
use std::path::Path;

#[derive(Copy, Clone, Eq, PartialEq)]
struct XYZ {
    x: i32,
    y: i32,
    z: i32
}

impl ops::Add for XYZ {
    type Output = XYZ;
    fn add(self, rhs: Self) -> XYZ {
        return XYZ{x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z};
    }
}

impl ops::AddAssign for XYZ {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        };
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Brick {
    start: XYZ,
    end: XYZ
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct BrickStep {
    brick: Brick,
    step: XYZ
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn step(b: Brick) -> XYZ {
    let mut r = XYZ{ x:0, y:0, z:0 };
    if b.end.x > b.start.x { r.x =  1; }
    else { if b.end.x < b.start.x { r.x = -1; } }
    if b.end.y > b.start.y { r.y =  1; }
    else { if b.end.y < b.start.y { r.y = -1; } }
    if b.end.z > b.start.z { r.z =  1; }
    else { if b.end.z < b.start.z { r.z = -1; } }
    return r;
}

const UP : XYZ = XYZ{ x: 0, y: 0, z: 1 };
const DOWN : XYZ = XYZ{ x: 0, y: 0, z: -1 };
fn supports(top: BrickStep, bottom: BrickStep) -> bool {
    let mut p0 : XYZ = top.brick.start;
    let e0 = top.brick.end;
    let s0 = top.step;
    loop {
        let p0s = p0 + DOWN;

        let mut p1 : XYZ = bottom.brick.start;
        let e1 = bottom.brick.end;
        let s1 = bottom.step;
        loop {
            if p1 == p0s {
                return true;
            }
            if p1 == e1 { break; }
            p1 += s1;
        }
        if p0 == e0 { break; }
        if s0 == UP { break; }
        p0 += s0;
    }
    return false;
}

fn supported(top: BrickStep, bottom: BrickStep) -> bool {
    return top.brick.start.z == 1 || supports(top, bottom);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    const DIM : usize = 2000;
    const BS : BrickStep = BrickStep{ brick: Brick{start: XYZ{x:0,y:0,z:0}, end: XYZ{x:0,y:0,z:0}}, step: XYZ{x:0,y:0,z:0} };
    let mut brick : [BrickStep; DIM] = [BS; DIM];
    let mut bricks : usize = 0;

    println!("processing input");

    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(contents) = line {
                let v: Vec<&str> = contents.split('~').collect();
                let v0: Vec<&str> = v[0].split(',').collect();
                let v1: Vec<&str> = v[1].split(',').collect();
                let xyz0 = XYZ{ x: v0[0].parse::<i32>().unwrap(), y: v0[1].parse::<i32>().unwrap(), z: v0[2].parse::<i32>().unwrap() };
                let xyz1 = XYZ{ x: v1[0].parse::<i32>().unwrap(), y: v1[1].parse::<i32>().unwrap(), z: v1[2].parse::<i32>().unwrap() };
                let b = Brick{ start: xyz0, end: xyz1 };
                let s : XYZ = step(b);
                brick[bricks] = BrickStep{brick: b, step: s};
                bricks += 1;
            }
            else {
                panic!();
            }
        }
    }

    println!("processing fall");

    let mut moved : bool = true;
    while moved {
        moved = false;
        for i in 0..bricks {
            let b = brick[i];
            let mut sup : bool = false;
            for j in 0..bricks {
                if i != j && supported(b, brick[j]) {
                    sup = true;
                    break;
                }
            }
            if !sup {
                // println!("  fell: {}", i);
                brick[i].brick.start += DOWN;
                brick[i].brick.end += DOWN;
                moved = true;
            }
        }
    }

    println!("processing deletes");
    let orig : [BrickStep; DIM] = brick;

    let mut result : i32 = 0;
    for d in 0..bricks {
        brick = orig;

        let mut moved : bool = true;
        while moved {
            moved = false;
            for i in 0..bricks {
                if i == d { continue; }
                let b = brick[i];
                let mut sup : bool = false;
                for j in 0..bricks {
                    if i == j || j == d { continue; }
                    if supported(b, brick[j]) {
                        sup = true;
                        break;
                    }
                }
                if !sup {
                    // println!("  fell: {}", i);
                    brick[i].brick.start += DOWN;
                    brick[i].brick.end += DOWN;
                    moved = true;
                }
            }
        }

        let mut local : i32 = 0;
        for i in 0..bricks {
            if brick[i] != orig[i] { local += 1; result += 1; }
        }
        println!("delete {} fall {}", d, local);

    }

    println!("result {}", result);
}
