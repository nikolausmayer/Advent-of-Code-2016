use std::env;
use std::fs;

fn swap(input : &String) -> String
{
    let mut words = input.split(' ').collect::<Vec<&str>>();
    match *words.get(0).unwrap() {
        "inc" => { words[0] = "dec"; },
        "dec" => { words[0] = "inc"; },
        "tgl" => { words[0] = "inc"; },
        "cpy" => { words[0] = "jnz"; },
        "jnz" => { words[0] = "cpy"; },
        _ => { words[0] = "NOP"; }
    }
    words.join(" ")
}

fn main(){
    let args : Vec<String> = env::args().collect();
    if args.len() < 2 {
        return;
    }
    let lines = fs::read_to_string(&args[1]).unwrap();

    let mut registers = [7, 0, 0, 0];
    // Part 2
    //let mut registers = [7, 0, 0, 0];
    // <-- Part 2

    let mut instructions = lines.split('\n').map(|s| s.to_string()).collect::<Vec<String>>();
    let mut iptr : i32 = 0;
    loop {
        if iptr < 0 || iptr >= instructions.len() as i32 - 1 {
            break;
        }
        let line = &instructions[iptr as usize];
        let words = line.split(' ').collect::<Vec<&str>>();
        match *words.get(0).unwrap() {
            "cpy" => {
                let target = words.get(2).unwrap().chars().nth(0).unwrap() as i32 - 'a' as i32;
                if 0 <= target && target <= 3 {
                    match words.get(1).unwrap().parse::<i32>() {
                        Ok(i) => { 
                            registers[target as usize] = i; 
                        },
                        Err(_) => { 
                            let source = words.get(1).unwrap().chars().nth(0).unwrap() as usize - 'a' as usize;
                            registers[target as usize] = registers[source];
                        },
                    }
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

                match words.get(2).unwrap().parse::<i32>() {
                    Ok(i) => {
                        if checkval != 0 && i != 0 {
                            iptr += i;
                        } else {
                            iptr += 1;
                        }
                    },
                    _ => {
                        let target = words.get(2).unwrap().chars().nth(0).unwrap() as usize - 'a' as usize;
                        if checkval != 0 && registers[target] != 0  {
                            iptr += registers[target];
                        } else {
                            iptr += 1;
                        }
                    }
                }
            },
            "tgl" => {
                let target = registers[words.get(1).unwrap().chars().nth(0).unwrap() as usize - 'a' as usize] + iptr;
                if 0 <= target && target < instructions.len() as i32 {
                    instructions[target as usize] = swap(&instructions[target as usize]);
                }
                iptr += 1;
            },
            _ => {}
        }
    }

    println!("{}", registers.get(0).unwrap());

}

