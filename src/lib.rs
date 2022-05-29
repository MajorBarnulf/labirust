//! # Labirust
//!
//! This crate is a small and naive implementation of several [`Algorithm`]s resolving [`Maze`]s.
//!
//! * It exposes the [`Algorithm`] trait encapsulating the behavior of such an algorithm.
//! * It also provides structures to generate [`Maze`] ([`generate`]) and execute said algorithms on them ([`Executor`]).
//!

mod algorithm;
pub mod implementations;
mod labyrinth;
mod position;

pub use algorithm::{Algorithm, Executor};
pub use labyrinth::{generator::generate, Maze};
pub use position::Pos;
