use std::env;
use std::fs;
use regex::{Regex};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, BTreeMap, BTreeSet};

#[derive(Eq, PartialEq)]
struct State {
    cost: u8,
    heuristic: u8,
    data: BTreeMap<i8, u8>,
    //history: Vec<BTreeMap<i8, u8>>
}
fn get_heuristic(map : &BTreeMap<i8, u8>) -> u8 {
    map.keys()
       .filter(|&x| x != &0)
       .map(|x| 4 - map[x])
       .sum::<u8>() / 2
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // "other cmp self" is a "min" ordering which is 
        // reverse to the default
        (other.cost + other.heuristic).cmp(&(self.cost + self.heuristic))
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Clone for State {
    fn clone(&self) -> Self {
        let mut data : BTreeMap<i8, u8> = BTreeMap::new();
        for (key, value) in self.data.iter() {
            data.insert(*key, *value);
        }
        //let mut history = Vec::new();
        //for item in &self.history {
        //    let mut e : BTreeMap<i8, u8> = BTreeMap::new();
        //    for (key, value) in item.iter() {
        //        e.insert(*key, *value);
        //    }
        //    history.push(e);
        //}
        State{data, /*history,*/ ..*self}
    }
}

fn print_state(state : &BTreeMap<i8, u8>, 
               names : &BTreeMap<i8, String>) {
    for floor in (1..5).rev() {
        print!("F{} ", floor);
        print!("{} ", {if state[&0] == floor {"E "} else {". "}});
        for i in 1..state.keys().max().unwrap()+1 {
            let e = &names[&i];
            if state[&i] == floor {
                print!("{}G ", e);
            } else {
                print!(".   ");
            }
            if state[&-i] == floor {
                print!("{}M ", e);
            } else {
                print!(".   ");
            }
        }
        println!();
    }
}



fn main(){
    let args : Vec<String> = env::args().collect();
    if args.len() < 2 {
        return;
    }
    let lines = fs::read_to_string(&args[1]).unwrap();

    let re_gen = Regex::new(r"a (\w+) generator").unwrap();
    let re_chip = Regex::new(r"a (\w+)-compatible").unwrap();

    let mut initial_data = BTreeMap::new();
    // Elevator is "0" on floor 1
    initial_data.insert(0, 1);

    let mut elements : BTreeMap<String, i8> = BTreeMap::new();
    {
        let mut floorcount = 1;
        for line in lines.split('\n') {
            if line == "" {
                continue;
            }

            // Part 2
            if floorcount == 1 {
                {
                    let elem = "elerium".to_string();
                    let newkey = elements.values()
                                         .max()
                                         .unwrap_or(&0) 
                                 + 1;
                    let key = elements.entry(elem)
                                      .or_insert(newkey);
                    initial_data.insert(*key, floorcount);
                    initial_data.insert(-*key, floorcount);
                }
                {
                    let elem = "dilithium".to_string();
                    let newkey = elements.values()
                                         .max()
                                         .unwrap_or(&0) 
                                 + 1;
                    let key = elements.entry(elem)
                                      .or_insert(newkey);
                    initial_data.insert(*key, floorcount);
                    initial_data.insert(-*key, floorcount);
                }
            }
            // <- Part 2

            for cap in re_gen.captures_iter(line) {
                let elem = cap.get(1).unwrap().as_str().to_string();
                let newkey = elements.values()
                                     .max()
                                     .unwrap_or(&0) 
                             + 1;
                let key = elements.entry(elem)
                                  .or_insert(newkey);

                initial_data.insert(*key, floorcount);
            }
            for cap in re_chip.captures_iter(line) {
                let elem = cap.get(1).unwrap().as_str().to_string();
                let newkey = elements.values()
                                     .max()
                                     .unwrap_or(&0) + 1;
                let key = elements.entry(elem)
                                  .or_insert(newkey);

                initial_data.insert(-*key, floorcount);
            }

            floorcount += 1;
        }
    }
    let mut revelements = BTreeMap::<i8, String>::new();
    for (elem, key) in &elements {
        revelements.insert(*key, elem[0..2].to_string());
    }

    let mut visited = BTreeSet::<BTreeMap<i8, u8>>::new();

    let mut heap = BinaryHeap::new();
    heap.push(State{
        cost: 0, 
        heuristic: get_heuristic(&initial_data), 
        data: initial_data,
        //history: Vec::new()
    });
    #[allow(irrefutable_let_patterns)]
    while let node = heap.pop() {
        if node == None {
            break;
        }
        let node = node.unwrap();

        if visited.contains(&node.data) {
            continue;
        }
        visited.insert(node.data.clone());

        let elevator_floor = node.data[&0];
        let mut things_here = node.data.keys()
            .filter(|&x| node.data[x] == elevator_floor)
            .cloned()
            .collect::<Vec<i8>>();
        things_here.sort();

        if node.data.values().into_iter().min().unwrap() == &4 {
            //println!();
            //let mut stepcount = 0;
            //for item in &node.history {
            //    println!("{}", stepcount);
            //    stepcount += 1;
            //    print_state(&item, &revelements);
            //    println!();
            //}
            //println!("{}", stepcount);
            //print_state(&node.data, &revelements);
            println!();
            println!("{}", node.cost);
            break;
        }

        for thing_a in &things_here {
            for thing_b in &things_here {
                if *thing_a >= *thing_b {
                    continue;
                }
                if elevator_floor > 1 {
                    let mut next = State{..node.clone()};
                    next.cost += 1;
                    *next.data.get_mut(&0).unwrap() -= 1;
                    if *thing_a != 0 {
                        *next.data.get_mut(&thing_a).unwrap() -= 1;
                    }
                    if *thing_b != 0 {
                        *next.data.get_mut(&thing_b).unwrap() -= 1;
                    }

                    if !visited.contains(&next.data) {
                        let mut things_here = next.data.keys()
                            .filter(|&x| next.data[x] == elevator_floor-1)
                            .cloned()
                            .collect::<Vec<i8>>();
                        things_here.sort();
                        let unpaired_chip_here = things_here.iter()
                            .filter(|&x| *x < 0 && !things_here.contains(&-x))
                            .cloned()
                            .collect::<Vec<i8>>();
                        let rtg_here = things_here.iter()
                            .filter(|&x| *x > 0)
                            .cloned()
                            .collect::<Vec<i8>>();
                        if !(rtg_here.len() > 0 && unpaired_chip_here.len() > 0) {
                            next.heuristic = get_heuristic(&next.data);
                            //next.history.push(node.data.clone());
                            heap.push(next);
                        }
                    }
                }
                if elevator_floor < 4 {
                    let mut next = State{..node.clone()};
                    next.cost += 1;
                    *next.data.get_mut(&0).unwrap() += 1;
                    if *thing_a != 0 {
                        *next.data.get_mut(&thing_a).unwrap() += 1;
                    }
                    if *thing_b != 0 {
                        *next.data.get_mut(&thing_b).unwrap() += 1;
                    }

                    if !visited.contains(&next.data) {
                        let mut things_here = next.data.keys()
                            .filter(|&x| next.data[x] == elevator_floor+1)
                            .cloned()
                            .collect::<Vec<i8>>();
                        things_here.sort();
                        let unpaired_chip_here = things_here.iter()
                            .filter(|&x| *x < 0 && !things_here.contains(&-x))
                            .cloned()
                            .collect::<Vec<i8>>();
                        let rtg_here = things_here.iter()
                            .filter(|&x| *x > 0)
                            .cloned()
                            .collect::<Vec<i8>>();
                        if !(rtg_here.len() > 0 && unpaired_chip_here.len() > 0) {
                            next.heuristic = get_heuristic(&next.data);
                            //next.history.push(node.data.clone());
                            heap.push(next);
                        }
                    }
                }
            }
        }

    }
}

