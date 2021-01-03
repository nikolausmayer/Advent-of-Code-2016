use std::env;
use std::fs;
use regex::Regex;
use std::collections::{HashMap, LinkedList};

struct Robot {
    low_to : i32,
    high_to : i32,
    values : Vec<i32>,
}


fn main(){
    let args : Vec<String> = env::args().collect();
    if args.len() < 2 {
        return;
    }
    let lines = fs::read_to_string(&args[1]).unwrap();

    let re_in = Regex::new(r"^value (\d+) goes to bot (\d+)").unwrap();
    let re_out = Regex::new(r"^bot (\d+) gives low to (\w+) (\d+) and high to (\w+) (\d+)").unwrap();

    let mut robots  : HashMap<i32, Robot> = HashMap::new();
    let mut outputs : HashMap<i32, Vec<i32>> = HashMap::new();

    let mut queue : LinkedList<i32> = LinkedList::new();

    for line in lines.split('\n') {
        if line == "" {
            continue;
        }

        match re_in.captures(line) {
            None => {},
            Some(i) => {
                let v : i32 = i.get(1).unwrap().as_str().parse().unwrap();
                let r : i32 = i.get(2).unwrap().as_str().parse().unwrap();

                robots.entry(r).or_insert(Robot{low_to: 0, high_to: 0, values: Vec::new()});
                robots.get_mut(&r).unwrap().values.push(v);
                //println!("{} to {}", v, r);
                if robots.get_mut(&r).unwrap().values.len() == 2 {
                    //println!("{} ready", r);
                    queue.push_back(r);
                }
            }
        }
    }

    for line in lines.split('\n') {
        if line == "" {
            continue;
        }

        match re_out.captures(line) {
            None => {},
            Some(i) => {
                let ri : i32 = i.get(1).unwrap().as_str().parse().unwrap();
                let lo = i.get(2).unwrap().as_str();
                let lv : i32 = i.get(3).unwrap().as_str().parse().unwrap(); 
                let ho = i.get(4).unwrap().as_str();
                let hv : i32 = i.get(5).unwrap().as_str().parse().unwrap(); 

                robots.entry(ri).or_insert(Robot{low_to: 0, high_to: 0, values: Vec::new()});

                robots.get_mut(&ri).unwrap().low_to  = if lo == "bot" {lv} else {-lv-1};
                robots.get_mut(&ri).unwrap().high_to = if ho == "bot" {hv} else {-hv-1};

                //println!("{} => {} {}, {} {}", ri, lo, lv, ho, hv);
            }
        }
    }

    while queue.len() > 0 {
        let next = queue.pop_front().unwrap();
        //println!("{} process", next);
        
        let l = *robots[&next].values.iter().min().unwrap();
        let h = *robots[&next].values.iter().max().unwrap();
        let lo = robots[&next].low_to;
        let ho = robots[&next].high_to;

        // Part 1
        if l == 17 && h == 61 {
            println!("{}", next);
        }

        if lo >= 0 {
            robots.get_mut(&lo).unwrap().values.push(l);
            if robots[&lo].values.len() == 2 {
                queue.push_back(lo);
                //println!("{} ready", lo);
            }
        } else {
            outputs.entry(-lo-1).or_insert(Vec::new());
            outputs.get_mut(&(-lo-1)).unwrap().push(l);
        }
        if ho >= 0 {
            robots.get_mut(&ho).unwrap().values.push(h);
            if robots[&ho].values.len() == 2 {
                queue.push_back(ho);
                //println!("{} ready", ho);
            }
        } else {
            outputs.entry(-ho-1).or_insert(Vec::new());
            outputs.get_mut(&(-ho+1)).unwrap().push(h);
        }
    }

    // Part 2
    println!("{}", outputs[&0][0] * outputs[&1][0] * outputs[&2][0]);
}

