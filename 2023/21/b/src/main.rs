use std::env;
use std::fs;
use std::mem;
use std::str;
use std::collections::BTreeSet;
use std::collections::BTreeMap;
use std::string::String;
use std::sync::{Arc, RwLock, Mutex};
//use std::thread;
//use std::sync::mpsc::{channel,Sender};

fn split_with_char(input: &str, with: char) -> (&str, &str) {
    match input.find(with) {
        Some(index) => (&input[0..index], &input[index+1..]),
        None => (input, "")
    }
}

fn commaify( mut n : usize, output : &mut[u8] ) -> &str {
    let mut three = 0;
    let mut start = output.len();
    while n != 0 {
        if three == 3 {
            three = 0;
            if start == 0 { panic!("buffer too small"); }
            start -= 1;
            output[start] = b',';
        }
        three += 1;
        let digit = (n % 10) as u8;
        n = n / 10;
        if start == 0 { panic!("buffer too small"); }
        start -= 1;
        output[start] = b'0' + digit;
    }
    str::from_utf8(&output[start..]).unwrap()
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
struct YX { y:i32, x:i32 }
impl YX {
    fn new() -> Self { Default::default() }
    fn add(&self, rhs : Self) -> Self { Self{ y:self.y + rhs.y, x: self.x + rhs.x } }
    fn usize_tuple(&self) -> (usize,usize) { ( self.y as usize, self.x as usize) }
}
impl Default for YX {
    fn default() -> Self { Self{ y:0, x:0 } }
}

const DIM : usize = 140;
type Map = [[u8; DIM]; DIM];

const UPDATES : usize = 512;
struct Updates {
    yx : [YX; UPDATES],
    yxs : usize
}
impl Default for Updates {
    fn default() -> Self { Self { yx: [YX::new(); UPDATES], yxs:0 } }
}
impl Updates {
    fn new() -> Self { Default::default() }
    fn clear(&mut self) { self.yxs = 0; }
    fn push(&mut self, new_item : YX) {
        self.yx[self.yxs] = new_item;
        self.yxs += 1;
    }
    fn slice<'a>(&'a self) -> &'a[YX] {
        let yxs = self.yxs;
        &self.yx[..yxs]
    }
}

const STEPS : usize = 4;
const STEP : [YX; STEPS] = [YX{y:-1,x:0}, YX{y:1,x:0}, YX{y:0,x:-1}, YX{y:0,x:1}];

// includes 0,0
const DIRS : usize = 5;

struct BlockMutableData {
    map : Map,
    filled : [usize; 2],
    adds : Updates
}
impl Default for BlockMutableData {
    fn default() -> Self { Self { map: [[0; DIM];DIM], filled:[0,0], adds: Updates::new()} }
}
impl BlockMutableData  {
    fn new() -> Self { Default::default() }
}

struct BlockLocalData {
    inputs : [Arc<Mutex<Updates>>; DIRS],
    outputs : [Arc<Mutex<Updates>>; DIRS]
}
impl BlockLocalData  {
}
impl Default for BlockLocalData {
    fn default() -> Self {
        let empty_updates = Arc::new(Mutex::new(Updates::new()));
        let outputs = [
            Arc::new(Mutex::new(Updates::new())),
            Arc::new(Mutex::new(Updates::new())),
            Arc::new(Mutex::new(Updates::new())),
            Arc::new(Mutex::new(Updates::new())),
            Arc::new(Mutex::new(Updates::new())),
        ];
        let inputs = [
            Arc::clone(&empty_updates),
            Arc::clone(&empty_updates),
            Arc::clone(&empty_updates),
            Arc::clone(&empty_updates),
            Arc::clone(&outputs[4])
        ];
        Self { inputs, outputs }
    }
}
impl BlockLocalData  {
    fn new() -> Self { Default::default() }
}

struct BlockData {
    mutable : BlockMutableData,
    local : BlockLocalData
}
impl Default for BlockData {
    fn default() -> Self { Self { mutable: BlockMutableData::new(), local: BlockLocalData::new() } }
}
impl BlockData  {
    fn new() -> Self { Default::default() }
}

