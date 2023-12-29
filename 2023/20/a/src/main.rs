use std::env;
use std::fs;
use std::collections::VecDeque;

#[derive(Copy, Clone)]
struct Signal {
    from: usize,
    node: usize,
    low: bool
}

const TARGETS : usize = 8;
const SOURCES : usize = 10;
#[derive(Copy, Clone)]
struct Node<'a> {
    name: &'a str,
    kind: char,
    target_text: &'a str,
    target: [usize; TARGETS],
    targets: usize,
    on: bool,
    sources: usize,
    source: [usize; SOURCES],
    last_low: [bool; SOURCES]
}

fn split_with_char(input: &str, with: char) -> (&str, &str) {
    match input.find(with) {
        Some(index) => (&input[0..index], &input[index+1..]),
        None => (input, "")
    }
}

fn split_with_str<'a, 'b>(input: &'a str, with: &'b str) -> (&'a str, &'a str) {
    match input.find(with) {
        Some(index) => (&input[0..index], &input[index+with.len()..]),
        None => (input, "")
    }
}

fn low_text(low: bool) -> &'static str {
    match low {
        true => "low",
        false => "high"
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
    const NODES : usize = 60;
    let empty_node = Node{name:"", kind:' ', target_text:"", target:[TARGETS; TARGETS], targets:0, on:false, sources:0, source:[0;SOURCES], last_low:[true;SOURCES] };
    let mut node = [empty_node; NODES];
    let mut names : [&str; NODES] = [""; NODES];
    let mut number_of_nodes : usize = 0;
    while !rest.is_empty() {
        let line : &str;
        (line, rest) = split_with_char(rest, '\n');

        let (source, dest) = split_with_str(line, " -> ");

        let mut name = source;
        let mut kind = ' ';
        if source.starts_with('%') {
            kind = '%';
            name = &source[1..];
        }
        if source.starts_with('&') {
            kind = '&';
            name = &source[1..];
        }

        node[number_of_nodes] = Node{name, kind, target_text:dest, target:[TARGETS; TARGETS], targets:0, on:false, sources:0, source:[0;SOURCES], last_low:[true;SOURCES] };
        names[number_of_nodes] = name;
        number_of_nodes += 1;
    }

    node[number_of_nodes] = Node{name:"output", kind:'O', target_text:"", target:[TARGETS; TARGETS], targets:0, on:false, sources:0, source:[0;SOURCES], last_low:[true;SOURCES] };
    names[number_of_nodes] = "output";
    number_of_nodes += 1;

    for ni in 0..number_of_nodes {
        let mut rest = node[ni].target_text;
        while !rest.is_empty() {
            let dest;
            (dest, rest) = split_with_str(rest, ", ");
            'g: {
                for di in 0..number_of_nodes {
                    if node[di].name == dest {
                        node[ni].target[node[ni].targets] = di;
                        node[ni].targets += 1;
                        break 'g;
                    }
                }
                // Route unknown inputs to output
                node[ni].target[node[ni].targets] = number_of_nodes-1;
                node[ni].targets += 1;
                println!("couldn't find {}", dest);
            }
        }
    }

    let start = 'find_start: {
        for ni in 0..number_of_nodes {
            let n = &mut node[ni];
            if n.name == "broadcaster" {
                n.kind = 'S';
                break 'find_start ni;
            }
        }
        panic!("no broadcaster");
    };
    println!("start: {}", start);

    for ni in 0..number_of_nodes {
        if node[ni].kind == '&' {
            for si in 0..number_of_nodes {
                let targets = node[si].targets;
                for ti in 0..targets {
                    if node[si].target[ti] == ni {
                        node[ni].source[node[ni].sources] = si;
                        node[ni].sources += 1;
                        println!("Adding source {} -> {}", node[si].name, node[ni].name);
                    }
                }
            }

            if node[ni].sources == 0 { panic!("no inputs"); }
            if node[ni].sources == 1 {
                node[ni].kind = '!';
            }
        }
    }

    for ni in 0..number_of_nodes {
        let &n = &node[ni];
        print!("{} {} : {} :", n.name, n.kind, n.target_text);
        for ti in 0..n.targets {
            print!(" {}", n.target[ti]);
        }
        println!();
    }

    let mut low_sent : u64 = 0;
    let mut hi_sent : u64 = 0;

    let mut deque: VecDeque<Signal> = VecDeque::new();
    for _pushes in 0..1000 {

        println!("button -low -> broadcaster");
        deque.push_back(Signal{from:start, node: start, low: true});
        low_sent += 1;

        while !deque.is_empty() {
            let input = match deque.pop_front() {
                Some(s) => s,
                None => panic!("deque empty")
            };

            let from = input.from;
            let low = input.low;
            let ni = input.node;

            let result = match node[ni].kind {
                'O' => None,
                'S' => Some(low),
                '%' =>
                    if low {
                        node[ni].on = !node[ni].on;
                        Some(!node[ni].on)
                    } else {
                        None
                    },
                '&' => {
                        'check: {
                            for si in 0..node[ni].sources {
                                if from == node[ni].source[si] {
                                    node[ni].last_low[si] = low;
                                    break 'check;
                                }
                            }
                            panic!("source not found {} -> {}", node[from].name, node[ni].name);
                        }

                        let mut high = true;
                        for si in 0..node[ni].sources {
                            if node[ni].last_low[si] {
                                high = false;
                                break;
                            }
                        }
                        Some(high)
                    },
                '!' => Some(!low),
                _ => panic!("failed to match kind {}", node[ni].kind)
            };

            match result {
                Some(send_low) => {

                    for ti in 0..node[ni].targets {
                        deque.push_back(Signal{from:ni, node: node[ni].target[ti], low: send_low});
                        println!("send {} {} -{}-> {}", node[ni].kind, node[ni].name, low_text(send_low), node[node[ni].target[ti]].name);
                    }

                    if send_low {
                        low_sent += node[ni].targets as u64;
                    }
                    else {
                        hi_sent += node[ni].targets as u64;
                    }
                },
                None => println!("process {}=node {} {} {} X", input.node, low_text(input.low), node[ni].name, node[ni].kind)
            };
        }
        println!("partial {} low {} result {}\n", hi_sent, low_sent, hi_sent * low_sent);
    }
    println!("final high {} low {} result {}", hi_sent, low_sent, hi_sent * low_sent);
    println!("final@1000 high {} low {} result {}", hi_sent*250, low_sent*250, hi_sent * low_sent * 250 * 250);
}
