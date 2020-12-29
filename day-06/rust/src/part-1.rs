use std::env;
use std::fs;

fn main(){
    let args : Vec<String> = env::args().collect();
    if args.len() < 2 {
        return;
    }
    let lines = fs::read_to_string(&args[1]).unwrap();

    let linelength = lines.split('\n').collect::<Vec<&str>>()[0].len();

    let mut map : Vec<Vec<i32>> = Vec::new();
    for _ in 0..linelength {
        map.push(Vec::<i32>::new());
        for _ in 0..26 {
            map.last_mut().unwrap().push(0);
        }
    }

    for line in lines.split('\n') {
        if line == "" {
            continue;
        }

        for i in 0..linelength {
            let c = line.chars().nth(i).unwrap();
            map[i][(c as usize) - ('a' as usize)] += 1;
        }
    }

    for submap in map {
        let maxcount = submap.iter().max().unwrap();
        for i in 0..26 {
            if submap[i] == *maxcount {
                print!("{}", (i + 'a' as usize) as u8 as char);
                break;
            }
        }
    }
    println!();
}

