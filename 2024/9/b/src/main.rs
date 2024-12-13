use std::fs::read_to_string;

struct Span {
    start : usize, length : usize
}

fn main() {
    let use_example = 0;
    let file_string;
    if use_example != 0 {
        file_string = read_to_string("/home/jgrills/adventofcode/2024/9/example.txt").unwrap();
    } else {
        file_string = read_to_string("/home/jgrills/adventofcode/2024/9/input.txt").unwrap();
    };

    let mut state_file = true;
    let mut file_index : usize = 0;
    let mut file_starts = Vec::new();
    let mut sector_map = Vec::new();
    for b in file_string.bytes() {
        let size = b - b'0';
        let extend_fill : Option<usize> =
            if state_file {
                let extend_index = file_index;
                file_starts.push(Span{start:sector_map.len(), length:size as usize});
                file_index += 1;
                Some(extend_index)
            } else {
                None
            };

        for _ in 0..size { sector_map.push(extend_fill); }
        state_file = !state_file;
    }
    println!("Sector map size {}", sector_map.len());
    println!("file_starts {}", file_starts.len());

    while !file_starts.is_empty() {

        let file = file_starts.pop().unwrap();
        let file_index = file_starts.len();

        'outer: for i in 0..file.start {
            for j in i..i+file.length {
                if sector_map[j] != None { continue 'outer; }
            }

            for e in file.start..file.start+file.length {
                sector_map[e] = None;
            }
            for f in i..i+file.length {
                sector_map[f] = Some(file_index);
            }
            break;
        };
    }

    let mut checksum : usize = 0;
    for i in 0..sector_map.len() {
        let sector = sector_map[i];
        match sector {
            Some(file_index) => {
                checksum += i * file_index;
            }
            None => ()
        }
    }
    println!("checksum {}", checksum);
}