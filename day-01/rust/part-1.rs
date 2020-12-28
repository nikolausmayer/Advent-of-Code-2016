use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return;
    }

    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut dir : i32 = 0;
    for s in fs::read_to_string(&args[1]).unwrap().trim().split(", ") {
        let turn = s.chars().nth(0).unwrap();
        match turn {
            'R' => dir = (dir + 1) % 4,
            'L' => dir = (dir - 1 + 4) % 4,
            _ => {}
        }
        let walk : i32 = (&s[1..]).parse().unwrap();
        match dir {
            0 => y -= walk,
            1 => x += walk,
            2 => y += walk,
            3 => x -= walk,
            _ => {}
        }
    }
    println!("{}", x.abs() + y.abs());
}

