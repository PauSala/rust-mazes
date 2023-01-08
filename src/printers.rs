use std::collections::{HashMap, HashSet};

use speedy2d::{color::Color, shape::Rectangle, Graphics2D};

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

pub fn draw_grid(graphics: &mut Graphics2D, size: u32, scale: f32) {
    for row in 0..size + 1 {
        graphics.draw_line(
            (row as f32 * scale + scale, scale),
            (row as f32 * scale + scale, size as f32 * scale + scale),
            1.0,
            Color::BLACK,
        );
    }
    for col in 0..size + 1 {
        graphics.draw_line(
            (scale, col as f32 * scale + scale),
            (size as f32 * scale + scale, col as f32 * scale + scale),
            1.0,
            Color::BLACK,
        );
    }
}
pub fn draw_maze(
    graphics: &mut Graphics2D,
    graph: &HashMap<usize, HashSet<usize>>,
    size: usize,
    scale: f32,
) {
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

pub fn draw_solution(
    graphics: &mut Graphics2D,
    solution: &Vec<usize>,
    i: usize,
    size: usize,
    scale: f32,
) {
    let current_item = solution[i];
    let row = current_item / size;
    let col = current_item % size;
    let top_left = (col as f32 * scale + scale, row as f32 * scale + scale).into();
    let bottom_right = (
        col as f32 * scale + scale * 2.0,
        row as f32 * scale + scale * 2.0,
    )
        .into();
    let rect = Rectangle::new(top_left, bottom_right);

    graphics.draw_rectangle(rect, Color::from_int_rgba(118, 35, 237, 220));
}

#[cfg(test)]
mod test {
    use crate::maze_generators::bfs_generator::bfs_generator;

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
}
