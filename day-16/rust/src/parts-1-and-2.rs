use std::env;
use std::fs;

fn main(){
    let args : Vec<String> = env::args().collect();
    if args.len() < 2 {
        return;
    }
    let mut a = fs::read_to_string(&args[1]).unwrap()
        .strip_suffix('\n').unwrap().to_string()
        .chars().collect::<Vec<char>>();

    // Part 1
    let disksize = 272;
    // Part 2
    //let disksize = 35651584;

    while a.len() < disksize {
        let l = a.len();
        a.push('0');
        for i in (0..l).rev() {
            let c = a.get(i).unwrap();
            match c {
                '0' => { a.push('1'); },
                '1' => { a.push('0'); },
                _ => {}
            }
        }
    }

    let mut checksum = a.clone();
    loop {
        let mut newchecksum = Vec::new();

        let mut i = 0;
        while i < std::cmp::min(disksize, checksum.len()) {
            if checksum.get(i).unwrap() == 
               checksum.get(i+1).unwrap() {
                newchecksum.push('1');
            } else {
                newchecksum.push('0');
            }
            i += 2;
        }

        checksum = newchecksum;
        if checksum.len() % 2 == 1 {
            break;
        }
    }

    for c in checksum {
        print!("{}", c);
    }
    println!();
}

