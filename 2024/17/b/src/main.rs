use std::fs::read_to_string;
use std::str::FromStr;
use std::thread;

fn clean_input(input: &str) -> &str {
    match input.find(':') {
        Some(index) => &input[index+2..],
        None => ""
    }
}

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
        filename = "/home/jgrills/adventofcode/2024/17/example_b.txt";
    } else {
        filename = "/home/jgrills/adventofcode/2024/17/input.txt";
    };
    let file_string = read_to_string(filename).unwrap();
    let mut lines = file_string.lines();

    // Register A: 729
    // Register B: 0
    // Register C: 0
    //
    // Program: 0,1,5,4,3,0
    lines.next();
    lines.next();
    lines.next();
    lines.next();
    let mut p = clean_input(lines.next().unwrap());
    println!("program {}", p);
    let mut program = [0_u8; 32];
    let mut program_len = 0;
    while !p.is_empty() {
        let a;
        let b;
        (a,p) = split_with_char(p, ',');
        (b,p) = split_with_char(p, ',');
        program[program_len] = u8::from_str(a).unwrap();
        program_len += 1;
        program[program_len] = u8::from_str(b).unwrap();
        program_len += 1;
    }

    for i in 0..program_len {
        print!(" {}", program[i]);
    }
    println!();

    let mut done = false;
    let mut children = Vec::new();
    for thread_index in 0..8 {
        children.push(thread::spawn(move || {
            let mut i = thread_index;
            'outer: loop {
                if done { break; }
                let mut a = i;
                let base_a = a;
                i += 8;
                let mut b = 0;
                let mut c = 0;
                let mut ip = 0;
                let mut out = 0;

                while ip < program_len {
                    let ins = program[ip];
                    let arg = program[ip+1];
                    let mut next_ip = ip + 2;
                    let (lit, combo)  = match arg {
                            0 => (0, 0),
                            1 => (1, 1),
                            2 => (2, 2),
                            3 => (3, 3),
                            4 => (4, a),
                            5 => (5, b),
                            6 => (6, c),
                            7 => (7, 0),
                            _ => { assert!(false, "bad arg {}", arg as char); (0,0) }
                        };

                    match ins {
                        // The adv instruction (opcode 0) performs division. The numerator is the value in the A register. The denominator is found by raising 2 to the power of the instruction's combo operand. (So, an operand of 2 would divide A by 4 (2^2); an operand of 5 would divide A by 2^B.) The result of the division operation is truncated to an integer and then written to the A register.
                        0 => a = a / (2_u32.pow(combo as u32) as usize),

                        // The bxl instruction (opcode 1) calculates the bitwise XOR of register B and the instruction's literal operand, then stores the result in register B.
                        1 => b = b ^ lit,

                        // The bst instruction (opcode 2) calculates the value of its combo operand modulo 8 (thereby keeping only its lowest 3 bits), then writes that value to the B register.
                        2 => b = combo % 8,

                        // The jnz instruction (opcode 3) does nothing if the A register is 0. However, if the A register is not zero, it jumps by setting the instruction pointer to the value of its literal operand; if this instruction jumps, the instruction pointer is not increased by 2 after this instruction.
                        3 => if a != 0 { next_ip = lit; },

                        // The bxc instruction (opcode 4) calculates the bitwise XOR of register B and register C, then stores the result in register B. (For legacy reasons, this instruction reads an operand but ignores it.)
                        4 => b = b ^ c,

                        // The out instruction (opcode 5) calculates the value of its combo operand modulo 8, then outputs that value. (If a program outputs multiple values, they are separated by commas.)
                        5 => {
                            if out == program_len { /* println!("too long"); */ continue 'outer; }
                            if program[out] != (combo % 8) as u8 {  /* println!("bad digit {} {}", out, combo % 7); */ continue 'outer; }
                            out += 1;
                        },

                        // The bdv instruction (opcode 6) works exactly like the adv instruction except that the result is stored in the B register. (The numerator is still read from the A register.)
                        6 => b = a / (2_u32.pow(combo as u32) as usize),

                        // The cdv instruction (opcode 7) works exactly like the adv instruction except that the result is stored in the C register. (The numerator is still read from the A register.)
                        7 => c = a / (2_u32.pow(combo as u32) as usize),

                        _ => assert!(false, "bad instruction")
                    };

                    ip = next_ip;
                }
                if out == program_len {
                    println!("found {}", base_a);
                    done = true;
                } else {
                    /* println!("too short"); */
                }
        }}));
    }

    for child in children {
        let _ = child.join();
    }
}
