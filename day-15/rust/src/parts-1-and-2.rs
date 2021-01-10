use std::env;
use std::fs;

fn main(){
    let args : Vec<String> = env::args().collect();
    if args.len() < 2 {
        return;
    }
    let lines = fs::read_to_string(&args[1]).unwrap();

    let mut disks = Vec::<(i32, i32)>::new();

    for line in lines.split('\n') {
        if line == "" {
            continue;
        }
        let words = line.split(' ').collect::<Vec<&str>>();
        
        let disksize = words.get(3).unwrap().parse::<i32>().unwrap();
        let startpos = words.get(11).unwrap().to_string().strip_suffix('.').unwrap().parse::<i32>().unwrap();
        disks.push((disksize, startpos));
    }

    // Part 2
    disks.push((11, 0));
    // <- Part 2

    let mut start  : i32 = 0;
    let mut period : i32 = 1;
    let mut offset : i32 = 1;
    for (dperiod, dstart) in disks {
        while (start+offset+dstart + dperiod) % dperiod != 0 {
            start += period;
        }

        period *= dperiod;
        offset += 1;
    }

    println!("{}", start);
}

