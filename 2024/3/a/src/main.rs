use std::fmt;
use std::fs::read_to_string;
// use std::str::FromStr;

#[derive(PartialEq, Clone, Copy)]
enum ParserState {
    Init,
    M,
    U,
    L,
    Open,
    Num00,
    Num01,
    Num02,
    Comma,
    Num10,
    Num11,
    Num12,
    Close
}

impl fmt::Display for ParserState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match self {
        ParserState::Init =>write!(f, "init"),
        ParserState::M =>write!(f, "m"),
        ParserState::U =>write!(f, "u"),
        ParserState::L =>write!(f, "l"),
        ParserState::Open =>write!(f, "("),
        ParserState::Num00 =>write!(f, "#01"),
        ParserState::Num01 =>write!(f, "#01"),
        ParserState::Num02 =>write!(f, "#03"),
        ParserState::Comma =>write!(f, ","),
        ParserState::Num10 =>write!(f, "#11"),
        ParserState::Num11 =>write!(f, "#12"),
        ParserState::Num12 =>write!(f, "#13"),
        ParserState::Close =>write!(f, ")"),
       }
    }
}

fn main() {
    let use_example = 0;
    let filename =
        if use_example != 0 {
            "/home/jgrills/adventofcode/2024/3/examplea.txt"
        } else {
            "/home/jgrills/adventofcode/2024/3/input.txt"
        };
    let file_string = read_to_string(filename).unwrap();

    let mut parser_state = ParserState::Init;
    let mut num0 : i64 = 0;
    let mut num1 : i64 = 0;
    let mut total : i64 = 0;
    let mut count : i64 = 0;
    for ch in file_string.chars() {
        match parser_state {
            ParserState::Init => {
                match ch {
                    'm' => parser_state = ParserState::M,
                    _ =>  parser_state = ParserState::Init,
                };
            },
            ParserState::M =>
                match ch {
                    'u' => parser_state = ParserState::U,
                    'm' => parser_state = ParserState::M,
                    _ =>  parser_state = ParserState::Init,
                },
            ParserState::U =>
                match ch {
                    'l' => parser_state = ParserState::L,
                    'm' => parser_state = ParserState::M,
                    _ =>  parser_state = ParserState::Init,
                },
            ParserState::L =>
                match ch {
                    '(' => parser_state = ParserState::Open,
                    'm' => parser_state = ParserState::M,
                    _ =>  parser_state = ParserState::Init,
                },
            ParserState::Open => {
                match ch {
                    '0' ..= '9' => {
                        num0 = (ch as i64) - ('0' as i64);
                        parser_state = ParserState::Num00;
                    },
                    'm' => parser_state = ParserState::M,
                    _ =>  parser_state = ParserState::Init,
                }
            },
            ParserState::Num00 =>
                match ch {
                    '0' ..= '9' => {
                        num0 = (num0 * 10) + (ch as i64) - ('0' as i64);
                        parser_state = ParserState::Num01;
                    },
                    ',' => parser_state = ParserState::Comma,
                    'm' => parser_state = ParserState::M,
                    _ =>  parser_state = ParserState::Init,
                },
            ParserState::Num01 =>
                match ch {
                    '0' ..= '9' => {
                        num0 = (num0 * 10) + (ch as i64) - ('0' as i64);
                        parser_state = ParserState::Num02;
                    },
                    ',' => parser_state = ParserState::Comma,
                    'm' => parser_state = ParserState::M,
                    _ =>  parser_state = ParserState::Init,
                },
            ParserState::Num02 =>
                match ch {
                    ',' => parser_state = ParserState::Comma,
                    'm' => parser_state = ParserState::M,
                    _ =>  parser_state = ParserState::Init,
                },
            ParserState::Comma =>
                match ch {
                    '0' ..= '9' => {
                        num1 = (ch as i64) - ('0' as i64);
                        parser_state = ParserState::Num10;
                    },
                    'm' => parser_state = ParserState::M,
                    _ =>  parser_state = ParserState::Init,
                },
            ParserState::Num10 =>
                match ch {
                    '0' ..= '9' => {
                        num1 = (num1 * 10) + (ch as i64) - ('0' as i64);
                        parser_state = ParserState::Num11;
                    },
                    ')' => parser_state = ParserState::Close,
                    'm' => parser_state = ParserState::M,
                    _ =>  parser_state = ParserState::Init,
                },
            ParserState::Num11 =>
                match ch {
                    '0' ..= '9' => {
                        num1 = (num1 * 10) + (ch as i64) - ('0' as i64);
                        parser_state = ParserState::Num12;
                    },
                    ')' => parser_state = ParserState::Close,
                    'm' => parser_state = ParserState::M,
                    _ =>  parser_state = ParserState::Init,
                },
            ParserState::Num12 =>
                match ch {
                    ')' => parser_state = ParserState::Close,
                    'm' => parser_state = ParserState::M,
                    _ =>  parser_state = ParserState::Init,
                },
            ParserState::Close => assert!(false, "Should never enter state machine at close"),
        }

        if parser_state == ParserState::Close {
            let result = num0 * num1;
            total += result;
            count += 1;
            println!("mul({},{})={} => {} @ {}", num0, num1, result, count, total);
            parser_state = ParserState::Init;
        }
    }
}
