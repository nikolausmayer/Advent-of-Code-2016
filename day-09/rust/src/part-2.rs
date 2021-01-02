use std::env;
use std::fs;

fn recurser(text : Vec<char>) -> i64 {
    let mut stage  = 0;
    let mut length = 0;
    let mut repeat = 0;
    let mut len    : i64 = 0;
    let mut subtext : Vec<char> = Vec::new();
    for c in text {
        if stage == 0 {
            if c == '(' {
                stage   = 1;
                length  = 0;
                repeat  = 0;
                subtext = Vec::new();
            } else {
                len += 1;
            }
        } else if stage == 1 {
            if c == 'x' {
                stage = 2;
            } else {
                length = length * 10 + (c as i32 - '0' as i32);
            }
        } else if stage == 2 {
            if c == ')' {
                stage = 3;
            } else {
                repeat = repeat * 10 + (c as i32 - '0' as i32);
            }
        } else {
            length -= 1;
            subtext.push(c);
            if length == 0 {
                stage = 0;
                len += repeat as i64 * recurser(subtext.clone());
            } 
        }
    }
    len
}

fn main(){
    let args : Vec<String> = env::args().collect();
    if args.len() < 2 {
        return;
    }
    let lines = fs::read_to_string(&args[1]).unwrap();

    for line in lines.split('\n') {
        if line == "" {
            continue;
        }

        println!("{}", recurser(line.chars().collect::<Vec<char>>()));
    }
}

