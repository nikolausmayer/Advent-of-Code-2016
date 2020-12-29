use std::collections::HashMap;
use std::env;
use std::fs;

fn main()
{
    let args : Vec<String> = env::args().collect();
    if args.len() < 2 {
        return;
    }

    let mut valid_rooms = 0;
    for line in fs::read_to_string(&args[1]).unwrap().split('\n') {
        let mut room_valid : bool = true;
        let mut map = HashMap::new();
        let mut match_phase : bool = false;
        let mut sector_id : i32 = 0;
        for letter in line.chars() {
            match letter {
                '0'..='9' => {
                    sector_id = sector_id * 10 + letter.to_digit(10).unwrap() as i32;
                }
                '-' => {},
                '[' => { 
                    if map.keys().count() < 5 {
                        room_valid = false;
                        break;
                    }
                    match_phase = true;
                },
                ']' => {},
                 _  => {
                    if !match_phase {
                        if !map.contains_key(&letter) {
                            map.insert(letter, 0);
                        }
                        *map.get_mut(&letter).unwrap() += 1;
                    } else {
                        let maxcount = map.values().max().unwrap();
                        let mut top = map.keys()
                                         .filter(|x| map[x] == *maxcount)
                                         .collect::<Vec<&char>>();
                        top.sort();
                        let l = top[0].clone();
                        map.remove(&l);
                        if letter != l {
                            room_valid = false;
                            break;
                        }
                    }
                }
            }
        }
        if room_valid {
            valid_rooms += sector_id;
        }
    }
    println!("{}", valid_rooms);
}

