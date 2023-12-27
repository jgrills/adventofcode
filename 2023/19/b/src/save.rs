use std::env;
use std::fs;
use std::collections::VecDeque;

#[derive(Copy, Clone)]
struct Rule<'a> {
    field : char,
    op : char,
    value : i32,
    target : &'a str,
    tindex : usize
}

const RULES : usize = 8;
#[derive(Copy, Clone)]
struct Workflow<'a> {
    name: &'a str,
    rules: usize,
    rule: [Rule<'a>; RULES]
}

#[derive(Copy, Clone)]
struct Xmas {
    workflow: usize,
    rule: usize,
    x0: i32,
    x1: i32,
    m0: i32,
    m1: i32,
    a0: i32,
    a1: i32,
    s0: i32,
    s1: i32
}

fn print_xmas(prefix: &str, xmas: &Xmas) {
    println!("{}{}:{} ({},{})=x ({},{})=m ({},{})=a ({},{})=s", prefix, xmas.workflow, xmas.rule, xmas.x0, xmas.x1, xmas.m0, xmas.m1, xmas.a0, xmas.a1, xmas.s0, xmas.s1);
}

fn split_with(input: &str, with: char) -> (&str, &str) {
    match input.find(with) {
        Some(index) => (&input[0..index], &input[index+1..]),
        None => (input, "")
    }
}

