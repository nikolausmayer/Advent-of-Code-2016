use std::env;
use std::fs;
use md5;

fn has_three(s : &String) -> char
{
    let mut pp = '\0';
    let mut p  = '\0';
    for c in s.chars() {
        if c == p && p == pp {
            return c;
        }
        pp = p;
        p  = c;
    }
    return '\0';
}

fn has_five(s : &String) -> Vec<char> 
{
    let mut result = Vec::new();

    let mut pppp = '\0';
    let mut ppp  = '\0';
    let mut pp   = '\0';
    let mut p    = '\0';
    for c in s.chars() {
        if c == p && p == pp && pp == ppp && ppp == pppp {
            result.push(c);
        }
        pppp = ppp;
        ppp  = pp;
        pp   = p;
        p    = c;
    }

    return result;
}

fn main(){
    let args : Vec<String> = env::args().collect();
    if args.len() < 2 {
        return;
    }
    let salt : String = fs::read_to_string(&args[1]).unwrap()
        .split('\n').collect::<Vec<&str>>()
        .get(0).unwrap().to_string();

    let mut otp_keys = Vec::<usize>::new();
    let mut hashes = Vec::<String>::new();

    let mut num : usize = 0;
    loop {
        let teststring = format!("{}{}", salt, num);
        let mut hash = format!("{:x}", md5::compute(&teststring));
        // Part 2
        for _ in 0..2016 {
            hash = format!("{:x}", md5::compute(&hash));
        }
        // <- Part 2
        hashes.push(hash);

        if num >= 1001 {
            let c = has_three(&hashes[num-1001]);
            if c != '\0' {
                for i in (num-1000)..num {
                    if has_five(&hashes[i]).contains(&c) {
                        otp_keys.push(num-1001);
                        //println!("Key #{}: index {}", otp_keys.len(), num-1001);
                        break;
                    }
                }
            }
        }

        if otp_keys.len() == 64 {
            println!("{}", otp_keys[63]);
            break;
        }
        
        num += 1;
    }
}

