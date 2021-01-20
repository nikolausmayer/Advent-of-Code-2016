use std::env;
use std::fs;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap};

#[derive(Copy, Clone, Eq, PartialEq)]
struct State
{
    cost: i32,
    heuristic: i32,
    x: i32,
    y: i32,
}
impl Ord for State 
{
    fn cmp(&self, other : &State) -> std::cmp::Ordering
    {
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
    let lines = fs::read_to_string(&args[1]).unwrap();

    let mut map : Vec<Vec<i32>> = Vec::new();
    let mut keys = Vec::<i32>::new();
    let mut key_locations = BTreeMap::<i32, (i32, i32)>::new();

    let mut y = 0;
    for line in lines.split('\n') {
        if line == "" {
            continue;
        }
        let mut lv = Vec::new();
        let mut x = 0;
        for c in line.chars() {
            match c {
                '.' => { lv.push(1); },
                '#' => { lv.push(0); },
                '\n' => {},
                _ => {
                    let ikey = c as i32 - '0' as i32;
                    keys.push(ikey);
                    key_locations.insert(ikey, (x, y));
                    lv.push(2);
                }
            }
            x += 1;
        }
        map.push(lv);
        y += 1;
    }
    let H = map.len();
    let W = map[0].len();

    let mut key_distances = BTreeMap::<(i32, i32), i32>::new();

    for from_key in &keys {
        key_distances.insert((*from_key, *from_key), 0);
        for to_key in &keys {
            if from_key >= to_key {
                continue;
            }

            let target_x = key_locations[to_key].0;
            let target_y = key_locations[to_key].1;

            let mut heap = BinaryHeap::<State>::new();
            heap.push(State{
                cost: 0,
                heuristic: 1,
                x: key_locations[from_key].0,
                y: key_locations[from_key].1
            });
            let mut visited = BTreeSet::new();
            #[allow(irrefutable_let_patterns)]
            while let node = heap.pop() {
                if node == None {
                    break;
                }
                let node = node.unwrap();
                if node.heuristic == 0 {
                    key_distances.insert((*from_key, *to_key), node.cost);
                    key_distances.insert((*to_key, *from_key), node.cost);
                    break;
                }
                if visited.contains(&(node.x, node.y)) {
                    continue;
                }
                visited.insert((node.x, node.y));

                let x = node.x as usize;
                let y = node.y as usize;
                if x > 0 && map[y][x-1] == 1 ||
                   (map[y][x-1] == 2 && 
                    y as i32 == target_y && 
                    (x-1) as i32 == target_x) {
                    let mut next = node.clone();
                    next.cost += 1;
                    next.x -= 1;
                    next.heuristic = (target_x - (next.x as i32)).abs() + (target_y - (next.y as i32)).abs();
                    heap.push(next);
                }
                if x < W && map[y][x+1] == 1 ||
                   (map[y][x+1] == 2 && 
                    y as i32 == target_y && 
                    (x+1) as i32 == target_x) {
                    let mut next = node.clone();
                    next.cost += 1;
                    next.x += 1;
                    next.heuristic = (target_x - (next.x as i32)).abs() + (target_y - (next.y as i32)).abs();
                    heap.push(next);
                }
                if y > 0 && map[y-1][x] == 1 ||
                   (map[y-1][x] == 2 && 
                    (y-1) as i32 == target_y && 
                    x as i32 == target_x) {
                    let mut next = node.clone();
                    next.cost += 1;
                    next.y -= 1;
                    next.heuristic = (target_x - (next.x as i32)).abs() + (target_y - (next.y as i32)).abs();
                    heap.push(next);
                }
                if y < H && map[y+1][x] == 1 ||
                   (map[y+1][x] == 2 && 
                    (y+1) as i32 == target_y && 
                    x as i32 == target_x) {
                    let mut next = node.clone();
                    next.cost += 1;
                    next.y += 1;
                    next.heuristic = (target_x - (next.x as i32)).abs() + (target_y - (next.y as i32)).abs();
                    heap.push(next);
                }
            }
        }
    }

    let mut mindist = std::i32::MAX;
    fn recurser (keys_left : Vec<i32>, last_key : i32, dist : i32, mut mindist : &mut i32, distances : BTreeMap<(i32, i32), i32>) {
        if keys_left.len() == 0 {
            if dist < *mindist {
                *mindist = dist;
            }
        } else {
            for i in 0..keys_left.len() {
                let next_key = keys_left[i];
                if !distances.contains_key(&(last_key, next_key)) {
                    continue;
                }
                let to_next = distances[&(last_key, next_key)];
                let mut rest = keys_left.clone();
                rest.remove(i);
                let mut newdist = distances.clone();
                for key_a in &keys_left {
                    for key_b in &keys_left {
                        if newdist.contains_key(&(*key_a, last_key)) && 
                           newdist.contains_key(&(last_key, *key_b)) {
                            let sum = newdist[&(*key_a, last_key)] +
                                      newdist[&(last_key, *key_b)];
                            if !newdist.contains_key(&(*key_a, *key_b)) ||
                               sum < newdist[&(*key_a, *key_b)] {
                                   newdist.insert((*key_a, *key_b), sum);
                                   newdist.insert((*key_b, *key_a), sum);
                            }
                        }
                    }
                }
                recurser(rest, next_key, dist + to_next, mindist, newdist);
            }
        }
    }
    let keys_to_get = keys.iter().filter(|&&x| x != 0).cloned().collect::<Vec<i32>>();
    let mut newdist = key_distances.clone();
    for key_a in &keys_to_get {
        for key_b in &keys_to_get {
            if newdist.contains_key(&(*key_a, 0)) && 
               newdist.contains_key(&(0, *key_b)) {
                let sum = newdist[&(*key_a, 0)] +
                          newdist[&(0, *key_b)];
                if !newdist.contains_key(&(*key_a, *key_b)) ||
                   sum < newdist[&(*key_a, *key_b)] {
                       newdist.insert((*key_a, *key_b), sum);
                       newdist.insert((*key_b, *key_a), sum);
                }
            }
        }
    }
    recurser(keys_to_get, 0, 0, &mut mindist, newdist);
    println!("{}", mindist);
}

