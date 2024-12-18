use std::fs::read_to_string;

struct Map {
    height : usize,
    width : usize,
    bytes : String
}

impl Map {
    fn cell(&self, y : usize, x : usize) -> u8 {
        let index = (y * (self.width + 1)) + x;
        self.bytes.bytes().nth(index).unwrap()
    }
}

struct Search {
    cells : usize,
    edges : usize,
    claimed : [[bool; 160]; 160]
}

impl Search {
    fn claim(&mut self, map : &Map, ch : u8, y : usize, x : usize) {

        if map.cell(y,x) == ch && !self.claimed[y][x] {
            self.claimed[y][x] = true;
            self.cells += 1;

            if y == 0 {
                self.edges += 1;
            } else {
                if map.cell(y-1, x) != ch {
                    self.edges += 1;
                }
                else {
                    self.claim(map, ch, y-1, x);
                }
            }
            if y == map.height-1 {
                self.edges += 1;
            } else {
                if map.cell(y+1, x) != ch {
                    self.edges += 1;
                }
                else {
                    self.claim(map, ch, y+1, x);
                }
            }

            if x == 0 {
                self.edges += 1;
            } else {
                if map.cell(y, x-1) != ch {
                    self.edges += 1;
                }
                else {
                    self.claim(map, ch, y, x-1);
                }
            }

            if x == map.width-1 {
                self.edges += 1;
            } else {
                if map.cell(y, x+1) != ch {
                    self.edges += 1;
                }
                else {
                    self.claim(map, ch, y, x+1);
                }
            }
        }
    }

    fn partition(&mut self, map : &Map) {
        let mut total = 0;
        for y in 0..map.height {
            for x in 0..map.width {
                if self.claimed[y][x] == false {
                    self.cells = 0;
                    self.edges = 0;
                    self.claim(map, map.cell(y,x,), y, x);
                    let part = self.cells * self.edges;
                    total += part;
                    println!("Region {} : {} {} : {} -> {}", map.cell(y,x) as char, self.cells, self.edges, part, total);
                }
            }
        }
    }
}

fn main() {
    let use_example = 0;
    let map : Map;
    if use_example != 0 {
        map = Map{height:10, width:10, bytes:read_to_string("/home/jgrills/adventofcode/2024/12/example.txt").unwrap()};
    } else {
        map = Map{height:140, width:140, bytes:read_to_string("/home/jgrills/adventofcode/2024/12/input.txt").unwrap()};
    };

    let mut search = Search{cells:0, edges:0, claimed:[[false; 160]; 160]};
    search.partition(&map);
}
