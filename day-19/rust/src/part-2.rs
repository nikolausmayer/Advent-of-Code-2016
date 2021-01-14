use std::env;
use std::fs;

fn main(){
    let args : Vec<String> = env::args().collect();
    if args.len() < 2 {
        return;
    }
    let input = fs::read_to_string(&args[1]).unwrap()
        .strip_suffix('\n').unwrap()
        .parse::<i32>().unwrap();

    let mut elves = Vec::<i32>::new();
    for i in 1..input+1 {
        elves.push(i);
    }
    let mut i = 0;
    while elves.len() > 1 {
        let opposite = (i + elves.len()/2) % elves.len();
        elves.remove(opposite);
        if opposite > i {
            i += 1;
        }
        i %= elves.len();
    }
    println!("{}", elves[0]);
}

