use std::env;
use std::fs;
use std::cmp::Ordering;
use std::collections::{BTreeSet, BinaryHeap};
use md5;

#[derive(Clone, Eq, PartialEq)]
struct State 
{
    cost : i32,
    path : Vec<char>,
    x : i32,
    y : i32
}
impl State {
    fn doors(&self, hash : &String) -> [bool; 4]
    {
        let mut res = [false; 4];
        let test = format!("{}{}", hash, self.path.iter().collect::<String>());
        let hash = format!("{:x}", md5::compute(&test));
        for i in 0..4 {
            let c : char = hash.chars().nth(i).unwrap();
            if c as u8 >= 'b' as u8 &&
               c as u8 <= 'f' as u8 {
                res[i] = true;
            }
        }
        res
    }

    fn heuristic(&self) -> i32
    {
        (3 - self.x) + (3 - self.y)
    }
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        (other.cost + other.heuristic()).cmp(&(self.cost + self.heuristic()))
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


fn main(){
    let args : Vec<String> = env::args().collect();
    if args.len() < 2 {
        return;
    }
    let input = fs::read_to_string(&args[1]).unwrap()
        .strip_suffix('\n').unwrap().to_string();

    
    let mut visited = BTreeSet::<Vec<char>>::new();
    let mut heap = BinaryHeap::new();
    heap.push(State{cost: 0, path: Vec::new(), x: 0, y: 0});

    #[allow(irrefutable_let_patterns)]
    while let node = heap.pop() {
        if node == None {
            break;
        }
        let node = node.unwrap();

        if node.heuristic() == 0 {
            for c in node.path {
                print!("{}", c);
            }
            println!();
            break;
        }

        if visited.contains(&node.path) {
            continue;
        }
        visited.insert(node.path.clone());

        let doors = node.doors(&input);
        if node.x > 0 && doors[2] {
            let mut next = node.clone();
            next.cost += 1;
            next.path.push('L');
            next.x -= 1;
            heap.push(next);
        }
        if node.y > 0 && doors[0] {
            let mut next = node.clone();
            next.cost += 1;
            next.path.push('U');
            next.y -= 1;
            heap.push(next);
        }
        if node.x < 3 && doors[3] {
            let mut next = node.clone();
            next.cost += 1;
            next.path.push('R');
            next.x += 1;
            heap.push(next);
        }
        if node.y < 3 && doors[1] {
            let mut next = node.clone();
            next.cost += 1;
            next.path.push('D');
            next.y += 1;
            heap.push(next);
        }
    }
}

