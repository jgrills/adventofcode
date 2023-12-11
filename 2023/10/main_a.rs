use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
struct XY {
    x: i32,
    y: i32
}

// Provide defaults for NumberData
impl Default for XY {
    fn default() -> XY {
        XY {
            x: 0,
            y: 0        }
    }
}

struct Connection {
    data: [XY;2]
}

//| is a vertical pipe connecting north and south.
//- is a horizontal pipe connecting east and west.
//L is a 90-degree bend connecting north and east.
//J is a 90-degree bend connecting north and west.
//7 is a 90-degree bend connecting south and west.
//F is a 90-degree bend connecting south and east.


#[derive(Copy, Clone)]
struct Cell {
    text: char,
    distance: i32
}

impl Default for Cell {
    fn default() -> Cell {
        Cell {
            text:  ' ',
            distance: -1        }
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn main() {
    let mut width : i32 = 0;
    let mut y : i32 = 1;
    let mut map: [[Cell; 200]; 200] = [[Cell{..Default::default() };200];200];

    let mut updates: [[XY; 20000]; 2] = [[XY{..Default::default() };20000]; 2];
    let mut n0 = 0;
    let mut n1 = 0;
    let mut n = 0;

    let mut connections :  HashMap::<char, Connection> = HashMap::new();
    connections.insert('|', Connection{data:[XY{x:  0, y: -1}, XY{x:  0, y:  1}]});
    connections.insert('-', Connection{data:[XY{x: -1, y:  0}, XY{x:  1, y:  0}]});
    connections.insert('L', Connection{data:[XY{x:  1, y:  0}, XY{x:  0, y: -1}]});
    connections.insert('J', Connection{data:[XY{x: -1, y:  0}, XY{x:  0, y: -1}]});
    connections.insert('7', Connection{data:[XY{x: -1, y:  0}, XY{x:  0, y:  1}]});
    connections.insert('F', Connection{data:[XY{x:  1, y:  0}, XY{x:  0, y:  1}]});

    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(contents) = line {
                if width == 0 {
                    width = contents.len() as i32 + 2;
                }
                println!("line {} {}", contents, contents.len());
                for (x, c) in contents.chars().enumerate() {
                    map[y as usize][x+1 as usize].text = c;
                    if c == 'S' {
                        updates[0][0].x = x as i32 + 1;
                        updates[0][0].y = y;
                        n1 = 1;
                        n = 1;
                        map[y as usize][x+1 as usize].distance = 0;
                    }
                }
            }
            y += 1;
        }
    }

    for y2 in 0..y {
        for x2 in 0..width {
            if map[y2 as usize][x2 as usize].distance < 0 {
                print!(".");
            }
            else {
                print!("{}", map[y2 as usize][x2 as usize].distance);
            }
        }
        println!("");
    }

    for d in 0..140*140 {
        let mut updated = false;
        println!("top: d {} n1 {} n {}", d, n1, n);
        for q0 in 0..n1 {
            let cxy = updates[1-n][q0];
            println!("  q0 {} x {} y {}", q0, cxy.x, cxy.y);

            for y2 in cxy.y-1 .. cxy.y+2 {
                for x2 in cxy.x-1 .. cxy.x+2 {
                    println!("  x2 {} y2 {}", x2, y2);

                    let c = connections.get(&map[y2 as usize][x2 as usize].text);
                    match c {
                        None    => {},

                        //
                        Some(xys) => {
                            
                            if map[y2 as usize][x2 as usize].distance < 0 {
                                for ci in 0 .. 2 {

                                    let x3 = x2 + xys.data[ci].x;
                                    let y3 = y2 + xys.data[ci].y;
                                    if map[y3 as usize][x3 as usize].distance == d {
                                        map[y2 as usize][x2 as usize].distance = d+1;
                                        println!("update: x {} y {} n0 {} n {}", x2, y2, n0, n);

                                        updates[n][n0] = XY{x: x2, y: y2};
                                        n0 += 1;
                                        updated = true;
                                    }
                                }
                            }

                            let d3 = map[y2 as usize][x2 as usize].distance;
                            if d3 >= 0 {
                                //print!("{}", d3 % 10 );
                            } else { 
                                //print!("*");
                            }
                        }
                    }
                }
                // println!("");
            }
        }
        // println!("");
        // println!("");
        println!("length {} {}", d, n0);
        if !updated { break; }
        n = 1 - n;
        n1 = n0;
        n0 = 0;
    }
}
