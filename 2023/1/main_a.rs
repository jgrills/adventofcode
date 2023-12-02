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

    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines("./input") {
        // Consumes the iterator, returns an (Optional) String
        let mut sum = 0;
        for line in lines {
            if let Ok(mut contents) = line {
                contents.retain(|c| c >= '0' && c <= '9');
                let first = contents.chars().nth(0).unwrap();
                let last = contents.chars().last().unwrap();
                let zero = '0' as i32;
                let value = ((first as i32 - zero) * 10) + (last as i32- zero);
                sum += value;
                println!("{}{} {} {}", first, last, value, contents);
            }
        }
        println!("sun {}", sum);
    }

}
