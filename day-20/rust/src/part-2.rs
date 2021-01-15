use std::env;
use std::fs;

fn main(){
    let args : Vec<String> = env::args().collect();
    if args.len() < 2 {
        return;
    }
    let input = fs::read_to_string(&args[1]).unwrap();

    let mut pairs = Vec::new();
    for line in input.split('\n') {
        if line == "" {
            continue;
        }
        let nums = line.split('-').collect::<Vec<&str>>();
        let a = nums[0].parse::<u32>().unwrap();
        let b = nums[1].parse::<u32>().unwrap();
        pairs.push((a, b));
    }
    pairs.sort_by(|a, b| a.0.cmp(&b.0));

    let mut allowed : u32 = 0;
    let mut highest : u32 = 0;
    for pair in pairs {
        if highest < std::u32::MAX && pair.0 > highest+1 {
            let n : u32 = pair.0 - (highest+1);
            allowed += n;
        }
        highest = std::cmp::max(pair.1, highest);
    }
    if highest < std::u32::MAX {
        allowed += std::u32::MAX - highest;
    }
    println!("{}", allowed);
}

