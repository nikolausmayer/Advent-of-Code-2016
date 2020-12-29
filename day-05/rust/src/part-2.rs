use std::env;
use std::fs;
use md5;

fn main(){
    let args : Vec<String> = env::args().collect();
    if args.len() < 2 {
        return;
    }
    let door_id = fs::read_to_string(&args[1]).unwrap().trim().to_string();

    let mut result : Vec<char> = "________".chars().collect();

    let mut num = 0;
    loop {
        let teststring = format!("{}{}", door_id, num);
        let hexhash = format!("{:x}", md5::compute(&teststring));
        if hexhash.starts_with("00000") {
            let pos = (hexhash.chars().nth(5).unwrap() as usize) - ('0' as usize);
            if pos < 8 && result[pos] == '_' {
                let val = hexhash.chars().nth(6).unwrap() as char;
                result[pos] = val;
            }
            if result.iter().filter(|&x| x == &'_').count() == 0 {
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

