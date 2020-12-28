use std::cmp;
use std::env;
use std::fs;

fn main() 
{
    let args : Vec<String> = env::args().collect();
    if args.len() > 2 {
        return;
    }

    let mut x : i32 = 2;
    let mut y : i32 = 2;
    for line in fs::read_to_string(&args[1]).unwrap().split('\n') {
        if line == "" {
            continue;
        }
        for instruction in line.trim().chars() {
            match instruction {
                'U' => { y = cmp::max(1, y-1); },
                'R' => { x = cmp::min(3, x+1); },
                'D' => { y = cmp::min(3, y+1); },
                'L' => { x = cmp::max(1, x-1); },
                _ => {}
            }
        }
        print!("{}", (y - 1) * 3 + x);
    }
    println!("");
}

