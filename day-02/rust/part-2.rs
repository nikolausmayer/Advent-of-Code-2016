use std::env;
use std::fs;

fn elem(x : i32, y : i32) -> char
{
    let map = [['0', '0', '1', '0', '0'],
               ['0', '2', '3', '4', '0'],
               ['5', '6', '7', '8', '9'],
               ['0', 'A', 'B', 'C', '0'],
               ['0', '0', 'D', '0', '0']];
    map[y as usize][x as usize]
}

fn main() 
{
    let args : Vec<String> = env::args().collect();
    if args.len() > 2 {
        return;
    }

    let mut x : i32 = 0;
    let mut y : i32 = 2;
    for line in fs::read_to_string(&args[1]).unwrap().split('\n') {
        if line == "" {
            continue;
        }
        for instruction in line.trim().chars() {
            match instruction {
                'U' => if y > 0 && elem(x, y-1) != '0' { y -= 1 },
                'R' => if x < 4 && elem(x+1, y) != '0' { x += 1 },
                'D' => if y < 4 && elem(x, y+1) != '0' { y += 1 },
                'L' => if x > 0 && elem(x-1, y) != '0' { x -= 1 },
                _ => {}
            }
        }
        print!("{}", elem(x, y));
    }
    println!("");
}

