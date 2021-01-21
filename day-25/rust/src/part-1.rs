use std::env;
use std::fs;
use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash)]
struct State
{
    regs : [i32; 4],
    iptr : i32,
    out : i32,
}

fn main(){
    let args : Vec<String> = env::args().collect();
    if args.len() < 2 {
        return;
    }
    let lines = fs::read_to_string(&args[1]).unwrap();

    let mut starting_value = 0;
    'outer: loop {
        starting_value += 1;

        let mut output = Vec::new();
        let mut registers = [starting_value, 0, 0, 0];
        let mut visited = HashSet::<State>::new();

        let instructions = lines.split('\n').collect::<Vec<&str>>();
        let mut iptr : i32 = 0;
        loop {
            if iptr < 0 || iptr >= instructions.len() as i32 - 1 {
                break;
            }
            let line = instructions[iptr as usize];
            let words = line.split(' ').collect::<Vec<&str>>();
            match *words.get(0).unwrap() {
                "cpy" => {
                    let target = words.get(2).unwrap().chars().nth(0).unwrap() as usize - 'a' as usize;
                    match words.get(1).unwrap().parse::<i32>() {
                        Ok(i) => { 
                            registers[target] = i; 
                        },
                        Err(_) => { 
                            let source = words.get(1).unwrap().chars().nth(0).unwrap() as usize - 'a' as usize;
                            registers[target] = registers[source];
                        },
                    }
                    iptr += 1;
                },
                "inc" => {
                    let target = words.get(1).unwrap().chars().nth(0).unwrap() as usize - 'a' as usize;
                    registers[target] += 1;
                    iptr += 1;
                },
                "dec" => {
                    let target = words.get(1).unwrap().chars().nth(0).unwrap() as usize - 'a' as usize;
                    registers[target] -= 1;
                    iptr += 1;
                },
                "jnz" => {
                    let mut checkval : i32;
                    match words.get(1).unwrap().parse::<i32>() {
                        Ok(i) => { 
                            checkval = i; 
                        },
                        Err(_) => { 
                            let source = words.get(1).unwrap().chars().nth(0).unwrap() as usize - 'a' as usize;
                            checkval = registers[source];
                        },
                    }

                    if checkval != 0 {
                        iptr += words.get(2).unwrap().parse::<i32>().unwrap();
                    } else {
                        iptr += 1;
                    }
                },
                "out" => {
                    let mut out;
                    match words.get(1).unwrap().parse::<i32>() {
                        Ok(i) => { 
                            out = i;
                        },
                        Err(_) => { 
                            let source = words.get(1).unwrap().chars().nth(0).unwrap() as usize - 'a' as usize;
                            out = registers[source];
                        },
                    }
                    output.push(out);
                    {
                        let state = State{regs: registers, iptr, out};
                        if visited.contains(&state) {
                            let mut last = 1 - output[0];
                            for i in &output {
                                if *i != (1 - last) {
                                    continue 'outer;
                                }
                                last = *i;
                            }
                            println!("{}", starting_value);
                            break 'outer;
                        } else {
                            visited.insert(state);
                        }
                    }
                    iptr += 1;
                },
                _ => {}
            }
        }

    }

}

