use std::env;
use std::fs;

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

        let mut stage  = 0;
        let mut length = 0;
        let mut repeat = 0;
        let mut len    = 0;
        for c in line.chars() {
            if stage == 0 {
                if c == '(' {
                    stage = 1;
                    length = 0;
                    repeat = 0;
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
                    len += length * repeat;
                } else {
                    repeat = repeat * 10 + (c as i32 - '0' as i32);
                }
            } else {
                length -= 1;
                if length == 0 {
                    stage = 0;
                } 
            }
        }
        println!("{}", len);
    }
}

