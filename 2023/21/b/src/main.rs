use std::env;
use std::fs;
use std::mem;
use std::str;
use std::collections::HashSet;
use std::collections::BTreeMap;
use std::string::String;
use std::sync::{Arc, RwLock, Mutex};
use std::thread;
use std::sync::mpsc::{channel,Sender};

fn split_with_char(input: &str, with: char) -> (&str, &str) {
    match input.find(with) {
        Some(index) => (&input[0..index], &input[index+1..]),
        None => (input, "")
    }
}

fn blockid(y : i32, x : i32, h32 : i32, w32 : i32 ) -> YX {
    YX{
        y: if y < 0 { -(-y + (h32 - 1)) / h32 } else { y / h32 },
        x: if x < 0 { -(-x + (w32 - 1)) / w32 } else { x / w32 }
    }
}

fn mapyx(y : i32, x : i32, h32 : i32, w32 : i32 ) -> (usize, usize) {
        (
            ((if y < 0 { y + ((-y / h32) + 1) * h32 } else { y }) % h32) as usize,
            ((if x < 0 { x + ((-x / w32) + 1) * w32 } else { x }) % w32) as usize
        )
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

const THREADS : usize = 16;

const UPDATES : usize = 128;
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
    fn slice_mut<'a>(&'a mut self) -> &'a mut[YX] {
        let yxs = self.yxs;
        &mut self.yx[..yxs]
    }
}

const DIRS : usize = 5;
const DIRECTIONS : [YX; DIRS] = [YX{y:-1,x:0}, YX{y:1,x:0}, YX{y:0,x:-1}, YX{y:0,x:1},YX{y:0,x:0}];
const STEPS : usize= 4;
const STEP : [YX; STEPS] = [YX{y:-1,x:0}, YX{y:1,x:0}, YX{y:0,x:-1}, YX{y:0,x:1}];
struct BlockMutableData {
    map : Map,
    filled : usize,
    adds : Updates,
    relink : bool
}
impl Default for BlockMutableData {
    fn default() -> Self { Self { map: [[0; DIM];DIM], filled:0, adds: Updates::new(), relink: true } }
}
impl BlockMutableData  {
    fn new() -> Self { Default::default() }
    fn adds_clear(&mut self) {
        self.adds.clear();
    }
    fn adds_push(&mut self, s : YX) {
        let (y,x) = s.usize_tuple();
        if self.map[y][x] == 0 {
            self.map[y][x] = 1;
            self.adds.push(s);
            self.filled += 1;
        }
    }
    fn get_adds_slice<'a>(&'a self) -> &'a[YX] {
        self.adds.slice()
    }
}

struct BlockLocalData {
    inputs : [Arc<Mutex<Updates>>; DIRS],
    outputs : [Arc<Mutex<Updates>>; DIRS]
}
impl Default for BlockLocalData {
    fn default() -> Self {
        let local = Arc::new(Mutex::new(Updates::new()));
        Self {
            inputs: [
                Arc::new(Mutex::new(Updates::new())),
                Arc::new(Mutex::new(Updates::new())),
                Arc::new(Mutex::new(Updates::new())),
                Arc::new(Mutex::new(Updates::new())),
                local.clone(),
            ],
            outputs: [
                local.clone(),
                local.clone(),
                local.clone(),
                local.clone(),
                local,
            ]
        }
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
        let bd = self.data.write().unwrap();
        let inputs : &mut Updates = &mut bd.local.inputs[DIRS-1].lock().unwrap();
        inputs.push(start);
    }
    fn gather_adds(&mut self) {
        // read all the input threads while we are mutable
        let bd : &mut BlockData = &mut self.data.write().unwrap();
        let inputs = &bd.local.inputs;
        let ins : [&Updates; 5]= [
            &inputs[0].lock().unwrap(),
            &inputs[1].lock().unwrap(),
            &inputs[2].lock().unwrap(),
            &inputs[3].lock().unwrap(),
            &inputs[4].lock().unwrap()
        ];
        let mut adds : Updates = Updates::new();
        for inn in ins.iter()  {
            for yx in inn.slice() {
                adds.push(*yx);
            }
        }

        mem::swap(&mut adds, &mut bd.mutable.adds);
    }

    fn explore_neighbors(&self, h:i32, w:i32, map : &Map) {
        let bd = self.data.read().unwrap();
        let outputs = &bd.local.outputs;
        let mut outs : [&mut Updates; 5]= [
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

        for ad in bd.mutable.adds.slice() {
            for s in STEP.iter() {
                let out : &mut Updates;
                let mut n = ad.add(*s);
                if n.y < 0 {
                    n.y += h;
                    out = outs[0];
                } else if n.y >= h {
                    n.y -= h;
                    out = outs[1];
                } else if n.x < 0 {
                    n.x += w;
                    out = outs[2];
                } else if n.x >= w {
                    n.x -= w;
                    out = outs[3];
                } else {
                    out = outs[4];
                }
                let (ny, nx) = n.usize_tuple();
                if map[ny][nx] == 2 {

                    out.push(n);
                }
            }
        }
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

    let _print_map = | dests : &HashSet<YX> | {
        for y in 0..height {
            let y32 = y as i32;
            for x in 0..width  {
                let x32 = x as i32;
                let v = map[y][x];
                let out = match v {
                    1 => if dests.contains(&YX{y:y32,x:x32}) { panic!(); } else { '#' },
                    2 => if dests.contains(&YX{y:y32,x:x32}) { 'O' } else { '.' },
                    _ => panic!("unexpected value {}", v)
                };
                print!("{}", out);
            }
            println!();
        }
    };

    let mut blocks = BTreeMap::<YX, Block>::new();
    let mut active = BTreeMap::<YX, Block>::new();
    let w32 = width as i32;
    let h32 = height as i32;
    let mut initial = Block::new();
    initial.add_start(start);
    blocks.insert(YX::new(), initial);
    let mut initial = Block::new();
    initial.add_start(start);
    active.insert(YX::new(), initial);
    let mut txv : Vec<Sender<usize>> = Vec::new();
    txv.reserve(THREADS);

    let mut active_blocks = Vec::new();
    active_blocks.push(YX::new());

    for i in 0..THREADS {
        let (tx, rx) = channel::<usize>();
        txv.push(tx);
        thread::spawn(move|| {
            rx.recv().unwrap();
        });
    }

    let num_steps : i32 = 26501365;
    for _steps in 0..num_steps {
        for (_k,_v) in blocks.iter().enumerate() {

        }
    }
}
