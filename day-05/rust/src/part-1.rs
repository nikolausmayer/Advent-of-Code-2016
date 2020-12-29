use std::env;
use std::fs;
use md5;

fn main(){
    let args : Vec<String> = env::args().collect();
    if args.len() < 2 {
        return;
    }
    let door_id = fs::read_to_string(&args[1]).unwrap().trim().to_string();

    let mut result : Vec<char> = Vec::new();

    let mut num = 0;
    loop {
        let teststring = format!("{}{}", door_id, num);
        let hexhash = format!("{:x}", md5::compute(&teststring));
        if hexhash.starts_with("00000") {
            result.push(hexhash.chars().nth(5).unwrap());
            if result.len() == 8 {
                break;
            }
        }
        num += 1;
    }

    for c in &result {
        print!("{}", c);
    }
    println!();
}

