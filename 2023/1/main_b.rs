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

fn start_digit(text : &str) -> i32 {
    if text.starts_with("0") { return 0; }
    if text.starts_with("one") || text.starts_with("1") { return 1; }
    if text.starts_with("two") || text.starts_with("2") { return 2; }
    if text.starts_with("three") || text.starts_with("3") { return 3; }
    if text.starts_with("four") || text.starts_with("4") { return 4; }
    if text.starts_with("five") || text.starts_with("5") { return 5; }
    if text.starts_with("six") || text.starts_with("6") { return 6; }
    if text.starts_with("seven") || text.starts_with("7") { return 7; }
    if text.starts_with("eight") || text.starts_with("8") { return 8; }
    if text.starts_with("nine") || text.starts_with("9") { return 9; }
    return -1;
}

fn end_digit(text : &str) -> i32 {
    if text.ends_with("0") { return 0; }
    if text.ends_with("one") || text.ends_with("1") { return 1; }
    if text.ends_with("two") || text.ends_with("2") { return 2; }
    if text.ends_with("three") || text.ends_with("3") { return 3; }
    if text.ends_with("four") || text.ends_with("4") { return 4; }
    if text.ends_with("five") || text.ends_with("5") { return 5; }
    if text.ends_with("six") || text.ends_with("6") { return 6; }
    if text.ends_with("seven") || text.ends_with("7") { return 7; }
    if text.ends_with("eight") || text.ends_with("8") { return 8; }
    if text.ends_with("nine") || text.ends_with("9") { return 9; }
    return -1;
}

fn main() {

    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines("./input") {
        // Consumes the iterator, returns an (Optional) String
        let mut sum = 0;
        for line in lines {
            if let Ok(contents) = line {
                let mut tens: i32 = -1;
                let mut first = contents.as_str();
                while tens == -1 {
                    tens = start_digit(first);
                    first = &first[1..];
                }
                let mut ones: i32 = -1;
                let mut last = contents.as_str();
                while ones == -1 {
                    ones = end_digit(last);
                    last = &last[..last.len()-1];
                }

                //contents = contents.replace("two", "2");
                //contents = contents.replace("three", "3");
                //contents = contents.replace("four", "4");
                //contents = contents.replace("five", "5");
                //contents = contents.replace("six", "6");
                //contents = contents.replace("seven", "7");
                //contents = contents.replace("eight", "8");
                //contents = contents.replace("nine", "9");
                let value = tens * 10 + ones;
                sum += value;
                println!("{}{} {} {}", tens, ones, value, contents);
            }
        }
        println!("sun {}", sum);
    }

}
