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

    let mut connections :  HashMap::<char, Connection> = HashMap::new();
    connections.insert('|', Connection{data:[XY{x:  0, y: -1}, XY{x:  0, y:  1}]});
    connections.insert('-', Connection{data:[XY{x: -1, y:  0}, XY{x:  1, y:  0}]});
    connections.insert('L', Connection{data:[XY{x:  1, y:  0}, XY{x:  0, y: -1}]});
    connections.insert('J', Connection{data:[XY{x: -1, y:  0}, XY{x:  0, y: -1}]});
    connections.insert('7', Connection{data:[XY{x: -1, y:  0}, XY{x:  0, y:  1}]});
    connections.insert('F', Connection{data:[XY{x:  1, y:  0}, XY{x:  0, y:  1}]});
    connections.insert('.', Connection{data:[XY{x:  0, y:  0}, XY{x:  0, y:  0}]});

    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let mut start_x = -1;
    let mut start_y = -1;

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
                        start_x = x as i32 + 1;
                        start_y = y;
                        map[y as usize][x+1 as usize].distance = 0;
                    }
                }
            }
            y += 1;
        }
    }

    let mut start_up = false;
    let mut start_down = false;
    let mut start_left = false;
    let mut start_right = false;
    for y2 in start_y-1 .. start_y+2 {
        for x2 in start_x-1 .. start_x+2 {
            println!("  x2 {} y2 {} t2 {}", x2, y2, map[y2 as usize][x2 as usize].text);
            let c = connections.get(&map[y2 as usize][x2 as usize].text);
            match c {
                None    => {},
                Some(cxy) => {
                    for ci in 0 .. 2 {
                        let cx = cxy.data[ci].x;
                        let cy = cxy.data[ci].y;
                        let x3 = x2 + cx;
                        let y3 = y2 + cy;
                        if start_x == x3 && start_y == y3 {
                            if -cx < 0 { start_left = true; }
                            if -cx > 0 { start_right = true; }
                            if -cy < 0 { start_up = true; }
                            if -cy > 0 { start_down = true; }
                            println!("  start {} {} {} {} {} {}", -cx, -cy, start_up, start_down, start_left, start_right);
                        }

                    }
                }
            }
        }
    }

    println!("start {} {} {} {}", start_up, start_down, start_left, start_right);

    let start_char;
    if       start_up &&  start_down && !start_left && !start_right { start_char = '|'; }
    else if !start_up && !start_down &&  start_left &&  start_right { start_char = '-'; }
    else if  start_up && !start_down && !start_left &&  start_right { start_char = 'L'; }
    else if  start_up && !start_down &&  start_left && !start_right { start_char = 'J'; }
    else if !start_up &&  start_down &&  start_left && !start_right { start_char = '7'; }
    else if !start_up &&  start_down && !start_left &&  start_right { start_char = 'F'; }
    else { panic!(""); }
    map[start_y as usize][start_x as usize].text = start_char;
    println!("S replaced {}", start_char);

    let mut updates: [[XY; 20000]; 2] = [[XY{..Default::default() };20000]; 2];
    let mut previous_update_count = 0;
    let mut previous_update = 0;
    let mut active_update_count = 0;
    let mut active_update = 1;

    updates[0][0].x = start_x;
    updates[0][0].y = start_y;
    updates[0][1].x = start_x;
    updates[0][1].y = start_y;
    if start_up { updates[0][previous_update_count].y -= 1; previous_update_count+=1;  }
    if start_down { updates[0][previous_update_count].y += 1; previous_update_count+=1;  }
    if start_left { updates[0][previous_update_count].x -= 1; previous_update_count+=1;  }
    if start_right { updates[0][previous_update_count].x += 1; previous_update_count+=1;  }
    if previous_update_count != 2 { panic!(""); }

    println!("first two {} {} {} {}", updates[0][0].x, updates[0][0].y, updates[0][1].x, updates[0][1].y);


    println!("fill");

    let mut breadth = 1;
    while previous_update_count > 0 {
        for u in 0..previous_update_count {

            let up = updates[previous_update][u];
            map[up.y as usize][up.x as usize].distance = breadth;
            let cell = map[up.y as usize][up.x as usize];
            let next_char = cell.text;
            println!("pipe {} {} {} {}", up.x, up.y, next_char, cell.distance);
            let c = connections.get(&next_char);
            match c {
                None => {},
                Some(cxy) => {
                    for ci in 0 .. 2 {
                        let cx = cxy.data[ci].x;
                        let cy = cxy.data[ci].y;
                        let x3 = up.x + cx;
                        let y3 = up.y + cy;
                        println!("  pipe {} {} {} {}", x3, y3, map[y3 as usize][x3 as usize].text, map[y3 as usize][x3 as usize].distance);
                        if map[y3 as usize][x3 as usize].distance < 0 {
                            updates[active_update][active_update_count].x = x3;
                            updates[active_update][active_update_count].y = y3;
                            active_update_count += 1;
                            map[y3 as usize][x3 as usize].distance = breadth;
                        }
                    }
                }
            }
        }

        // advance to second list
        previous_update_count = active_update_count;
        previous_update = active_update;
        active_update = 1 - active_update;
        active_update_count = 0;
        breadth += 1;
    }

    println!("pipe max {}", breadth-1);
    let mut count = 0;
    for y4 in 1..y {
        let mut inside = false;
        let mut start = ' ';
        for x4 in 1..width-1 {
            let spot = map[y4 as usize][x4 as usize].text;
            if map[y4 as usize][x4 as usize].distance >= 0  {
                if spot == '|' {
                    inside = !inside;
                }
                else if spot == 'F' || spot == 'L' 
                {
                    start = spot;
                }
                else if spot == '7' || spot == 'J'
                {
                    if (start == 'L' && spot == '7') || (start == 'F' && spot == 'J') {
                        inside = !inside;
                    }
                }
                print!("+");
            }
            else {
                if inside {
                    count += 1;
                    print!("*");
                }
                else
                {
                    print!(".");
                }
            }
        }
        println!("");
    }
    println!("inside {}", count);
}
