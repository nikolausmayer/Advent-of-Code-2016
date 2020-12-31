use std::env;
use std::fs;

fn main(){
    let args : Vec<String> = env::args().collect();
    if args.len() < 2 {
        return;
    }
    let lines = fs::read_to_string(&args[1]).unwrap();

    #[allow(non_snake_case)]
    let mut SSL_IPs = 0;
    for line in lines.split('\n') {
        if line == "" {
            continue;
        }

        let mut pp = '0';
        let mut p  = '0';
        let mut abas : Vec<String> = Vec::new();
        let mut hypernetsequence  = false;
        let mut ok = false;
        for c in line.chars() {
            if c == '[' {
                hypernetsequence = true;
            } else if c == ']' {
                hypernetsequence = false;
            }
            if pp != '0' {
                if !hypernetsequence && pp == c && pp != p {
                    abas.push(vec![p, c, p].into_iter().collect::<String>());
                }
            }
            pp  = p;
            p   = c;
        }
        for c in line.chars() {
            if c == '[' {
                hypernetsequence = true;
            } else if c == ']' {
                hypernetsequence = false;
            }
            if pp != '0' {
                if hypernetsequence && pp == c && pp != p {
                    if abas.iter().any(|x| x.clone() == vec![pp, p, c].into_iter().collect::<String>()) {
                        ok = true;
                    }
                }
            }
            pp  = p;
            p   = c;
        }

        if ok {
            SSL_IPs += 1;
        }
    }

    println!("{}", SSL_IPs);
}

