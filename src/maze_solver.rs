use std::{collections::{VecDeque, HashSet, HashMap}, time::SystemTime};

pub fn iterative_bfs_solver(graph: &HashMap<usize, HashSet<usize>>, from:usize, to:usize) -> Vec<usize> {

    println!("Solving maze...");
    let start_time = SystemTime::now();

    let mut frontier: VecDeque<usize> = VecDeque::new();
    let mut path: Vec<usize> = Vec::new();
    let mut visited: Vec<usize> = Vec::new();


    visited.resize(graph.len(), usize::MAX);

    frontier.push_front(from);
    visited[from as usize] = from;

    while !frontier.is_empty() {
        let p = frontier.pop_front();

        if p.unwrap() == to {
            break;
        }

        let nbrs = graph.get(&p.unwrap()).expect("Always should be an element");

        for n in nbrs {
            if visited[*n as usize] == usize::MAX {
                visited[*n as usize] = p.unwrap();
                frontier.push_back(*n);
            }
        }
    }
    println!("Building path...");

    let mut p = to;
    path.push(p as usize);

    while p != from {
        p = visited[p];
        path.push(p as usize);
    }

    path.reverse();

    let end_time = SystemTime::now();
    let duration = end_time.duration_since(start_time).unwrap();
    println!("Solving maze took {} seconds", duration.as_secs());

    return path;
}
