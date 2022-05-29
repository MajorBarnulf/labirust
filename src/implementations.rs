//! ## Implementations
//!
//! This module contains concrete types implementing the [`crate::Algorithm`] trait.
//! They drive the resolution of a [`crate::Maze`] through different means.
//!

mod breath_first;
mod depth_first;

pub use breath_first::BreathFirst;
pub use depth_first::DepthFirst;

#[test]
fn depth_first() {
    use crate::{generate, Executor};
    let algorithm = DepthFirst::new();
    let maze = generate(20, 20);
    let mut executor = Executor::new(maze, algorithm);
    executor.run();
}

#[test]
fn breath_first() {
    use crate::{generate, Executor};
    let algorithm = BreathFirst::new();
    let maze = generate(20, 20);
    let mut executor = Executor::new(maze, algorithm);
    executor.run();
}
