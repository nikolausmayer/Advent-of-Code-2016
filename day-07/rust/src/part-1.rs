use std::env;
use std::fs;

fn main(){
    let args : Vec<String> = env::args().collect();
    if args.len() < 2 {
        return;
    }
    let lines = fs::read_to_string(&args[1]).unwrap();

    #[allow(non_snake_case)]
    let mut TLS_IPs = 0;
    for line in lines.split('\n') {
        if line == "" {
            continue;
        }

        let mut ppp = '0';
        let mut pp  = '0';
        let mut p   = '0';
        let mut c1  = false;
        let mut c2  = true;
        let mut hypernetsequence  = false;
        for c in line.chars() {
            if c == '[' {
                hypernetsequence = true;
            } else if c == ']' {
                hypernetsequence = false;
            }
            if ppp != '0' {
                if ppp == c && pp == p && ppp != pp {
                    c1 = true;
                }
                if hypernetsequence && ppp == c && pp == p && ppp != pp {
                    c2 = false;
                }
            }

            ppp = pp;
            pp  = p;
            p   = c;
        }
        if c1 && c2 {
            TLS_IPs += 1;
        }
    }

    println!("{}", TLS_IPs);
}

