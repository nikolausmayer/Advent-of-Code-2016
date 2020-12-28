use std::env;
use std::fs;

fn main()
{
    let args : Vec<String> = env::args().collect();
    if args.len() < 2 {
        return;
    }

    let mut lines : Vec<Vec<i32>> = Vec::new();
    for line in fs::read_to_string(&args[1]).unwrap().split('\n') {
        if line == "" {
            continue;
        }
        lines.push(line.split(' ')
                   .filter(|&x| x != "")
                   .map(|x| x.parse().unwrap())
                   .collect::<Vec<i32>>());
    }

    let mut possible_triangles = 0;
    for i in 0..(lines.len()/3) {
        for j in 0..3 {
            let tri : Vec<i32> = vec![lines[i*3+0][j],
                                      lines[i*3+1][j],
                                      lines[i*3+2][j]];
            if tri[0] + tri[1] > tri[2] &&
               tri[0] + tri[2] > tri[1] &&
               tri[1] + tri[2] > tri[0] {
                possible_triangles += 1;
            }
        }
    }

    println!("{}", possible_triangles);
}

