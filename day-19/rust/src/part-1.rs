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
    while elves.len() > 1 {
        let mut newelves = Vec::new();
        let mut i = 0;
        while i < elves.len() {
            newelves.push(elves[i]);
            i += 2;
        }
        if i == elves.len()+1 {
            newelves.remove(0);
        }
        elves = newelves;
    }
    println!("{}", elves[0]);
}

