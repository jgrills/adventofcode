use std::collections::BTreeMap;
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

    let mut stones_0 : BTreeMap<usize, usize> = BTreeMap::new();
    let mut stones_1 : BTreeMap<usize, usize> = BTreeMap::new();
    for item in file_string.split_whitespace() {
        let i = usize::from_str(item).unwrap();
        stones_0.entry(i).and_modify(|i| *i += 1).or_insert(1);
    }

    for i in 0..75 {
        let (input,output) =
            if i & 1 == 0 {
                (&stones_0, & mut stones_1)
            } else {
                (&stones_1, & mut stones_0)
            };
        output.clear();

        for (vref,countref) in input.into_iter() {
            let v = *vref;
            let count = *countref;
            if v == 0 {
                output.entry(1).and_modify(|i| *i += count).or_insert(count);
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
                    output.entry(low).and_modify(|i| *i += count).or_insert(count);
                    output.entry(high).and_modify(|i| *i += count).or_insert(count);
                } else {
                    output.entry(v * 2024).and_modify(|i| *i += count).or_insert(count);
                }
            }
        }

        let mut total = 0;
        for (vref,countref) in output.into_iter() {
            let v = *vref;
            let count = *countref;
            total += count;
            // print!(" {}(*{})", v, count);
        }
        // println!();

        println!("{} total {}", i+1, total);
    }
}
