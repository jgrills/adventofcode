use std::fmt;
use std::fs::read_to_string;

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
    Close,

    D,
    O,
    DoOpen,
    DoClose,
    N,
    Tick,
    T,
    DontOpen,
    DontClose,
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
            ParserState::D =>write!(f, "d"),
            ParserState::O =>write!(f, "o"),
            ParserState::DoOpen =>write!(f, "do()"),
            ParserState::DoClose =>write!(f, "do)"),
            ParserState::N =>write!(f, "n"),
            ParserState::Tick =>write!(f, "'"),
            ParserState::T =>write!(f, "T"),
            ParserState::DontOpen =>write!(f, "dont("),
            ParserState::DontClose =>write!(f, "dont)"),
        }
    }
}

fn main() {
    let use_example = 0;
    let filename =
        if use_example != 0 {
            "/home/jgrills/adventofcode/2024/3/exampleb.txt"
        } else {
            "/home/jgrills/adventofcode/2024/3/input.txt"
        };
    let file_string = read_to_string(filename).unwrap();

    let mut parser_state = ParserState::Init;
    let mut num0 : i64 = 0;
    let mut num1 : i64 = 0;
    let mut enabled = true;
    let mut total : i64 = 0;
    let mut count : i64 = 0;
    for ch in file_string.chars() {
        match parser_state {
            ParserState::Init => {
                match ch {
                    'd' => parser_state = ParserState::D,
                    'm' => parser_state = ParserState::M,
                    _ =>  parser_state = ParserState::Init,
                };
            },
            ParserState::M =>
                match ch {
                    'u' => parser_state = ParserState::U,
                    'd' => parser_state = ParserState::D,
                    'm' => parser_state = ParserState::M,
                    _ =>  parser_state = ParserState::Init,
                },
            ParserState::U =>
                match ch {
                    'l' => parser_state = ParserState::L,
                    'd' => parser_state = ParserState::D,
                    'm' => parser_state = ParserState::M,
                    _ =>  parser_state = ParserState::Init,
                },
            ParserState::L =>
                match ch {
                    '(' => parser_state = ParserState::Open,
                    'd' => parser_state = ParserState::D,
                    'm' => parser_state = ParserState::M,
                    _ =>  parser_state = ParserState::Init,
                },
            ParserState::Open => {
                match ch {
                    '0' ..= '9' => {
                        num0 = (ch as i64) - ('0' as i64);
                        parser_state = ParserState::Num00;
                    },
                    'd' => parser_state = ParserState::D,
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
                    'd' => parser_state = ParserState::D,
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
                    'd' => parser_state = ParserState::D,
                    'm' => parser_state = ParserState::M,
                    _ =>  parser_state = ParserState::Init,
                },
            ParserState::Num02 =>
                match ch {
                    ',' => parser_state = ParserState::Comma,
                    'd' => parser_state = ParserState::D,
                    'm' => parser_state = ParserState::M,
                    _ =>  parser_state = ParserState::Init,
                },
            ParserState::Comma =>
                match ch {
                    '0' ..= '9' => {
                        num1 = (ch as i64) - ('0' as i64);
                        parser_state = ParserState::Num10;
                    },
                    'd' => parser_state = ParserState::D,
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
                    'd' => parser_state = ParserState::D,
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
                    'd' => parser_state = ParserState::D,
                    'm' => parser_state = ParserState::M,
                    _ =>  parser_state = ParserState::Init,
                },
            ParserState::Num12 =>
                match ch {
                    ')' => parser_state = ParserState::Close,
                    'd' => parser_state = ParserState::D,
                    'm' => parser_state = ParserState::M,
                    _ =>  parser_state = ParserState::Init,
                },

            ParserState::D =>
                match ch {
                    'o' => parser_state = ParserState::O,
                    'd' => parser_state = ParserState::D,
                    'm' => parser_state = ParserState::M,
                    _ =>  parser_state = ParserState::Init,
                },

            ParserState::O =>
                match ch {
                    '(' => parser_state = ParserState::DoOpen,
                    'n' => parser_state = ParserState::N,
                    'd' => parser_state = ParserState::D,
                    'm' => parser_state = ParserState::M,
                    _ =>  parser_state = ParserState::Init,
                },

            ParserState::DoOpen =>
                match ch {
                    ')' => parser_state = ParserState::DoClose,
                    'd' => parser_state = ParserState::D,
                    'm' => parser_state = ParserState::M,
                    _ =>  parser_state = ParserState::Init,
                },

            ParserState::N =>
                match ch {
                    '\'' => parser_state = ParserState::Tick,
                    'd' => parser_state = ParserState::D,
                    'm' => parser_state = ParserState::M,
                    _ =>  parser_state = ParserState::Init,
                },

            ParserState::Tick =>
                match ch {
                    't' => parser_state = ParserState::T,
                    'd' => parser_state = ParserState::D,
                    'm' => parser_state = ParserState::M,
                    _ =>  parser_state = ParserState::Init,
                },

            ParserState::T =>
                match ch {
                    '(' => parser_state = ParserState::DontOpen,
                    'd' => parser_state = ParserState::D,
                    'm' => parser_state = ParserState::M,
                    _ =>  parser_state = ParserState::Init,
                },

            ParserState::DontOpen =>
                match ch {
                    ')' => parser_state = ParserState::DontClose,
                    'd' => parser_state = ParserState::D,
                    'm' => parser_state = ParserState::M,
                    _ =>  parser_state = ParserState::Init,
                },

                ParserState::Close | ParserState::DoClose | ParserState::DontClose => assert!(false, "Should never enter state machine at close"),
        }


        if parser_state == ParserState::DoClose {
            enabled = true;
            println!("do()");
            parser_state = ParserState::Init;
        } else if parser_state == ParserState::DontClose {
            enabled = false;
            println!("don't()");
            parser_state = ParserState::Init;
        } else if parser_state == ParserState::Close {
            let result = num0 * num1;

            if enabled {
                total += result;
                count += 1;
            }
            println!("mul({},{})={} => {} + {} @ {}", num0, num1, result, enabled, count, total);
            parser_state = ParserState::Init;
        }
    }
}
