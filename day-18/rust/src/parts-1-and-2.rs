use std::env;
use std::fs;

fn main(){
    let args : Vec<String> = env::args().collect();
    if args.len() < 2 {
        return;
    }
    let input = fs::read_to_string(&args[1]).unwrap()
        .strip_suffix('\n').unwrap()
        .chars().collect::<Vec<char>>();

    let mut line = input.iter()
        .map(|&x| x == '.' )
        .collect::<Vec<bool>>();
    let mut safe_tiles = line.iter()
        .filter(|&&x| x)
        .count();

    // Part 1
    //let lines = 40;
    // Part 2
    let lines = 400000;

    for _ in 0..lines-1 {
        let mut nextline = Vec::<bool>::new();
        for i in 0..line.len() {
            let l = {i == 0 || line[i-1]};
            let c = line[i];
            let r = {i == line.len()-1 || line[i+1]};
            if (!l && !c &&  r) || 
               ( l && !c && !r) || 
               (!l &&  c &&  r) || 
               ( l &&  c && !r) {
                nextline.push(false);
            } else {
                nextline.push(true);
                safe_tiles += 1;
            }
        }
        line = nextline;
    }
    println!("{}", safe_tiles);
}

