//! ## Algorithm
//!
//! This module contains the definition of the [`Algorithm`] trait, implemented by [`Maze`] resolution strategies.
//! Already existing implementations of that trait can be found in the [`crate::implementations`] module.
//!

use crate::{Context, Guess, Insight};

/// Trait encapsulating the behavior of an algorithm solving mazes.
/// Implementing this trait is done by providing a `progress` method which gets called iteratively on each steps of a [`Maze`] resolution.
pub trait Algorithm {
    /// will be called on each step of the traversal of the [`Maze`].
    /// `insight` is a view on the position discovered on the previous movement.
    /// `ctx` is a view on the [`Maze`], useful for accessing properties of the maze.
    fn progress(&mut self, insight: &Insight, ctx: &mut Context) -> Guess;
}
