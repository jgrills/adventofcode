use std::fs::File;
use std::io::Write;
use std::fs::read_to_string;
use std::str::FromStr;

fn split_with_char(input: &str, with: char) -> (&str, &str) {
    match input.find(with) {
        Some(index) => (&input[0..index], &input[index+1..]),
        None => (input, "")
    }
}

fn parse_line(line : &str) -> (i32, i32, i32, i32) {
    // p=0,4 v=3,-3
    let (_, line) = split_with_char(line, '=');
    let (a, line) = split_with_char(line, ',');
    let (b, line) = split_with_char(line, ' ');
    let (_, line) = split_with_char(line, '=');
    let (c, d) = split_with_char(line, ',');
    (i32::from_str(a).unwrap(), i32::from_str(b).unwrap(), i32::from_str(c).unwrap(), i32::from_str(d).unwrap())
}

struct Robot { px : i32, py : i32, vx : i32, vy: i32 }

fn main() {
    let use_example = 0;
    let height;
    let width;
    let file_string;
    if use_example != 0 {
        file_string = read_to_string("/home/jgrills/adventofcode/2024/14/example.txt").unwrap();
        height = 7;
        width = 11;
    } else {
        file_string = read_to_string("/home/jgrills/adventofcode/2024/14/input.txt").unwrap();
        height = 103;
        width = 101;
    };

    let mut robots = Vec::new();
    for line in file_string.lines() {
        let (px, py, vx, vy) = parse_line(line);
        robots.push(Robot{px, py, vx, vy});
    }

    let my = (height - 1) / 2;
    let mx = (width - 1) / 2;
    for time in 1..10000 {
        let mut map = [[false; 128]; 128];
        let mut count = [[0;2]; 2];

        for ri in 0..robots.len()  {
            let mut ry = robots[ri].py + (robots[ri].vy * time);
            while ry < 0 { ry += height; };
            while ry >= height { ry -= height; }

            let mut rx = robots[ri].px + (robots[ri].vx * time);
            while rx < 0 { rx += width };
            while rx >= width { rx -= width; }

            if ry < my {
                if rx < mx { count[0][0] += 1; }
                if rx > mx { count[0][1] += 1; }
            } else
            if ry > my {
                if rx < mx { count[1][0] += 1; }
                if rx > mx { count[1][1] += 1; }
            }

            map[ry as usize][rx as usize] = true;
        }

        let path = format!("time_{}", time);
        let mut output = File::create(path).unwrap();
        for y in 0..height {
            for x in 0..width {
                let ch = if map[y as usize][x as usize] { '*' } else { '.' };
                write!(&mut output, "{}",  ch).unwrap();
            }
            writeln!(&mut output).unwrap();
        }
    }
}