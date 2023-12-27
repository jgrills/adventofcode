use std::env;
use std::fs;

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
    x: i32,
    m: i32,
    a: i32,
    s: i32
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
    let mut accepted : usize = 0;
    let mut rejected : usize = 0;
    let mut result : u64 = 0;
    'xmas: while !rest.is_empty() {
        let mut line : &str;
        (line, rest) = split_with(rest, '\n');

        line = match line.strip_suffix("}") {
            Some(result) => result,
            None => line
        };
        line = match line.strip_prefix("{") {
            Some(result) => result,
            None => line
        };

        let (xtxt, mastxt) = split_with(line, ',');
        let (mtxt, astxt) = split_with(mastxt, ',');
        let (atxt, stxt) = split_with(astxt, ',');
        if xtxt.chars().nth(0).unwrap() != 'x' { panic!(); }
        if mtxt.chars().nth(0).unwrap() != 'm' { panic!(); }
        if atxt.chars().nth(0).unwrap() != 'a' { panic!(); }
        if stxt.chars().nth(0).unwrap() != 's' { panic!(); }

        let x = xtxt[2..].parse::<i32>().unwrap();
        let m = mtxt[2..].parse::<i32>().unwrap();
        let a = atxt[2..].parse::<i32>().unwrap();
        let s = stxt[2..].parse::<i32>().unwrap();

        println!("{}> {} {} {} {}", accepted+rejected+1, x, m, a, s);
        let mut active = start;
        'apply_rules: while active <= workflows {
            println!("  {}", active);

            if active == accept {
                accepted += 1;
                result += (x + m + a + s) as u64;
                continue 'xmas;
            }
            if active == reject {
                rejected += 1;
                continue 'xmas;
            }

            let wkflw = workflow[active];
            println!("    {}", wkflw.name);

            for ri in 0..wkflw.rules {
                let rl = wkflw.rule[ri];
                if rl.op == 't' {
                    println!("    t {}", rl.tindex);
                    active = rl.tindex;
                    continue 'apply_rules;
                }
                let v = match rl.field {
                    'x' => x,
                    'm' => m,
                    'a' => a,
                    's' => s,
                    _ => panic!("bad field {}", rl.field)
                };
                let compare = match rl.op {
                    '<' => v < rl.value,
                    '>' => v > rl.value,
                    _ => panic!("bad op {}", rl.op)
                };
                if compare {
                    println!("    {}=={} {} {} true {}", rl.field, v, rl.op, rl.value, rl.tindex);
                    active = rl.tindex;
                    continue 'apply_rules;
                }
                else {
                    println!("    {}=={} {} {} false", rl.field, v, rl.op, rl.value);
                }
            }
            panic!("ran out of rules");
        }
    }
    println!("{}=result {}=accepted  {}=rejected", result, accepted, rejected);
}
