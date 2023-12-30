use std::env;
use std::fs;
use std::str;
use std::collections::HashSet;
use std::collections::BTreeMap;
use std::string::String;
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
struct YX { y:i32, x:i32 }

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


fn main() {
    const DIM : usize = 140;
    let mut height : usize = 0;
    let mut width : usize = 0;
    type Map = [[u8; DIM]; DIM];
    let mut map : Map = [[0; DIM]; DIM];

    // grab the first command line argument, use it as a filename and load that file into a string
    let file_path : String = env::args().nth(1).unwrap();
    let file_contents : String = match fs::read_to_string(file_path.clone()) {
        Ok(fc) => fc,
        Err(..) => panic!("couldn't read {}", file_path)
    };

    // get str for the rest of the file contents left to process
    let mut start =  YX{y:0,x:0};
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

    let mut blocks = BTreeMap::<YX, Map>::new();
    let mut frontier = Vec::<YX>::new();
    let mut frontier_new = Vec::<YX>::new();
    let mut removals = Vec::<YX>::new();
    let mut spots: usize = 0;
    let mut spots_new : usize = 0;
    let w32 = width as i32;
    let h32 = height as i32;
    frontier.reserve(10000000);
    frontier_new.reserve(10000000);
    removals.reserve(100000);
    frontier.push(start.clone());
    map[DIM-1][DIM-1] = 1;
    blocks.insert(YX{y:0,x:0}, map.clone());

    let step = | y : i32, x : i32, blocks : &mut BTreeMap<YX, Map>, frontier : &mut Vec<YX>, spots : &mut usize | {
        let byx = blockid(y, x, h32, w32);
        let bl = match blocks.get_mut(&byx) {
            Some(x) => x, 
            None => {
                // println!("adding block {} {}", byx.y, byx.x);
                blocks.insert(byx.clone(), map.clone());
                match blocks.get_mut(&byx) {
                    Some(x) => x, 
                    None => panic!()
                }
            }
        };

        bl[DIM-1][DIM-1] = 1;

        let (my , mx) = mapyx(y, x, h32, w32);
        if bl[my][mx] == 2 {
            bl[my][mx] = 4;
            *spots += 1;
            frontier.push(YX{y,x});
        }
    };

    let mut remove_count: usize = 0;
    let num_steps : i32 = 26501365;
    for steps in 0..num_steps {
        for yx in &frontier {
            let y = yx.y;
            let x = yx.x;
            let byx = blockid(y, x, h32, w32);
            let bl : &mut Map = match blocks.get_mut(&byx) {
                Some(x) => x, 
                None =>  panic!("{} {} {} {} -> {} {}", y,x,h32,w32,byx.y,byx.x)
            };
            bl[DIM-1][DIM-1] = 1;
            
            step(y-1, x  , &mut blocks, &mut frontier_new, &mut spots_new);
            step(y  , x-1, &mut blocks, &mut frontier_new, &mut spots_new);
            step(y+1, x  , &mut blocks, &mut frontier_new, &mut spots_new);
            step(y  , x+1, &mut blocks, &mut frontier_new, &mut spots_new);
        }

        for (k, ab) in blocks.iter_mut() {
            if ab[DIM-1][DIM-1] == 0 {
                removals.push(*k);
            } else {
                ab[DIM-1][DIM-1] = 0; 
            }
        }
        for &bxy in &removals {
            blocks.remove(&bxy);
        };
        remove_count += removals.len();
        removals.clear();

        if (steps+1) % 100 == 0 {

            let finished = (((steps+1)*100) as f32) / num_steps as f32; 
            let mut spots_buffer : [u8; 16] = [0;16];
            let mut blocks_buffer : [u8; 16] = [0;16];
            println!("{:.3}% {}=turn {}=spots {}=blocks {}=frontier {}=removals", finished, steps+1, commaify(spots_new, &mut spots_buffer), commaify(blocks.len(), &mut blocks_buffer), frontier_new.len(), remove_count);
            remove_count = 0;
        }

        //print_map(&active);
        std::mem::swap(&mut frontier, &mut frontier_new);
        std::mem::swap(&mut spots, &mut spots_new);
        frontier_new.clear();
    }
    let mut spots_buffer : [u8; 16] = [0;16];
    let mut blocks_buffer : [u8; 16] = [0;16];
    println!("{}=spots {}=blocks {}=frontier", commaify(spots, &mut spots_buffer), commaify(blocks.len(), &mut blocks_buffer), frontier.len());
}
