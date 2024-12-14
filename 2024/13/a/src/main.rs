use std::fs::read_to_string;
use std::str::FromStr;

fn split_with_char(input: &str, with: char) -> (&str, &str) {
    match input.find(with) {
        Some(index) => (&input[0..index], &input[index+1..]),
        None => (input, "")
    }
}

fn main() {
    let use_example = 0;
    let filename;
    if use_example != 0 {
        filename = "/home/jgrills/adventofcode/2024/13/example.txt";
    } else {
        filename = "/home/jgrills/adventofcode/2024/13/input.txt";
    };
    let file_string = read_to_string(filename).unwrap();

    let lines : Vec<&str> = file_string.lines().collect();
    let mut line_index = 0;
    let mut total = 0;
    while line_index < lines.len() {
        // Button A: X+92, Y+24
        // Button B: X+13, Y+94
        // Prize: X=8901, Y=8574

        let line = lines[line_index];
        let (_, line) = split_with_char(line, '+');
        let (axt, line) = split_with_char(line, ',');
        let (_, ayt) = split_with_char(line, '+');

        let line = lines[line_index+1];
        let (_, line) = split_with_char(line, '+');
        let (bxt, line) = split_with_char(line, ',');
        let (_, byt) = split_with_char(line, '+');

        let line = lines[line_index+2];
        let (_, line) = split_with_char(line, '=');
        let (pxt, line) = split_with_char(line, ',');
        let (_, pyt) = split_with_char(line, '=');

        let ax = usize::from_str(axt).unwrap();
        let ay = usize::from_str(ayt).unwrap();
        let bx = usize::from_str(bxt).unwrap();
        let by = usize::from_str(byt).unwrap();
        let px = usize::from_str(pxt).unwrap();
        let py = usize::from_str(pyt).unwrap();

        println!("a {} {} b {} {} p {} {}", ax, ay, bx, by, px, py);
        line_index += 4;

        let mut cheapest = None;
        for a in 0..100 {
            let aax = a * ax;
            if aax > px { continue; }
            let rx = px - aax;
            let b = rx / bx;
            if b * bx != rx { continue; }
            let aby = a * ay + b * by;
            if aby != py { continue; }
            println!("  solution {} {}", a, b);
            let cost = 3 * a + b;

            cheapest = match cheapest {
                Some(previous) =>
                    if cost < previous {
                        Some(cost)
                    } else {
                        Some(previous)
                    }
                None => Some(cost),
            }
        }
        match cheapest {
            Some(value) => { total += value; println!("cheapest {} {}", value, total); }
            None => (),
        }
    }
}
