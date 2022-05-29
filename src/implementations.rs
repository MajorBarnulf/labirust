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
    use crate::{Executor, SimpleGenerator};
    let algorithm = DepthFirst::new();
    let mut executor = Executor::build(algorithm, |b| b.generated(SimpleGenerator::new(40, 20)));
    executor.run();
}

#[test]
fn breath_first() {
    use crate::{Executor, SimpleGenerator};
    let algorithm = BreathFirst::new();
    let mut executor = Executor::build(algorithm, |b| b.generated(SimpleGenerator::new(40, 20)));
    executor.run();
}