fn main() {
    // grab the first command line argument, use it as a filename and load that file into a string
    let file_path : String = env::args().nth(1).unwrap();
    let file_contents : String = match fs::read_to_string(file_path.clone()) {
        Ok(fc) => fc,
        Err(..) => panic!("couldn't read {}", file_path)
    };

    // get str for the rest of the file contents left to process
    let mut rest : &str = file_contents.as_str();

    // import all the rules
    let mut workflows : usize = 0;
    const WORKFLOWS : usize = 600;
    let EmptyRule = Rule{field:' ',op:'f',value:0,target:"",tindex:WORKFLOWS};
    let EmptyWorkflow = Workflow{name:"", rules:0, rule:[EmptyRule; RULES]};
    let mut workflow = [EmptyWorkflow; WORKFLOWS];
    while !rest.is_empty() {
        let mut line : &str;
        (line, rest) = split_with(rest, '\n');

        // An empty line signals the end of workflows
        if line.is_empty() { break; }

        let wkflw : &mut Workflow = &mut workflow[workflows];

        (wkflw.name, line) = split_with(line, '{');
        line = match line.strip_suffix("}") {
            Some(result) => result,
            None => line
        };

        print!("{}% {}", workflows, wkflw.name);
        while !line.is_empty() {
            let rule_text;
            (rule_text, line) = split_with(line, ',');

            let rl : &mut Rule = &mut wkflw.rule[wkflw.rules];
            *rl = match rule_text.find(':') {
                Some(index) => {
                    let field = rule_text.chars().nth(0).unwrap();
                    let op = rule_text.chars().nth(1).unwrap();
                    let value = rule_text[2..index].parse::<i32>().unwrap();
                    let target = &rule_text[index+1..];
                    Rule{field,op,value,target,tindex:WORKFLOWS}
                }
                None => {
                    Rule{field:' ',op:'t',value:0,target:rule_text,tindex:WORKFLOWS}
                }
            };
            print!(" [{} {} {} : {}]", rl.field, rl.op, rl.value, rl.target);
            wkflw.rules += 1;
        }
        println!();

        workflows += 1;
    }

    let accept = workflows;
    let reject = accept+1;

    let mut start : usize = 0;
    for wi in 0..workflows {
        if workflow[wi].name == "in" {
            start = wi;
        }
        for ri in 0..workflow[wi].rules {
            let t = workflow[wi].rule[ri].target;
            if t == "A" {
                workflow[wi].rule[ri].tindex = accept;
            }
            else {
                if t == "R" {
                    workflow[wi].rule[ri].tindex = reject;
                }
                else {
                    for wti in 0..workflows {
                        if t == workflow[wti].name {
                            workflow[wi].rule[ri].tindex = wti;
                            break;
                        }
                    }
                    if workflow[wi].rule[ri].tindex >= workflows { panic!("target {}", t); }
                }
            }
        }
    }

    // process all the items
    let whole : Xmas = Xmas{ workflow: start, rule: 0, x0: 1, x1:4000, m0: 1, m1:4000, a0: 1, a1:4000, s0: 1, s1:4000};
    let mut deque : VecDeque<Xmas> = VecDeque::new();
    deque.push_back(whole);
    let mut result : u64 = 0;
    'xmas: while !deque.is_empty() {
        let mut xmas = match deque.pop_front() {
            Some(x) => x,
            None => panic!("empty queue")
        };

        print_xmas("", &xmas);

        if xmas.workflow == accept {
            let dx : u64 = (xmas.x1 - xmas.x0) as u64 + 1;
            let dm : u64 = (xmas.m1 - xmas.m0) as u64 + 1;
            let da : u64 = (xmas.a1 - xmas.a0) as u64 + 1;
            let ds : u64 = (xmas.s1 - xmas.s0) as u64 + 1;
            let bucket = dx * dm * da * ds;
            println!("  accept {}", bucket);
            result += bucket;
            continue 'xmas;
        }

        if xmas.workflow == reject {
            continue 'xmas;
        }

        let wkflw = workflow[xmas.workflow];
        let rl = wkflw.rule[xmas.rule];
        if rl.op == 't' {
            println!("    t {}", rl.tindex);
        }

        match rl.op {
            't' => {
                xmas.workflow = rl.tindex;
                xmas.rule = 0;
                print_xmas("  t ", &xmas);
                deque.push_back(xmas);
                continue 'xmas;
            }
            '<' => {
                match rl.field {
                    'x' => {
                        if xmas.x1 < rl.value {
                            xmas.workflow = rl.tindex;
                            xmas.rule = 0;
                            print_xmas("  << ", &xmas);
                            deque.push_back(xmas);
                            continue 'xmas;
                        }
                        else {
                            if xmas.x0 >= rl.value {
                                xmas.rule += 1;
                                print_xmas("  >> ", &xmas);
                                deque.push_back(xmas);
                                continue 'xmas;
                            }
                            else {
                                let x0 = xmas.x0;

                                xmas.x0 = rl.value;
                                xmas.rule += 1;
                                print_xmas("  > ", &xmas);
                                deque.push_back(xmas);

                                xmas.x0 = x0;
                                xmas.x1 = rl.value-1;
                                xmas.workflow = rl.tindex;
                                xmas.rule = 0;
                                print_xmas("  < ", &xmas);
                                deque.push_back(xmas);

                                continue 'xmas;
                            }
                        }
                    },
                    'm' => {
                        if xmas.m1 < rl.value {
                            xmas.workflow = rl.tindex;
                            xmas.rule = 0;
                            print_xmas("  << ", &xmas);
                            deque.push_back(xmas);
                            continue 'xmas;
                        }
                        else {
                            if xmas.m0 >= rl.value {
                                xmas.rule += 1;
                                print_xmas("  >> ", &xmas);
                                deque.push_back(xmas);
                                continue 'xmas;
                            }
                            else {
                                let x0 = xmas.m0;

                                xmas.m0 = rl.value;
                                xmas.rule += 1;
                                print_xmas("  > ", &xmas);
                                deque.push_back(xmas);

                                xmas.m0 = x0;
                                xmas.m1 = rl.value-1;
                                xmas.workflow = rl.tindex;
                                xmas.rule = 0;
                                print_xmas("  < ", &xmas);
                                deque.push_back(xmas);

                                continue 'xmas;
                            }
                        }
                    },
                    'a' => {
                        if xmas.a1 < rl.value {
                            xmas.workflow = rl.tindex;
                            xmas.rule = 0;
                            print_xmas("  << ", &xmas);
                            deque.push_back(xmas);
                            continue 'xmas;
                        }
                        else {
                            if xmas.a0 >= rl.value {
                                xmas.rule += 1;
                                print_xmas("  >> ", &xmas);
                                deque.push_back(xmas);
                                continue 'xmas;
                            }
                            else {
                                let x0 = xmas.a0;

                                xmas.a0 = rl.value;
                                xmas.rule += 1;
                                print_xmas("  > ", &xmas);
                                deque.push_back(xmas);

                                xmas.a0 = x0;
                                xmas.a1 = rl.value-1;
                                xmas.workflow = rl.tindex;
                                xmas.rule = 0;
                                print_xmas("  < ", &xmas);
                                deque.push_back(xmas);

                                continue 'xmas;
                            }
                        }
                    }
                    's' => {
                        if xmas.s1 < rl.value {
                            xmas.workflow = rl.tindex;
                            xmas.rule = 0;
                            print_xmas("  << ", &xmas);
                            deque.push_back(xmas);
                            continue 'xmas;
                        }
                        else {
                            if xmas.s0 >= rl.value {
                                xmas.rule += 1;
                                print_xmas("  >> ", &xmas);
                                deque.push_back(xmas);
                                continue 'xmas;
                            }
                            else {
                                let x0 = xmas.s0;

                                xmas.s0 = rl.value;
                                xmas.rule += 1;
                                print_xmas("  > ", &xmas);
                                deque.push_back(xmas);

                                xmas.s0 = x0;
                                xmas.s1 = rl.value-1;
                                xmas.workflow = rl.tindex;
                                xmas.rule = 0;
                                print_xmas("  < ", &xmas);
                                deque.push_back(xmas);

                                continue 'xmas;
                            }
                        }
                    },
                    _ => panic!("bad field {}", rl.field)
                }
            },
            '>' => {
                match rl.field {
                    'x' => {
                        if xmas.x0 > rl.value {
                            xmas.workflow = rl.tindex;
                            xmas.rule = 0;
                            print_xmas("  << ", &xmas);
                            deque.push_back(xmas);
                            continue 'xmas;
                        }
                        else {
                            if xmas.x0 >= rl.value {
                                xmas.rule += 1;
                                print_xmas("  >> ", &xmas);
                                deque.push_back(xmas);
                                continue 'xmas;
                            }
                            else {
                                let x0 = xmas.x0;

                                xmas.x0 = rl.value;
                                xmas.rule += 1;
                                print_xmas("  > ", &xmas);
                                deque.push_back(xmas);

                                xmas.x0 = x0;
                                xmas.x1 = rl.value-1;
                                xmas.workflow = rl.tindex;
                                xmas.rule = 0;
                                print_xmas("  < ", &xmas);
                                deque.push_back(xmas);

                                continue 'xmas;
                            }
                        }
                    },
                    'm' => {
                        if xmas.m1 < rl.value {
                            xmas.workflow = rl.tindex;
                            xmas.rule = 0;
                            print_xmas("  << ", &xmas);
                            deque.push_back(xmas);
                            continue 'xmas;
                        }
                        else {
                            if xmas.m0 >= rl.value {
                                xmas.rule += 1;
                                print_xmas("  >> ", &xmas);
                                deque.push_back(xmas);
                                continue 'xmas;
                            }
                            else {
                                let x0 = xmas.m0;

                                xmas.m0 = rl.value;
                                xmas.rule += 1;
                                print_xmas("  > ", &xmas);
                                deque.push_back(xmas);

                                xmas.m0 = x0;
                                xmas.m1 = rl.value-1;
                                xmas.workflow = rl.tindex;
                                xmas.rule = 0;
                                print_xmas("  < ", &xmas);
                                deque.push_back(xmas);

                                continue 'xmas;
                            }
                        }
                    },
                    'a' => {
                        if xmas.a1 < rl.value {
                            xmas.workflow = rl.tindex;
                            xmas.rule = 0;
                            print_xmas("  << ", &xmas);
                            deque.push_back(xmas);
                            continue 'xmas;
                        }
                        else {
                            if xmas.a0 >= rl.value {
                                xmas.rule += 1;
                                print_xmas("  >> ", &xmas);
                                deque.push_back(xmas);
                                continue 'xmas;
                            }
                            else {
                                let x0 = xmas.a0;

                                xmas.a0 = rl.value;
                                xmas.rule += 1;
                                print_xmas("  > ", &xmas);
                                deque.push_back(xmas);

                                xmas.a0 = x0;
                                xmas.a1 = rl.value-1;
                                xmas.workflow = rl.tindex;
                                xmas.rule = 0;
                                print_xmas("  < ", &xmas);
                                deque.push_back(xmas);

                                continue 'xmas;
                            }
                        }
                    }
                    's' => {
                        if xmas.s1 < rl.value {
                            xmas.workflow = rl.tindex;
                            xmas.rule = 0;
                            print_xmas("  << ", &xmas);
                            deque.push_back(xmas);
                            continue 'xmas;
                        }
                        else {
                            if xmas.s0 >= rl.value {
                                xmas.rule += 1;
                                print_xmas("  >> ", &xmas);
                                deque.push_back(xmas);
                                continue 'xmas;
                            }
                            else {
                                let x0 = xmas.s0;

                                xmas.s0 = rl.value;
                                xmas.rule += 1;
                                print_xmas("  > ", &xmas);
                                deque.push_back(xmas);

                                xmas.s0 = x0;
                                xmas.s1 = rl.value-1;
                                xmas.workflow = rl.tindex;
                                xmas.rule = 0;
                                print_xmas("  < ", &xmas);
                                deque.push_back(xmas);

                                continue 'xmas;
                            }
                        }
                    },
                    _ => panic!("bad field {}", rl.field)
                }
            },
            _ => panic!("bad op {}", rl.op)
        };
    }

    println!("{}=result", result);
}
