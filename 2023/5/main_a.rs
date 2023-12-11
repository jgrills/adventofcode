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

fn find_next(slice: &[i32]) -> i32 {
    let sz = slice.len();
    println!("  find_next {}", sz);
    for q in slice {
        print!(" {}", q);
    }
    println!("");
    let mut arr : [i32; 100] = [0; 100];

    let mut again = false;
    for i in 0 .. sz-1 {
        arr[i] = slice[i+1] - slice[i];
        if arr[i] != 0 { again = true; }
    }
    if !again { return 0; }

    let down = &arr[0..sz-1];
    let next = find_next(down);
    let result = down.last().unwrap() + next;
    println!("  result {}", result);

    return result; 

}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let mut result : i64 = 0;
    let mut arr : [i32; 100] = [0; 100];
    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(contents) = line {
                let mut count = 0;
                for (x, c) in contents.split(" ").enumerate() {
                    let v = c.parse::<i32>().unwrap();
                    arr[x] = v;
                    count = x + 1;
                }

                let next = find_next(&arr[0..count]);
                let answer = next+arr[count-1];
                println!("final {} {}", next, answer);
                result += answer as i64;
            }
        }
    }
    println!("result {}", result);
}
