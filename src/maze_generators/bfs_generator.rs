use rand::Rng;
use speedy2d::color::Color;
use speedy2d::Graphics2D;
use std::collections::hash_map::Entry;
use std::collections::VecDeque;
use std::collections::{HashMap, HashSet};
use std::time::SystemTime;

pub fn bfs_generator(rows_cols: (usize, usize)) -> HashMap<usize, HashSet<usize>> {

    println!("Generating maze...");
    let start_time = SystemTime::now();

    let size = rows_cols.0 * rows_cols.1;
    let mut visisted_vec = vec![false; size];
    let mut graph: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut stack: VecDeque<usize> = VecDeque::new();
    let mut current = 0;
    visisted_vec = set_visited(current, visisted_vec);
    stack.push_back(current);

    //Loop
    while stack.len() > 0 {
        current = stack.pop_back().expect("Nothing to pop, this is weird");
        let unvisited_neighbours = get_unvisited_neighbours(current, rows_cols, &visisted_vec);
        if unvisited_neighbours.len() > 0 {
            let random_neighbour = get_random_neighbour(unvisited_neighbours);
            stack.push_back(current);
            let old = current;
            current = random_neighbour;
            stack.push_back(current);
            visisted_vec = set_visited(current, visisted_vec);

            match graph.entry(old) {
                Entry::Vacant(e) => {
                    let mut hash_set = HashSet::new();
                    hash_set.insert(current);
                    e.insert(hash_set);
                }
                Entry::Occupied(mut e) => {
                    e.get_mut().insert(current);
                }
            }
            match graph.entry(current) {
                Entry::Vacant(e) => {
                    let mut hash_set = HashSet::new();
                    hash_set.insert(old);
                    e.insert(hash_set);
                }
                Entry::Occupied(mut e) => {
                    e.get_mut().insert(old);
                }
            }
        }
    }
    let end_time = SystemTime::now();
    let duration = end_time.duration_since(start_time).unwrap();
    println!("Generating maze took {} seconds", duration.as_secs());
    graph
}

pub fn set_visited(pos: usize, mut vec: Vec<bool>) -> Vec<bool> {
    vec[pos] = true;
    vec
}


pub fn get_unvisited_neighbours(
    pos: usize,
    rows_cols: (usize, usize),
    vec: &Vec<bool>,
) -> Vec<usize> {
    let y = pos / rows_cols.0;
    let x = pos % rows_cols.0;
    let mut neighbours = vec![];

    if y > 0 && !vec[get_index_from_position(x, y - 1, rows_cols)] {
        neighbours.push(get_index_from_position(x, y - 1, rows_cols));
    }
    if y < rows_cols.0 - 1 && !vec[get_index_from_position(x, y + 1, rows_cols)] {
        neighbours.push(get_index_from_position(x, y + 1, rows_cols));
    }
    if x > 0 && !vec[get_index_from_position(x - 1, y, rows_cols)] {
        neighbours.push(get_index_from_position(x - 1, y, rows_cols));
    }
    if x < rows_cols.1 - 1 && !vec[get_index_from_position(x + 1, y, rows_cols)] {
        neighbours.push(get_index_from_position(x + 1, y, rows_cols));
    }
    neighbours
}

pub fn get_index_from_position(x: usize, y: usize, rows_cols: (usize, usize)) -> usize {
    y * rows_cols.1 + x
}

pub fn get_random_neighbour(neighbours: Vec<usize>) -> usize {
    let random_int = rand::thread_rng().gen_range(0..4);
    neighbours[random_int % neighbours.len()]
}

