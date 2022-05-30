//! ## Labyrinth
//!
//! This module contains the data structure representing a maze for the rest of the library.

use std::collections::HashMap;

use crate::Pos;

/// Data structure representing a maze on a grid.
/// stores each possible paths as a [`HashMap`] mapping each positions to the accessible adjascent ones.
#[derive(Debug, Clone)]
pub struct Maze {
    width: isize,
    height: isize,
    start: Pos,
    end: Pos,
    paths: HashMap<Pos, Vec<Pos>>,
}

impl Maze {
    /// Constructor.
    pub fn new(
        width: isize,
        height: isize,
        start: Pos,
        end: Pos,
        paths_: Vec<(Pos, Vec<Pos>)>,
    ) -> Self {
        let mut paths = HashMap::new();

        for y in 0..height {
            for x in 0..width {
                paths.insert((x, y).into(), Vec::new());
            }
        }

        let mut result = Self {
            width,
            height,
            start,
            end,
            paths,
        };

        for (position, accessibles) in paths_ {
            for accessible in accessibles {
                result.create_path(position, accessible);
            }
        }

        result
    }

    fn create_path(&mut self, position_a: Pos, position_b: Pos) {
        self.paths
            .get_mut(&position_a)
            .expect("position out of bounds")
            .push(position_b); // warning: mutation before all preconditions are checked
        self.paths
            .get_mut(&position_b)
            .expect("position out of bounds")
            .push(position_a);
    }

    /// Width of the [`Maze`].
    pub fn width(&self) -> isize {
        self.width
    }

    /// Height of the [`Maze`].
    pub fn height(&self) -> isize {
        self.height
    }

    /// Tuple containing both the width and height of the [`Maze`].
    pub fn size(&self) -> (isize, isize) {
        (self.width(), self.height())
    }

    /// The starting position of the [`Maze`].
    pub fn start(&self) -> Pos {
        self.start
    }

    /// The ending position of the [`Maze`].
    pub fn end(&self) -> Pos {
        self.end
    }

    /// Check if the provided position is the start of the [`Maze`].
    pub fn is_start(&self, position: Pos) -> bool {
        self.start() == position
    }

    /// Check if the provided position is the end of the [`Maze`].
    pub fn is_end(&self, position: Pos) -> bool {
        self.end() == position
    }

    /// Returns an array of all positions directly accessible from a position in the [`Maze`].
    pub fn paths_from(&self, position: Pos) -> &[Pos] {
        let accessibles = self.paths.get(&position).expect("position out of bounds");
        accessibles
    }

    /// Check if a position is included within the [`Maze`].
    pub fn is_inside(&self, position: Pos) -> bool {
        let (x, y) = position.decompose();
        x >= 0 && x < self.width() && y >= 0 && y < self.height()
    }

    /// Returns adjascent positions of `position` that are included in the [`Maze`].
    pub fn adjascent(&self, position: Pos) -> Vec<Pos> {
        let (x, y) = position.decompose();
        [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
            .into_iter()
            .map(|t| t.into())
            .filter(|&p| self.is_inside(p))
            .collect()
    }

    /// Check if there is a wall between two adjascent positions in the [`Maze`].
    pub fn is_walled(&self, position_a: Pos, position_b: Pos) -> bool {
        self.paths_from(position_a)
            .iter()
            .find(|&&p| p == position_b)
            .is_none()
    }

    /// return a string representing the [`Maze`].
    pub fn display(&self, overlay: Option<HashMap<Pos, char>>) -> String {
        // output
        let mut out: Vec<Vec<_>> = (0..(self.height() * 2 + 1))
            .map(|_| (0..(self.width() * 2 + 1)).map(|_| ' ').collect())
            .collect();

        // outer walls
        for x in 0..self.width() {
            let mapped_x = (x * 2 + 1) as usize;
            out.first_mut().unwrap()[mapped_x] = '─';
            out.last_mut().unwrap()[mapped_x] = '─';
        }
        for y in 0..self.height() {
            let mapped_y = (y * 2 + 1) as usize;
            *out[mapped_y].first_mut().unwrap() = '│';
            *out[mapped_y].last_mut().unwrap() = '│';
        }

        // vertical walls
        for y in 0..self.height() {
            for x in 1..self.width() {
                let current_cell = (x, y).into();
                let left_cell = current_cell - (1, 0).into();
                if self.is_walled(left_cell, current_cell) {
                    let mapped_y = (2 * y + 1) as usize;
                    let mapped_x = (2 * x) as usize;
                    out[mapped_y][mapped_x] = '│'
                }
            }
        }
        // horizontal walls
        for y in 1..self.height() {
            for x in 0..self.width() {
                let current_cell = (x, y).into();
                let upper_cell = current_cell - (0, 1).into();
                if self.is_walled(upper_cell, current_cell) {
                    let mapped_x = (2 * x + 1) as usize;
                    let mapped_y = (2 * y) as usize;
                    out[mapped_y][mapped_x] = '─';
                }
            }
        }
        // corners
        for y in 0..(self.height() + 1) {
            for x in 0..(self.width() + 1) {
                let mapped_x = (2 * x) as usize;
                let mapped_y = (2 * y) as usize;
                out[mapped_y][mapped_x] = '•';
            }
        }

        // overlay
        if let Some(overlay) = overlay {
            for (position, character) in overlay {
                let (x, y) = position.decompose();
                let mapped_x = (x * 2 + 1) as usize;
                let mapped_y = (y * 2 + 1) as usize;
                out[mapped_y][mapped_x] = character;
            }
        }

        out.into_iter()
            .map(|line| line.into_iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[test]
fn display() {
    let maze = Maze::new(
        3,
        3,
        (0, 0).into(),
        (2, 2).into(),
        vec![
            ((0, 0).into(), vec![(1, 0).into(), (0, 1).into()]),
            ((1, 0).into(), vec![(1, 1).into()]),
            ((1, 1).into(), vec![(1, 2).into()]),
            ((1, 2).into(), vec![(2, 2).into()]),
        ],
    );

    let text = maze.display(None);
    println!("{text}");
}

pub mod generator;
