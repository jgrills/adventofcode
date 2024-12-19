use std::fs::read_to_string;
use std::collections::BTreeSet;

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

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Edge {
    y : usize, x : usize, side : usize
}

struct Search {
    cells : usize,
    edges : usize,
    panels : usize,
    claimed : [[bool; 160]; 160],
    edge_list : BTreeSet<Edge>
}


impl Search {
    fn claim(&mut self, map : &Map, ch : u8, y : usize, x : usize) {

        if map.cell(y,x) == ch && !self.claimed[y][x] {
            self.claimed[y][x] = true;
            self.cells += 1;

            if y == 0 {
                self.edge_list.insert(Edge{y,x,side:1});
                self.edges += 1;
            } else {
                if map.cell(y-1, x) != ch {
                    self.edge_list.insert(Edge{y,x,side:1});
                    self.edges += 1;
                }
                else {
                    self.claim(map, ch, y-1, x);
                }
            }
            if y == map.height-1 {
                self.edge_list.insert(Edge{y,x,side:2});
                self.edges += 1;
            } else {
                if map.cell(y+1, x) != ch {
                    self.edge_list.insert(Edge{y,x,side:2});
                    self.edges += 1;
                }
                else {
                    self.claim(map, ch, y+1, x);
                }
            }

            if x == 0 {
                self.edge_list.insert(Edge{y,x,side:3});
                self.edges += 1;
            } else {
                if map.cell(y, x-1) != ch {
                    self.edge_list.insert(Edge{y,x,side:3});
                    self.edges += 1;
                }
                else {
                    self.claim(map, ch, y, x-1);
                }
            }

            if x == map.width-1 {
                self.edge_list.insert(Edge{y,x,side:4});
                self.edges += 1;
            } else {
                if map.cell(y, x+1) != ch {
                    self.edge_list.insert(Edge{y,x,side:4});
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
                    self.panels = 0;
                    self.edge_list.clear();
                    self.claim(map, map.cell(y,x,), y, x);
                    while let Some(Edge{y,x,side}) = self.edge_list.pop_first() {
                        //println!("consider  edge {} {} {}", y, x, side);
                        self.panels += 1;
                        let (dy,dx) = if side == 1 || side == 2 { (0,1) } else { (1,0) };
                        let mut step = 1;
                        loop {
                            let (ny, nx) = (y + dy * step, x + dx * step);
                            if !self.edge_list.remove(&Edge{y:ny,x:nx,side}) {  break; }
                            //println!("remove edge {} {} {}", ny, nx, side);
                            step += 1;
                        }
                        step = 1;
                        loop {
                            let (sy, sx) = (dy * step, dx * step);
                            if sy > y || sx > x { break; }
                            let (ny, nx) = (y - sy, x - sx);
                            if !self.edge_list.remove(&Edge{y:ny,x:nx,side}) { break; }
                            //println!("remove edge {} {} {}", ny, nx, side);
                            step += 1;
                        }
                    }
                    let part = self.cells * self.panels;
                    total += part;
                    println!("Region {} : {} {} : {} -> {}", map.cell(y,x) as char, self.cells, self.panels, part, total);
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

    let mut search = Search{cells:0, edges:0, panels:0, claimed:[[false; 160]; 160], edge_list:BTreeSet::new()};
    search.partition(&map);
}
