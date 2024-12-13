use std::fs::read_to_string;

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
    let mut sector_map = Vec::new();
    for b in file_string.bytes() {
        let size = b - b'0';
        print!("{}", size);
        let extend_fill : Option<usize> =
            if state_file {
                let extend_index = file_index;
                file_index += 1;
                Some(extend_index)
            } else {
                None
            };

        for _ in 0..size { sector_map.extend([extend_fill; 1]); }
        state_file = !state_file;
    }
    println!();
    println!("Sector map size {}", sector_map.len());

    let mut left = 0;
    let mut right = sector_map.len() - 1;
    loop {
        while sector_map[left] != None { left += 1; }
        while sector_map[right] == None { right -= 1; }
        if left < right {
            let temp = sector_map[left];
            sector_map[left] = sector_map[right];
            sector_map[right] = temp;
        } else {
            println!("exiting {} {}", left, right);
            break;
        }
    }
    let mut checksum : usize = 0;
    for i in 0..left {
        let sector = sector_map[i];
        match sector {
            Some(file_index) => {
                checksum += i * file_index;
                println!("ck {} * {} = {}", i, file_index, checksum);
            }
            None => assert!(false, "everything should be valid"),
        }
    }

}