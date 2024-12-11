use std::fs::read_to_string;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
struct YX { y:i32, x:i32 }
impl YX {
    fn add(&self, rhs : Self) -> Self { Self{ y:self.y + rhs.y, x: self.x + rhs.x } }
    fn sub(&self, rhs : Self) -> Self { Self{ y:self.y - rhs.y, x: self.x - rhs.x } }
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
}

fn is_antenna(ch : u8) -> bool {
    match ch {
        b'a' ..= b'z' | b'A' ..= b'Z' | b'0' ..= b'9' => true,
        _ => false
    }
}

fn main() {
    let use_example = 0;
    let m : Map;
    if use_example != 0 {
        m = Map{ width:12, height:12, bytes:read_to_string("/home/jgrills/adventofcode/2024/8/example.txt").unwrap() };
    } else {
        m = Map{ width:50, height:50, bytes:read_to_string("/home/jgrills/adventofcode/2024/8/input.txt").unwrap() };
    };

    let mut antinode = [[false; 60]; 60];
    let mut antinodes = 0;
    for y0 in 0..m.height as i32 {
        for x0 in 0..m.width as i32 {
            let p0 = YX{y:y0, x:x0};
            let b0 = m.map(p0);
            if is_antenna(b0) {
                //println!("Antenna {} {} {}", b0 as char, y0, x0);
                for y1 in y0..m.height as i32 {
                    for x1 in 0..m.width as i32 {
                        if y0 == y1 && x1 <= x0 { continue; }
                        let p1 = YX{y:y1, x:x1};
                        let b1 = m.map(p1);
                        if b0 == b1 {
                            //println!("  Antenna pair {} {}", y1, x1);
                            let delta = p1.sub(p0);
                            //println!("    Antinode {} {}", a0.y, a0.x);
                            //println!("    Antinode {} {}", a1.y, a1.x);

                            let a0 = p0.sub(delta);
                            let a0y = a0.y as usize;
                            let a0x = a0.x as usize;
                            if m.is_valid(a0) && !antinode[a0y][a0x] {
                                antinode[a0y][a0x] = true;
                                antinodes += 1;
                                println!("{} Antinode0 {} {}", antinodes, a0y, a0x);
                        }

                            let a1 = p1.add(delta);
                            let a1y = a1.y as usize;
                            let a1x = a1.x as usize;
                            if m.is_valid(a1) && !antinode[a1y][a1x] {
                                antinode[a1y][a1x] = true;
                                antinodes += 1;
                                println!("{} Antinode1 {} {}", antinodes, a1y, a1x);
                            }
                        }
                    }
                }
            }
        }
    }
}
