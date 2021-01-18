use std::env;
use std::fs;
use regex;
use std::collections::BTreeMap;

fn main(){
    let args : Vec<String> = env::args().collect();
    if args.len() < 2 {
        return;
    }
    let input = fs::read_to_string(&args[1]).unwrap();

    let re = regex::Regex::new(r"/dev/grid/node-x(\d+)-y(\d+) +(\d+)T +(\d+)T +(\d+)T").unwrap();

    let mut map = BTreeMap::<(i8, i8), (i32, i32)>::new();

    for line in input.split('\n') {
        match re.captures(line) {
            None => {},
            Some(i) => {
                let x : i8 = i.get(1).unwrap().as_str().parse().unwrap();
                let y : i8 = i.get(2).unwrap().as_str().parse().unwrap();
                let s : i32 = i.get(3).unwrap().as_str().parse().unwrap();
                let a : i32 = i.get(4).unwrap().as_str().parse().unwrap();
                map.insert((x, y), (s, a));
            }
        }
    }

    let mut viable_pairs = 0;
    for a in &map {
        if a.1.1 == 0 {
            continue;
        }
        for b in &map {
            if a == b {
                continue;
            }
            if a.1.1 <= (b.1.0 - b.1.1) {
                viable_pairs += 1;
            }
        }
    }
    println!("{}", viable_pairs);
}