struct Block {
    data : Arc<RwLock<BlockData>>
}
impl Default for Block {
    fn default() -> Self { Self{ data: Arc::new(RwLock::new(BlockData::new())) }}
}
impl Block {
    fn new() -> Self {
        Default::default()
    }
    fn add_start(&mut self, start: YX) {
        let bd : &mut BlockData = &mut self.data.write().unwrap();
        let (y,x) = start.usize_tuple();
        bd.mutable.adds.push(start);
        bd.mutable.map[y][x] = 1;
        bd.mutable.filled[1] = 1;
    }
    fn gather_adds(&self, step : usize) -> usize {
        // read all the input threads while we are mutable
        let bd : &mut BlockData = &mut self.data.write().unwrap();
        let inputs = &bd.local.inputs;
        let ins : [&Updates; 5] = [
            &inputs[0].lock().unwrap(),
            &inputs[1].lock().unwrap(),
            &inputs[2].lock().unwrap(),
            &inputs[3].lock().unwrap(),
            &inputs[4].lock().unwrap()
        ];
        let mut adds : Updates = Updates::new();
        for input_updates in ins.iter() {
            for yx in input_updates.slice() {
                let (y,x) = yx.usize_tuple();
                if bd.mutable.map[y][x] == 0 {
                    adds.push(*yx);
                    bd.mutable.map[y][x] = 1;
                    bd.mutable.filled[step & 1usize] += 1;
                }
            }
        }

        mem::swap(&mut adds, &mut bd.mutable.adds);
        println!("    outgoing {}=active {}=filled", bd.mutable.adds.yxs, bd.mutable.filled[step & 1usize]);
        bd.mutable.adds.yxs
    }
    fn num_reachable(&self, step : usize) -> usize {
        let bd : &BlockData = &self.data.write().unwrap();
        bd.mutable.filled[step & 1usize]
    }

    fn explore_neighbors(&self, step:usize, byx: YX, h:i32, w:i32, map : &Map) -> Updates {

        let mut expand : Updates = Updates::new();
        let bd : & BlockData = &self.data.read().unwrap();
        println!("    incoming {}=active {}=filled", bd.mutable.adds.yxs, bd.mutable.filled[step & 1usize]);
        let outputs = &bd.local.outputs;
        let mut outs : [&mut Updates; 5] = [
            &mut outputs[0].lock().unwrap(),
            &mut outputs[1].lock().unwrap(),
            &mut outputs[2].lock().unwrap(),
            &mut outputs[3].lock().unwrap(),
            &mut outputs[4].lock().unwrap()
        ];

        // reset outputs before generating new ones
        for o in outs.iter_mut() {
            o.clear();
        }
        let mut local_map : Map = [[0; DIM]; DIM];
        for ad in bd.mutable.adds.slice() {
            //println!("  {},{}=ad ", ad.y,ad.x);
            for s in STEP.iter() {
                let out : &mut Updates;
                let mut n = ad.add(*s);
                if n.y < 0 {
                    n.y += h;
                    out = outs[0];
                    expand.push(byx.add(*s));
                } else if n.y >= h {
                    n.y -= h;
                    out = outs[1];
                    expand.push(byx.add(*s));
                } else if n.x < 0 {
                    n.x += w;
                    out = outs[2];
                    expand.push(byx.add(*s));
                } else if n.x >= w {
                    n.x -= w;
                    out = outs[3];
                    expand.push(byx.add(*s));
                } else {
                    out = outs[4];
                }
                let (ny, nx) = n.usize_tuple();
                //println!("  {},{}=ny,nx ", ny,nx);
                if map[ny][nx] == 2 && bd.mutable.map[ny][nx] == 0 && local_map[ny][nx] == 0 {
                    //println!("  pushing");
                    local_map[ny][nx] = 1;
                    out.push(n);
                } else {
                    //println!("  skipping");
                }
            }
        }
        expand
    }
}

