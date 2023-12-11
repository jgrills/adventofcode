use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let mut width = 0;
    let mut y = 0;

    let mut map : [[char; 250];250] = [[' ';250];250];

    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(contents) = line {
                if width == 0 {
                    width = contents.len();
                }
                println!("line {} {} {}", y, contents, contents.len());
                for (x, c) in contents.chars().enumerate() {
                    map[y][x] = c;
                }
            }
            y += 1;
        }
    }

    let mut row : [usize; 100] = [0; 100];
    let mut col : [usize; 100] = [0; 100];
    let mut empty_rows = 0;
    let mut empty_cols = 0;
    for y2 in 0..y {
        let mut empty = true;
        for x2 in 0..width {
            print!("{}", map[y2][x2]);
            if empty && map[y2][x2] != '.' { empty = false; }
        }
        if empty {
            row[empty_rows] = y2;
            empty_rows += 1;
        }
        println!(" {}", empty);
    }

    for x2 in 0..width {
        let mut empty = true;
        for y2 in 0..y {
            if empty && map[y2][x2] != '.' { empty = false; }
        }
        if empty {
            col[empty_cols] = x2;
            empty_cols += 1;
        }
        print!("{}", if empty { 't' } else { 'f' });
    }
    println!("");

    println!("empty rows:");
    for i in 0 .. empty_rows {
        print!(" {}", row[i])
    }
    println!("");

    println!("empty col:");
    for i in 0 .. empty_cols {
        print!(" {}", col[i])
    }
    println!("");

    if false {
        // duplicate empty row/cols
        let mut dy = -(empty_rows as i32);
        let mut er = empty_rows;
        width += empty_cols;
        y += empty_rows;
        println!("grown {} {}", y, width);
        for y2 in (0..y).rev() {
            let y3 = (y2 as i32 + dy) as usize;
            let mut dx = -(empty_cols as i32);
            let mut ec = empty_cols;
            for x2 in (0..width).rev() {
                let x3 = (x2 as i32 + dx) as usize;
                //println!("copy {} {} = {} {} | {}", y2, x2, y3, x3, map[y3][x3]);
                map[y2][x2] = map[y3][x3];
                if ec > 0 && col[ec-1] == x3 {
                    dx += 1;
                    ec -= 1;
                }
            }
            if er > 0 && row[er-1] == y3 {
                dy += 1;
                er -= 1;
            }
        }
    }

    // dump
    let mut g = 0;
    let mut gx : [i32; 500] = [0; 500];
    let mut gy : [i32; 500] = [0; 500];
    for y2 in 0..y {
        for x2 in 0..width {
            let ch = map[y2][x2];
            if ch == '#' {
                gx[g] = x2 as i32;
                gy[g] = y2 as i32;
                println!("gal {} {} {} {} {} {}", ch, g, gx[g], gy[g], x2, y2);
                g += 1;
            }
        }
        println!("");
    }


    let mut sum = 0;
    for g2 in 0..g {
        let g2y = gy[g2];
        let g2x = gx[g2];
        for g3 in g2+1..g {
            let g3y = gy[g3];
            let g3x = gx[g3];

            let mut d = (g2y-g3y).abs() + (g2x-g3x).abs();

            let minx = g2x.min(g3x) as usize;
            let maxx = g2x.max(g3x) as usize;
            let miny = g2y.min(g3y) as usize;
            let maxy = g2y.max(g3y) as usize;

            for r in 0..empty_rows {
                if row[r] >= miny && row[r] <= maxy { d += 999999; }
            }
            for c in 0..empty_cols {
                if col[c] >= minx && col[c] <= maxx { d += 999999; }
            }

            println!("gd {} {} delta {}", g2, g3, d);
            sum += d as i64;
        }
    }
    println!("done {}", sum);
}
