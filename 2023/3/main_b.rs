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
    let zero = '0' as i32;

    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines(file_path) {

        struct NumberData {
            number: i32,
            start: i32,
            end: i32
        }

        struct LineData {
            numbers: Vec<NumberData>,
            symbols: Vec<i32>
        }

        let mut all_lines : Vec<LineData> = Vec::new();

        let empty_line : LineData = LineData { numbers: Vec::new(), symbols: Vec::new()};
        all_lines.push(empty_line);


        for line in lines {
            if let Ok(mut contents) = line {
                let mut number : i32 = 0;
                let mut number_start : i32 = 0;
                let mut processing_number = false;

                let mut current_line : LineData = LineData { numbers: Vec::new(), symbols: Vec::new()};

                contents.push('.');
                for (i, c) in contents.chars().enumerate() {
                    if c.is_ascii_digit() {
                        if !processing_number {
                            processing_number = true;
                            number_start = (i as i32) - 1;
                            number = 0;
                        }
                        number = (number * 10) + (c as i32 - zero);
                    } else {
                        if processing_number {
                            let number_data = NumberData {
                                number,
                                start: number_start,
                                end: (i as i32)
                            };
                            current_line.numbers.push(number_data);
                            number_start = -1;
                            number = 0;
                            processing_number = false;
                        }
                        if c == '*' {
                            current_line.symbols.push(i as i32);
                        }
                    }
                }
                all_lines.push(current_line);
            }
        }

        let empty_line : LineData = LineData { numbers: Vec::new(), symbols: Vec::new()};
        all_lines.push(empty_line);

        let mut sum : i64 = 0;

        let mut gear_numbers : Vec<i32> = Vec::new();
        
        for line_num in 1 .. all_lines.len()-1 {
            let local = &all_lines[line_num-1 .. line_num+2];
            for sym in &local[1].symbols {

                gear_numbers.clear();
                for window_line in local {
                    for nd in &window_line.numbers {
                        if *sym >= nd.start && *sym <= nd.end {
                            gear_numbers.push(nd.number);
                        }
                    }
                }

                if gear_numbers.len() == 2 {
                    sum += (gear_numbers[0] * gear_numbers[1]) as i64;
                }
            }
        }
        println!("final sum {}", sum);
    }
}