fn main() {
    let mut map : Map = [[0; DIM]; DIM];
    let mut height : usize = 0;
    let mut width : usize = 0;

    // grab the first command line argument, use it as a filename and load that file into a string
    let file_path : String = env::args().nth(1).unwrap();
    let file_contents : String = match fs::read_to_string(file_path.clone()) {
        Ok(fc) => fc,
        Err(..) => panic!("couldn't read {}", file_path)
    };

    // get str for the rest of the file contents left to process
    let mut start = YX::new();
    let mut rest : &str = file_contents.as_str();
    while !rest.is_empty() {
        let line : &str;
        (line, rest) = split_with_char(rest, '\n');
        for (x, c) in line.chars().enumerate() {
            map[height][x] = match c {
                '.' => 2,
                'S' => {
                    start = YX{ y:height as i32, x:x as i32};
                    2
                },
                '#' => 1,
                _ => panic!("unknown map element")
            }
        }
        if width == 0 { width = line.len(); }
        height += 1;
    }

    let mut blocks = BTreeMap::<YX, Block>::new();
    let mut active = BTreeSet::<YX>::new();
    let w32 = width as i32;
    let h32 = height as i32;
    let mut initial = Block::new();
    initial.add_start(start);
    blocks.insert(YX::new(), initial);
    active.insert(YX::new());

    let num_steps : usize = 26501365;
    for step in 0..num_steps {
        println!("{}=step", step);
        let mut expand : Updates = Updates::new();
        for byx in active.iter() {
            println!("  {},{}=byx", byx.y, byx.x);
            match blocks.get(byx) {
                Some(block) => {
                    let block_expand : Updates = block.explore_neighbors(step, *byx, h32, w32, &map);
                    for e in block_expand.slice() {
                        expand.push(*e);
                    }
                },
                None => panic!()
            };
        }

        for byx in expand.slice() {
            println!("  {},{}=expand", byx.y, byx.x);
            match blocks.get(byx) {
                Some(_) => {
                    println!("    already present");
                    true
                },
                None => {
                    println!("    creating");
                    let nb = Block::new();
                    {
                        let nbd : &mut BlockData = &mut nb.data.write().unwrap();
                        let inputs = &mut nbd.local.inputs;
                        match blocks.get(&byx.add(STEP[0])) {
                            Some(other) => {
                                let other_bd : &BlockData = &other.data.write().unwrap();
                                inputs[0] = Arc::clone(&other_bd.local.outputs[1]);
                                true
                            },
                            None => false
                        };
                        match blocks.get(&byx.add(STEP[1])) {
                            Some(other) => {
                                let other_bd : &mut BlockData = &mut other.data.write().unwrap();
                                inputs[1] = Arc::clone(&other_bd.local.outputs[0]);
                                true
                            },
                            None => false
                        };
                        match blocks.get(&byx.add(STEP[2])) {
                            Some(other) => {
                                let other_bd : &mut BlockData = &mut other.data.write().unwrap();
                                inputs[2] = Arc::clone(&other_bd.local.outputs[3]);
                                true
                            },
                            None => false
                        };
                        match blocks.get(&byx.add(STEP[3])) {
                            Some(other) => {
                                let other_bd : &mut BlockData = &mut other.data.write().unwrap();
                                inputs[3] = Arc::clone(&other_bd.local.outputs[2]);
                                true
                            },
                            None => false
                        };
                    }
                    {
                        let nbd : &BlockData = &nb.data.read().unwrap();
                        let outputs = &nbd.local.outputs;
                        match blocks.get(&byx.add(STEP[0])) {
                            Some(other) => {
                                let other_bd : &mut BlockData = &mut other.data.write().unwrap();
                                other_bd.local.inputs[1] = Arc::clone(&outputs[0]);
                                true
                            },
                            None => false
                        };
                        match blocks.get(&byx.add(STEP[1])) {
                            Some(other) => {
                                let other_bd : &mut BlockData = &mut other.data.write().unwrap();
                                other_bd.local.inputs[0] = Arc::clone(&outputs[1]);
                                true
                            },
                            None => false
                        };
                        match blocks.get(&byx.add(STEP[2])) {
                            Some(other) => {
                                let other_bd : &mut BlockData = &mut other.data.write().unwrap();
                                other_bd.local.inputs[3] = Arc::clone(&outputs[2]);
                                true
                            },
                            None => false
                        };
                        match blocks.get(&byx.add(STEP[3])) {
                            Some(other) => {
                                let other_bd : &mut BlockData = &mut other.data.write().unwrap();
                                other_bd.local.inputs[2] = Arc::clone(&outputs[3]);
                                true
                            },
                            None => false
                        };
                        }
                    blocks.insert(*byx, nb);
                    active.insert(*byx);
                    false
                }
            };
        }

        let mut filled : usize = 0;
        let mut removes : Updates = Updates::new();
        for byx in active.iter() {
            filled += match blocks.get(byx) {
                Some(block) => {
                    let active = block.gather_adds(step);
                    let filled = block.num_reachable(step);
                    if filled != 0 && active == 0 {
                        removes.push(*byx);
                    }
                    filled
                },
                None => panic!()
            };
        }

        for r in removes.slice() {
            println!("  {},{}=inactive block", r.y, r.x);
            active.remove(r);
        }
        removes.clear();

        println!("  {}=filled", filled);
    }
}

