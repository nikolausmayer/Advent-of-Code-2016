use std::env;
use std::fs;
use regex;
use std::collections::{BinaryHeap, BTreeMap, BTreeSet};

#[derive(Clone, Eq, PartialEq)]
struct State {
    cost : u32,
    heuristic : u32,
    x : u16,
    y : u16,
    usage : Vec<u16>,
    //history : Vec<String>,
}
impl State {
    fn get_heuristic(&self) -> u32 
    {
        (self.x + self.y) as u32
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (other.cost + other.heuristic).cmp(&(self.cost + self.heuristic))
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}


fn main(){
    let args : Vec<String> = env::args().collect();
    if args.len() < 2 {
        return;
    }
    let input = fs::read_to_string(&args[1]).unwrap();

    let re = regex::Regex::new(r"/dev/grid/node-x(\d+)-y(\d+) +(\d+)T +(\d+)T +(\d+)T").unwrap();

    let mut map = BTreeMap::<(u16, u16), (u16, u16)>::new();

    let mut maxx : u16 = 0;
    let mut maxy : u16 = 0;

    for line in input.split('\n') {
        match re.captures(line) {
            None => {},
            Some(i) => {
                let x : u16 = i.get(1).unwrap().as_str().parse().unwrap();
                let y : u16 = i.get(2).unwrap().as_str().parse().unwrap();
                let s : u16 = i.get(3).unwrap().as_str().parse().unwrap();
                let a : u16 = i.get(4).unwrap().as_str().parse().unwrap();
                map.insert((x, y), (s, a));
                maxx = maxx.max((x+1) as u16);
                maxy = maxx.max((y+1) as u16);
            }
        }
    }

    let mut sizes = Vec::new();
    for y in 0..maxy {
        for x in 0..maxx {
            sizes.push(map.get(&(x, y)).unwrap().0);
        }
    }

    let mut heap = BinaryHeap::new();
    {
        let mut usage = Vec::new();
        for y in 0..maxy {
            for x in 0..maxx {
                usage.push(map.get(&(x, y)).unwrap().1);
            }
        }
        heap.push(State{
            cost: 0,
            heuristic: 0,
            x: (maxx-1),
            y: 0,
            usage,
            //history : Vec::new()
        });
    }
    let mut visited = BTreeSet::new();
    #[allow(irrefutable_let_patterns)]
    while let node = heap.pop() {
        if node == None {
            break;
        }
        let node = node.unwrap();

        if node.x == 0 && node.y == 0 {
            println!("{}", node.cost);
            //for step in node.history {
            //    println!("{}", step);
            //}
            break;
        }

        {
            let mut hash = node.usage.clone().into_iter().map(|x| (x > 0) as u16).collect::<Vec<u16>>();
            hash.push(node.x as u16);
            hash.push(node.y as u16);
            if visited.contains(&hash) {
                continue;
            }
            visited.insert(hash);
        }


        let mut free = Vec::new();
        for y in 0..maxy {
            for x in 0..maxx {
                free.push(map.get(&(x, y)).unwrap().0 - node.usage[(y*maxx+x) as usize]);
            }
        }
        for y in 0..maxy {
            for x in 0..maxx {
                let i = y as usize * maxx as usize + x as usize;

                if node.usage[i] == 0 {
                    continue;
                }

                if x > 0 && free[i-1] >= node.usage[i] {
                    //println!("{},{} can move <", x, y);
                    let mut next = node.clone();
                    next.cost += 1;
                    if x == next.x && y == next.y {
                        next.x -= 1;
                        //next.history.push(format!("! {},{} <", x, y));
                    } else {
                        //next.history.push(format!("{},{} <", x, y));
                    }
                    next.usage[i-1] += next.usage[i];
                    next.usage[i] = 0;
                    next.heuristic = next.get_heuristic();
                    {
                        let mut hash = next.usage.clone().into_iter().map(|x| (x > 0) as u16).collect::<Vec<u16>>();
                        hash.push(next.x as u16);
                        hash.push(next.y as u16);
                        if !visited.contains(&hash) {
                            heap.push(next);
                        }
                    }
                }

                if y > 0 && free[i-maxx as usize] >= node.usage[i] {
                    //println!("{},{} can move ^", x, y);
                    let mut next = node.clone();
                    next.cost += 1;
                    if x == next.x && y == next.y {
                        next.y -= 1;
                        //next.history.push(format!("! {},{} ^", x, y));
                    } else {
                        //next.history.push(format!("{},{} ^", x, y));
                    }
                    next.usage[i-maxx as usize] += next.usage[i];
                    next.usage[i] = 0;
                    next.heuristic = next.get_heuristic();
                    {
                        let mut hash = next.usage.clone().into_iter().map(|x| (x > 0) as u16).collect::<Vec<u16>>();
                        hash.push(next.x as u16);
                        hash.push(next.y as u16);
                        if !visited.contains(&hash) {
                            heap.push(next);
                        }
                    }
                }

                if x < maxx-1 && free[i+1] >= node.usage[i] {
                    //println!("{},{} can move >", x, y);
                    let mut next = node.clone();
                    next.cost += 1;
                    if x == next.x && y == next.y {
                        next.x += 1;
                        //next.history.push(format!("! {},{} >", x, y));
                    } else {
                        //next.history.push(format!("{},{} >", x, y));
                    }
                    next.usage[i+1] += next.usage[i];
                    next.usage[i] = 0;
                    next.heuristic = next.get_heuristic();
                    {
                        let mut hash = next.usage.clone().into_iter().map(|x| (x > 0) as u16).collect::<Vec<u16>>();
                        hash.push(next.x as u16);
                        hash.push(next.y as u16);
                        if !visited.contains(&hash) {
                            heap.push(next);
                        }
                    }
                }

                if y < maxy-1 && free[i+maxx as usize] >= node.usage[i] {
                    //println!("{},{} can move v ({} into {})", x, y, node.usage[i], free[i+maxx as usize]);
                    let mut next = node.clone();
                    next.cost += 1;
                    if x == next.x && y == next.y {
                        next.y += 1;
                        //next.history.push(format!("! {},{} v", x, y));
                    } else {
                        //next.history.push(format!("{},{} v", x, y));
                    }

                    next.usage[i+maxx as usize] += next.usage[i];
                    next.usage[i] = 0;
                    next.heuristic = next.get_heuristic();
                    {
                        let mut hash = next.usage.clone().into_iter().map(|x| (x > 0) as u16).collect::<Vec<u16>>();
                        hash.push(next.x as u16);
                        hash.push(next.y as u16);
                        if !visited.contains(&hash) {
                            heap.push(next);
                        }
                    }
                }
            }
        }
    }
}

