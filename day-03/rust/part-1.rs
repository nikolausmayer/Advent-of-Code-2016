use std::env;
use std::fs;

fn main()
{
    let args : Vec<String> = env::args().collect();
    if args.len() < 2 {
        return;
    }

    let mut possible_triangles = 0;
    for line in fs::read_to_string(&args[1]).unwrap().split('\n') {
        if line == "" {
            continue;
        }

        let v = line.split(' ')
                    .filter(|&x| x != "")
                    .map(|x| x.parse().unwrap())
                    .collect::<Vec<i32>>();

        if v[0] + v[1] > v[2] &&
           v[0] + v[2] > v[1] &&
           v[1] + v[2] > v[0] {
            possible_triangles += 1;
        }
    }

    println!("{}", possible_triangles);
}

