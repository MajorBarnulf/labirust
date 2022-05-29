//! ## Algorithm
//!
//! This module contains the definition of the [`Algorithm`] trait, implemented by [`Maze`] resolution strategies.
//! Already existing implementations of that trait can be found in the [`crate::implementations`] module.
//!

use std::{
    collections::{HashMap, HashSet},
    thread,
    time::Duration,
};

use crate::{Maze, Pos};

/// A guess to pass to the current [`Executor`] at the end of every `progress` call.
pub struct Guess(Vec<Pos>);

/// An insight given to the [`Algorithm`] on every `progress` call.
/// On the first time about the starting point and every consecutive call about the tail of the previous guess.
pub struct Insight<'p> {
    position: Pos,
    paths: &'p [Pos],
}

impl<'p> Insight<'p> {
    fn new(position: Pos, paths: &'p [Pos]) -> Self {
        Self { paths, position }
    }

    fn from_position(position: Pos, maze: &'p Maze) -> Self {
        let paths = maze.paths_from(position);
        Self::new(position, paths)
    }

    /// The position of the insight.
    pub fn position(&self) -> Pos {
        self.position
    }

    /// the paths from that position.
    pub fn paths(&self) -> &[Pos] {
        self.paths
    }
}

/// A context given to the [`Algorithm`] on every `progress` call, provide informations about the maze and method to create a [`Guess`].
pub struct Context<'m> {
    maze: &'m Maze,
}

impl<'m> Context<'m> {
    fn new(maze: &'m Maze) -> Self {
        Self { maze }
    }

    /// Constructor for [`Guess`].
    /// Takes a path, that is a vector of positions from the starting point to the position to discover on the next call to `progress`.
    pub fn guess(&self, pos: Vec<Pos>) -> Guess {
        Guess(pos)
    }

    /// Returns the position of the `start` of the [`Maze`].
    pub fn start(&self) -> Pos {
        self.maze.start()
    }

    /// Returns the position of the `end` of the [`Maze`].
    pub fn end(&self) -> Pos {
        self.maze.end()
    }

    /// Returns the `width` of the [`Maze`].
    pub fn width(&self) -> isize {
        self.maze.width()
    }

    /// Returns the `height` of the [`Maze`].
    pub fn height(&self) -> isize {
        self.maze.width()
    }

    /// Returns a tuple containing both the `width` and `height` of the [`Maze`].
    pub fn size(&self) -> (isize, isize) {
        self.maze.size()
    }
}

/// Trait encapsulating the behavior of an algorithm solving mazes.
/// Implementing this trait is done by providing a `progress` method which gets called iteratively on each steps of a [`Maze`] resolution.
pub trait Algorithm {
    /// will be called on each step of the traversal of the [`Maze`].
    /// `insight` is a view on the position discovered on the previous movement.
    /// `ctx` is a view on the [`Maze`], useful for accessing properties of the maze.
    fn progress(&mut self, insight: &Insight, ctx: &mut Context) -> Guess;
}

/// A structure holding a [`Maze`] and iteratively solving it with a provided [`Algorithm`].
pub struct Executor<Algo>
where
    Algo: Algorithm,
{
    maze: Maze,
    algorithm: Algo,
}

impl<A> Executor<A>
where
    A: Algorithm,
{
    /// Constructor.
    pub fn new(maze: Maze, algorithm: A) -> Self {
        Self { maze, algorithm }
    }

    /// Submit the maze to the [`Algorithm`] and iteratively progress through the maze driven by said algorithm.
    pub fn run(&mut self) {
        let Self { maze, algorithm } = self;
        let mut insight = Insight::from_position(maze.start(), &maze);
        let mut tick = 0;
        let mut tried = HashSet::new();
        loop {
            let mut context = Context::new(maze);
            let Guess(guess) = algorithm.progress(&insight, &mut context);
            // TODO:
            // - extract metrics from the context
            // - check if path is actually a path
            guess.iter().for_each(|&p| {
                tried.insert(p);
            });
            let tail = *guess.last().expect("returned an empty path");

            // draw
            Self::draw(maze, &tried, tick, &guess);
            thread::sleep(Duration::from_millis(100));
            tick += 1;

            // check for next iteration
            if maze.is_end(tail) {
                break;
            } else {
                insight = Insight::from_position(tail, maze)
            }
        }
    }

    fn draw(maze: &Maze, tried: &HashSet<Pos>, tick: usize, path: &Vec<Pos>) {
        let mut overlay = HashMap::new();
        for position in tried {
            overlay.insert(*position, 'T');
        }

        for position in path {
            overlay.insert(*position, '#');
        }

        overlay.insert(*path.last().unwrap(), 'G');

        let text = maze.display(Some(overlay));

        // DIRTY!
        // print the frame on top of the previous one
        if tick > 0 {
            let count = text.lines().count() + 1;
            let up = termion::cursor::Up(count as u16);
            print!("{up}")
        }

        print!("tick {tick}:\n{text}\n");
    }
}