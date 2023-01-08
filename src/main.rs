use std::collections::{HashMap, HashSet};

use rust_mazes::maze_generators::bfs_generator::{bfs_generator, draw_grid, draw_maze};

use rust_mazes::maze_solver::iterative_bfs_solver;
use speedy2d::color::Color;
use speedy2d::shape::Rectangle;
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::{Graphics2D, Window};

struct MyWindowHandler {
    graph: HashMap<usize, HashSet<usize>>,
    solution: Vec<usize>,
    size: u32,
    scale: f32,
    i: usize,
}

impl WindowHandler for MyWindowHandler {
    /**
     * Prints maze and solution
     */
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        if self.i == 0 {
            graphics.clear_screen(Color::WHITE);
            draw_grid(graphics, self.size, self.scale);
            draw_maze(graphics, &self.graph, self.size as usize, self.scale)
        }
        //draw solution
        if self.i < self.solution.len() {
            let current_item = self.solution[self.i];
            let row = current_item / self.size as usize;
            let col = current_item % self.size as usize;
            let top_left = (
                col as f32 * self.scale + self.scale,
                row as f32 * self.scale + self.scale,
            )
                .into();
            let bottom_right = (
                col as f32 * self.scale + self.scale * 2.0,
                row as f32 * self.scale + self.scale * 2.0,
            )
                .into();
            let rect = Rectangle::new(top_left, bottom_right);

            graphics.draw_rectangle(rect, Color::from_int_rgba(118, 35, 237, 220));
        }

        self.i = self.i + 1;

        // Request that we draw another frame once this one has finished
        helper.request_redraw();
    }
}
fn main() {
    let canvas_size = 700;
    let size = 799 as u32;
    let scale = canvas_size as f32 / size as f32;
    let window = Window::<()>::new_centered(
        "Mazes!",
        (
            canvas_size + (scale as u32)*2 ,
            canvas_size + (scale as u32)*2 ,
        ),
    )
    .unwrap();
    let graph = bfs_generator((size.try_into().unwrap(), size.try_into().unwrap()));

    let solution = iterative_bfs_solver(&graph, 0, graph.len() - 1);
    println!("Solution len: {}", solution.len());
    let i = 0;
    window.run_loop(MyWindowHandler {
        graph,
        solution,
        size,
        scale,
        i,
    });
}
