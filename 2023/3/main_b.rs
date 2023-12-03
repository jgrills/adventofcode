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

// Represents a number in the input, including its span of characters (-1 and +1 too to handle adjacency)
#[derive(Copy, Clone)]
struct NumberData {
    number: i32,
    start: i32,
    end: i32
}

impl NumberData {
    const CHAR_ZERO_DIGIT : i32 = '0' as i32;
    const NOT_STARTED : i32 = -1;

    fn has_number(&self) -> bool {
        return self.number != NumberData::NOT_STARTED;
    }

    fn add_digit(&mut self, digit : char, index : i32) {
        if self.number == NumberData::NOT_STARTED {
            self.number = 0;
            self.start = index - 1;
        }

        self.number = (self.number * 10) + (digit as i32 - NumberData::CHAR_ZERO_DIGIT);
        self.end = index + 1;
    }

    fn reset(&mut self) {
        *self = NumberData { ..Default::default() };
    }
}

// Provide defaults for NumberData
impl Default for NumberData {
    fn default() -> NumberData {
        NumberData {
            number: NumberData::NOT_STARTED,
            start: 0,
            end: 0
        }
    }
}

// Represents a usable line of input from the data
// Lists all the numbers and symbols/gears that were found on the line
#[derive(Clone)]
struct LineData {
    numbers: Vec<NumberData>,
    symbols: Vec<i32>
}

// Provide defaults
impl Default for LineData {
    fn default() -> LineData {
        LineData {
            numbers: Vec::new(),
            symbols: Vec::new(),
        }
    }
}


fn main() {

    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines(file_path) {

        // Vector for processed lines of input
        let mut all_lines : Vec<LineData> = Vec::new();

        // Prime the data with a blank line to make things easy
        let empty_line : LineData = LineData { ..Default::default() };
        all_lines.push(empty_line);

        let mut current_number : NumberData = NumberData { ..Default::default() };

        // Process all input lines
        for line in lines {
            if let Ok(contents) = line {
                let mut current_line : LineData = LineData { ..Default::default() };

                // Add a trailing dot so I don't have to special case numbers at the end of the line
                for (i, c) in contents.chars().enumerate() {

                    // Search the string for numeric characters and manually convert them to be able to easily track string lengths
                    if c.is_ascii_digit() {
                        current_number.add_digit(c, i as i32);
                    } else {
                        // Push and reset the current number if we have one
                        if current_number.has_number() {
                            current_line.numbers.push(current_number);
                            current_number.reset();
                        }

                        // Handle gears
                        if c == '*' {
                            current_line.symbols.push(i as i32);
                        }
                    }
                }

                // Handle numbers at the end of the line
                if current_number.has_number() {
                    current_line.numbers.push(current_number);
                    current_number.reset();
                }



                // store the processed input line
                all_lines.push(current_line);
            }
        }

        // Push a blank line at the end to give us a window of 3 lines for each input line
        let empty_line : LineData = LineData {..Default::default()};
        all_lines.push(empty_line);

        // Get some member
        let mut number_of_gears : i32;
        let mut gear_numbers : [i32; 2] = [0, 0];

        // Process all the input lines looking for gears
        let mut sum : i64 = 0;
        for line_num in 1 .. all_lines.len()-1 {
            let local = &all_lines[line_num-1 .. line_num+2];

            // for each gear '*' in the input data
            for sym in &local[1].symbols {

                // count the number of numbers adjacent to this gear
                number_of_gears = 0;
                for window_line in local {
                    for nd in &window_line.numbers {
                        if *sym >= nd.start && *sym <= nd.end {
                            if number_of_gears < 2 {
                                gear_numbers[number_of_gears as usize] = nd.number;
                            }
                            number_of_gears += 1;
                        }
                    }
                }

                // If it's two, then it's a gear.  Combine it into the sum.
                if number_of_gears == 2 {
                    sum += (gear_numbers[0] * gear_numbers[1]) as i64;
                }
            }
        }

        // Report the answer and we are done!
        println!("answer {}", sum);
    }
}
