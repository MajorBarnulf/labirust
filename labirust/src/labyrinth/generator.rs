//! ## Generator
//!
//! This module contains raw functions generating mazes.

use std::collections::HashSet;

use rand::{prelude::SliceRandom, thread_rng};

use crate::{Maze, Pos};

/// Trait encapsulating the behavior of a type capable to create mazes.
pub trait MazeGenerator {
    fn generate(&self) -> Maze;
}

/// Most common maze generation technique, recursively creating paths to unvisited cells, each time choosing next direction at random.
pub struct SimpleGenerator {
    width: isize,
    height: isize,
}

impl SimpleGenerator {
    pub fn new(width: isize, height: isize) -> Self {
        Self { height, width }
    }
}

impl MazeGenerator for SimpleGenerator {
    fn generate(&self) -> Maze {
        let Self { width, height } = *self;
        let mut result = Maze::new(
            width,
            height,
            Pos::zero(),
            (width - 1, height - 1).into(),
            Vec::new(),
        );

        fn recursive(current: Pos, result: &mut Maze, visited: &mut HashSet<Pos>) {
            visited.insert(current);
            let mut adjascent_positions = result.adjascent(current);
            adjascent_positions.shuffle(&mut thread_rng());
            for neighbor in adjascent_positions {
                if visited.contains(&neighbor) {
                    continue;
                }
                result.create_path(current, neighbor);
                recursive(neighbor, result, visited);
            }
        }

        let mut visited = HashSet::new();
        let current = Pos::zero();
        recursive(current, &mut result, &mut visited);

        result
    }
}

#[test]
fn generation() {
    let generator = SimpleGenerator::new(10, 10);
    let maze = generator.generate();
    let text = maze.display(None);
    println!("{text}");
}