pub fn has_top(cols: usize, pos: &usize, vec: &HashSet<usize>) -> bool {
    if pos >= &cols {
        let top_pos = pos - cols;
        return vec.contains(&top_pos);
    }
    false
}
pub fn has_bottom(cols: usize, pos: &usize, vec: &HashSet<usize>) -> bool {
    let top_pos = pos + cols;
    vec.contains(&top_pos)
}
pub fn has_left(pos: &usize, vec: &HashSet<usize>) -> bool {
    if pos > &0 {
        let top_pos = pos - 1;
        return vec.contains(&top_pos);
    }
    false
}
pub fn has_rigth(pos: &usize, vec: &HashSet<usize>) -> bool {
    let top_pos = pos + 1;
    vec.contains(&top_pos)
}
pub fn draw_top(graphics: &mut Graphics2D, col: f32, row: f32, scale: f32) {
    graphics.draw_line(
        (col * scale + scale, row * scale + scale),
        (col as f32 * scale + scale * 2.0, row as f32 * scale + scale),
        1.0,
        Color::WHITE,
    );
}
pub fn draw_bottom(graphics: &mut Graphics2D, col: f32, row: f32, scale: f32) {
    graphics.draw_line(
        (col * scale + scale, row * scale + scale * 2.0),
        (col * scale + scale * 2.0, row * scale + scale * 2.0),
        1.0,
        Color::WHITE,
    );
}
pub fn draw_left(graphics: &mut Graphics2D, col: f32, row: f32, scale: f32) {
    graphics.draw_line(
        (col * scale + scale, row * scale + scale),
        (col * scale + scale, row * scale + scale * 2.0),
        1.0,
        Color::WHITE,
    );
}

pub fn draw_right(graphics: &mut Graphics2D, col: f32, row: f32, scale: f32) {
    graphics.draw_line(
        (col * scale + scale * 2.0, row * scale + scale),
        (col * scale + scale * 2.0, row * scale + scale * 2.0),
        1.0,
        Color::WHITE,
    );
}

pub fn draw_grid(graphics: &mut Graphics2D, size: u32, scale:f32){
    for row in 0..size + 1 {
        graphics.draw_line(
            (row as f32 * scale + scale, scale),
            (
                row as f32 * scale + scale,
                size as f32 * scale + scale,
            ),
            1.0,
            Color::BLACK,
        );
    }
    for col in 0..size + 1 {
        graphics.draw_line(
            (scale, col as f32 * scale + scale),
            (
                size as f32 * scale + scale,
                col as f32 * scale + scale,
            ),
            1.0,
            Color::BLACK,
        );
    }
}
pub fn draw_maze(graphics: &mut Graphics2D, graph: &HashMap<usize, HashSet<usize>>, size: usize, scale:f32){
    for (key, value) in graph.iter() {
        let row = key / size as usize;
        let col = key % size as usize;
        if has_top(size as usize, key, value) {
            draw_top(graphics, col as f32, row as f32, scale);
        }
        if has_left(key, value) {
            draw_left(graphics, col as f32, row as f32, scale);
        }
        if has_rigth(key, value) {
            draw_right(graphics, col as f32, row as f32, scale);
        }
        if has_bottom(size as usize, key, value) {
            draw_bottom(graphics, col as f32, row as f32, scale);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn assert() {
        let graph = bfs_generator((3, 3));
        for (key, value) in graph.iter() {
            let row = key / 3;
            let col = key % 3;
            if has_top(3, key, value) {
                println!("{}-{}", col, row);
            }
            if has_bottom(3, key, value) {
                println!("{}-{}", col, row);
            }
            if has_left(key, value) {
                println!("{}-{}", col, row);
            }
            if has_rigth(key, value) {
                println!("{}-{}", col, row);
            }
        }
    }

    #[test]
    fn index_by_position_rows_gt_cols() {
        let mut index;
        let mut _index = 0;
        for j in 0..3 {
            for i in 0..2 {
                index = get_index_from_position(i, j, (3, 2));
                assert_eq!(index, _index);
                _index += 1;
            }
        }
    }

    #[test]
    fn index_by_position_cols_gt_rows() {
        let mut index;
        let mut _index = 0;
        for j in 0..2 {
            for i in 0..3 {
                index = get_index_from_position(i, j, (2, 3));
                assert_eq!(index, _index);
                _index += 1;
            }
        }
    }
}
