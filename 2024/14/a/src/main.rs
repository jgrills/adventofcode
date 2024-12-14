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

    let my = (height - 1) / 2;
    let mx = (width - 1) / 2;
    let time = 100;
    let mut count = [[0;2]; 2];
    for line in file_string.lines() {
        let (px, py, vx, vy) = parse_line(line);
        println!("{} {} {} {}", px, py, vx, vy);

        let mut ry = py + (vy * time);
        while ry < 0 { ry += height; };
        while ry >= height { ry -= height; }

        let mut rx = px + (vx * time);
        while rx < 0 { rx += width };
        while rx >= width { rx -= width; }

        println!("final robot yx {} {}", ry, rx);

        if ry < my {
            if rx < mx { count[0][0] += 1; }
            if rx > mx { count[0][1] += 1; }
        } else
        if ry > my {
            if rx < mx { count[1][0] += 1; }
            if rx > mx { count[1][1] += 1; }
        }
    }
    let total = count[0][0] * count[0][1] * count[1][0] * count[1][1];
    println!("quads {} {} {} {} = {}", count[0][0], count[0][1], count[1][0], count[1][1], total);
}