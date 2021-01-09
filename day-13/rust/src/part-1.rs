use std::env;
use std::fs;
use std::cmp::Ordering;
use std::collections::{BTreeSet, BinaryHeap};

#[derive(Copy, Clone, Eq, PartialEq)]
struct State 
{
    cost : u32,
    heuristic : u32,
    x : u32,
    y : u32
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        (other.cost + other.heuristic).cmp(&(self.cost + self.heuristic))
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn get_heuristic(x : u32, y : u32) -> u32 {
    ((31 - x as i32).abs() + (39 - y as i32).abs()) as u32
}

fn ones_bits(v : i32) -> i32 {
    (0..32).map(|x| 1<<x)
           .filter(|&x| v & x == x)
           .count() as i32
}

fn is_open_space(x : i32, y : i32, designer : i32) -> bool {
    let n = x*x + 3*x + 2*x*y + y + y*y + designer;
    ones_bits(n) % 2 == 0
}


fn main(){
    let args : Vec<String> = env::args().collect();
    if args.len() < 2 {
        return;
    }
    let input : i32 = fs::read_to_string(&args[1]).unwrap()
        .split('\n').collect::<Vec<&str>>()
        .get(0).unwrap()
        .parse::<i32>().unwrap();

    
    let mut visited = BTreeSet::<(u32, u32)>::new();
    let mut heap = BinaryHeap::new();
    heap.push(State{cost: 0, heuristic: get_heuristic(1, 1), x: 1, y: 1});

    #[allow(irrefutable_let_patterns)]
    while let node = heap.pop() {
        if node == None {
            break;
        }
        let node = node.unwrap();

        if node.heuristic == 0 {
            println!("{}", node.cost);
            break;
        }

        if visited.contains(&(node.x, node.y)) {
            continue;
        }
        visited.insert((node.x, node.y));

        for (dx,dy) in &[(-1,0),(1,0),(0,-1),(0,1)] {
            let x = node.x as i32 + dx;
            let y = node.y as i32 + dy;
            if x >= 0 && y >= 0 && is_open_space(x, y, input) {
                heap.push(State{
                    cost: node.cost + 1,
                    heuristic: get_heuristic(x as u32, y as u32),
                    x: x as u32,
                    y: y as u32
                });
            }
        }
    }
}

