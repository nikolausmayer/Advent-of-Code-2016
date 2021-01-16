use std::env;
use std::fs;
use regex;

fn main(){
    let args : Vec<String> = env::args().collect();
    if args.len() < 2 {
        return;
    }
    let input = fs::read_to_string(&args[1]).unwrap();

    let re_swappos  = regex::Regex::new(r"swap position (\d+) with position (\d+)").unwrap();
    let re_swaplet  = regex::Regex::new(r"swap letter (\w+) with letter (\w+)").unwrap();
    let re_rotleft  = regex::Regex::new(r"rotate left (\d+) step").unwrap();
    let re_rotright = regex::Regex::new(r"rotate right (\d+) step").unwrap();
    let re_rotbased = regex::Regex::new(r"rotate based on position of letter (\w+)").unwrap();
    let re_reverse  = regex::Regex::new(r"reverse positions (\d+) through (\d+)").unwrap();
    let re_move     = regex::Regex::new(r"move position (\d+) to position (\d+)").unwrap();

    let mut password = "fbgdceah".chars().collect::<Vec<char>>();

    for line in input.split('\n').rev() {
        if line == "" {
            continue;
        }

        match re_swappos.captures(line) {
            None => {},
            Some(i) => {
                let p0 : usize = i.get(1).unwrap().as_str().parse().unwrap();
                let p1 : usize = i.get(2).unwrap().as_str().parse().unwrap();

                let tmp = password[p0];
                password[p0] = password[p1];
                password[p1] = tmp;
            }
        }

        match re_swaplet.captures(line) {
            None => {},
            Some(i) => {
                let l0 : char = i.get(1).unwrap().as_str().parse().unwrap();
                let l1 : char = i.get(2).unwrap().as_str().parse().unwrap();
                let p0 = password.iter().position(|&x| x == l0).unwrap();
                let p1 = password.iter().position(|&x| x == l1).unwrap();

                let tmp = password[p0];
                password[p0] = password[p1];
                password[p1] = tmp;
            }
        }

        match re_rotright.captures(line) {
            None => {},
            Some(i) => {
                let steps : usize = i.get(1).unwrap().as_str().parse().unwrap();

                let mut tmp = Vec::new();
                for i in steps..password.len() {
                    tmp.push(password[i]);
                }
                for i in 0..steps {
                    tmp.push(password[i]);
                }
                password = tmp;
            }
        }

        match re_rotleft.captures(line) {
            None => {},
            Some(i) => {
                let steps : usize = i.get(1).unwrap().as_str().parse().unwrap();

                let mut tmp = Vec::new();
                for i in password.len()-steps..password.len() {
                    tmp.push(password[i]);
                }
                for i in 0..password.len()-steps {
                    tmp.push(password[i]);
                }
                password = tmp;
            }
        }

        match re_rotbased.captures(line) {
            None => {},
            Some(i) => {
                let l : char = i.get(1).unwrap().as_str().parse().unwrap();
                let pos : usize = password.iter().position(|&x| x == l).unwrap();
                let mut steps = match pos {
                    0 => 1,
                    1 => 1,
                    2 => 6,
                    3 => 2,
                    4 => 7,
                    5 => 3,
                    6 => 0,
                    7 => 4,
                    _ => 0
                };

                let mut tmp = Vec::new();
                for i in steps..password.len() {
                    tmp.push(password[i]);
                }
                for i in 0..steps {
                    tmp.push(password[i]);
                }
                password = tmp;
            }
        }

        match re_reverse.captures(line) {
            None => {},
            Some(i) => {
                let p0 : usize = i.get(1).unwrap().as_str().parse().unwrap();
                let p1 : usize = i.get(2).unwrap().as_str().parse().unwrap();

                let mut tmp = Vec::new();
                for i in 0..p0 {
                    tmp.push(password[i]);
                }
                for i in (p0..p1+1).rev() {
                    tmp.push(password[i]);
                }
                for i in p1+1..password.len() {
                    tmp.push(password[i]);
                }
                password = tmp;
            }
        }

        match re_move.captures(line) {
            None => {},
            Some(i) => {
                let p0 : usize = i.get(1).unwrap().as_str().parse().unwrap();
                let p1 : usize = i.get(2).unwrap().as_str().parse().unwrap();

                let mut tmp = Vec::new();
                for i in 0..password.len() {
                    if i == p1 {
                        continue;
                    }
                    if p0 < p1 {
                        if i == p0 {
                            tmp.push(password[p1]);
                        }
                        tmp.push(password[i]);
                    } else {
                        tmp.push(password[i]);
                        if i == p0 {
                            tmp.push(password[p1]);
                        }
                    }
                }
                password = tmp;
            }
        }
    }
    for l in &password { print!("{}", l); } println!();
}

