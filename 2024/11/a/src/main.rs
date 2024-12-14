use std::fs::read_to_string;
use std::str::FromStr;

fn main() {
    let use_example = 0;
    let filename;
    if use_example != 0 {
        filename = "/home/jgrills/adventofcode/2024/11/example.txt";
    } else {
        filename = "/home/jgrills/adventofcode/2024/11/input.txt";
    };
    let file_string = read_to_string(filename).unwrap();

    let mut stones : [Vec<usize>; 2] = Default::default();
    stones[0].reserve(256 * 1024);
    stones[1].reserve(256 * 1024);
    for item in file_string.split_whitespace() {
        let i = usize::from_str(item).unwrap();
        stones[0].push(i);
    }

    let mut input_index = 0;
    let mut output_index = 1;
    for i in 0..25 {
        stones[output_index].clear();
        for vi in 0..stones[input_index].len() {
            let v = stones[input_index][vi];
            if v == 0 {
                stones[output_index].push(1);
            } else {
                let mut digits = 0;
                let mut digits_v = v;
                while digits_v != 0{
                    digits += 1;
                    digits_v /= 10;
                }
                if digits & 1 == 0 {
                    let mut high = v;
                    let mut low = 0;
                    let mut low_mult = 1;
                    for _ in 0..digits/2 {
                        let d = high % 10;
                        high /= 10;
                        low = (d * low_mult) + low;
                        low_mult *= 10;
                    }
                    stones[output_index].push(high);
                    stones[output_index].push(low);
                } else {
                    stones[output_index].push(v * 2024);
                }
            }
        }
        println!("after {} {}", i + 1, stones[output_index].len());

        if false {
            for vi in 0..stones[output_index].len() {
                let v = stones[output_index][vi];
                print!(" {}", v);
            }
            println!();
        }

        output_index = 1 - output_index;
        input_index = 1 - input_index;
    }
}
