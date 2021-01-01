use std::env;
use std::fs;
use regex::{Regex};

fn main(){
    let args : Vec<String> = env::args().collect();
    if args.len() < 2 {
        return;
    }
    let lines = fs::read_to_string(&args[1]).unwrap();

    const H : usize = 6;
    const W : usize = 50;
    let mut state = [[false; W]; H];

    let re_rect = Regex::new(r"^rect (\d+)x(\d+)").unwrap();
    let re_rrow = Regex::new(r"^rotate row y=(\d+) by (\d+)").unwrap();
    let re_rcol = Regex::new(r"^rotate column x=(\d+) by (\d+)").unwrap();

    for line in lines.split('\n') {
        if line == "" {
            continue;
        }

        match re_rect.captures(line) {
            None => {},
            Some(i) => {
                let x : i32 = i.get(1).unwrap().as_str().parse().unwrap();
                let y : i32 = i.get(2).unwrap().as_str().parse().unwrap();
                for py in 0..y as usize {
                    for px in 0..x as usize {
                        state[py][px] = true;
                    }
                }
            }
        }

        match re_rrow.captures(line) {
            None => {},
            Some(i) => {
                let y : usize = i.get(1).unwrap().as_str().parse().unwrap();
                let r : usize = i.get(2).unwrap().as_str().parse().unwrap();
                for _ in 0..r {
                    let tmp = state[y][W-1];
                    for x in (1..W as usize).rev() {
                        state[y][x] = state[y][x-1];
                    }
                    state[y][0] = tmp;
                }
            }
        }

        match re_rcol.captures(line) {
            None => {},
            Some(i) => {
                let x : usize = i.get(1).unwrap().as_str().parse().unwrap();
                let r : usize = i.get(2).unwrap().as_str().parse().unwrap();
                for _ in 0..r {
                    let tmp = state[H-1][x];
                    for y in (1..H as usize).rev() {
                        state[y][x] = state[y-1][x];
                    }
                    state[0][x] = tmp;
                }
            }
        }
    }

    let mut lit_count = 0;
    for y in 0..H as usize {
        for x in 0..W as usize {
            if state[y][x] {
                lit_count += 1;
            }
        }
    }
    println!("{}", lit_count);

    for y in 0..H as usize {
        for x in 0..W as usize {
            match state[y][x] {
                true  => { print!("#") },
                false => { print!(" ") }
            }
        }
        println!();
    }
}

