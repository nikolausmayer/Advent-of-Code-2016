use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return;
    }

    let mut visited : Vec<(i32,i32)> = Vec::new();
    visited.push((0, 0));

    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut dir : i32 = 0;
    'outer: for s in fs::read_to_string(&args[1]).unwrap().trim().split(", ") {
        let turn = s.chars().nth(0).unwrap();
        match turn {
            'R' => dir = (dir + 1) % 4,
            'L' => dir = (dir - 1 + 4) % 4,
            _ => {}
        }
        let walk : i32 = (&s[1..]).parse().unwrap();

        for _ in 0..walk {
            match dir {
                0 => y -= 1,
                1 => x += 1,
                2 => y += 1,
                3 => x -= 1,
                _ => {}
            }

            if visited.contains(&(x, y)) {
                println!("{}", x.abs() + y.abs());
                break 'outer;
            } else {
                visited.push((x, y));
            }
        }
    }
}

