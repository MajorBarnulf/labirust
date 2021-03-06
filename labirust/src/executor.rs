//! ## Executor
//!
//! This module contains the definition of an [`Executor`], used to run an [`Algorithm`] and have a graphical output in the terminal.
//! This type is supposed to be created using the builder pattern (c.f. [`Executor`]`::build`).

use std::{
    collections::{HashMap, HashSet},
    thread,
    time::Duration,
};

use crate::{Algorithm, Maze, Pos};

use self::builder::{
    maze_state::{BuildableMazeState, Unprovided},
    new_builder, DynExecutorBuilder, ExecutorBuilder,
};

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

mod builder;

/// A structure holding a [`Maze`] and iteratively solving it with a provided [`Algorithm`].
pub struct Executor {
    delay: Duration,
    maze: Maze,
    algorithm: Box<dyn Algorithm>,
}

impl Executor {
    /// Constructor.
    fn new(maze: Maze, algorithm: Box<dyn Algorithm>, delay: Duration) -> Self {
        Self {
            maze,
            algorithm,
            delay,
        }
    }

    pub fn build<'f, A, F, MS>(algorithm: A, builder: F) -> Self
    where
        A: Algorithm + 'static,
        MS: BuildableMazeState,
        F: Fn(ExecutorBuilder<Unprovided>) -> ExecutorBuilder<MS>,
    {
        let operation = builder;
        let builder = (operation)(new_builder());
        let (maze, delay) = builder.build();
        let algorithm = Box::new(algorithm);
        Self::new(maze, algorithm, delay)
    }

    pub fn build_dyn<F>(algorithm: Box<dyn Algorithm>, builder: F) -> Self
    where
        F: Fn(DynExecutorBuilder) -> DynExecutorBuilder,
    {
        let operation = builder;
        let builder = (operation)(DynExecutorBuilder::new());
        let (maze, delay) = builder.build();
        Self::new(maze, algorithm, delay)
    }

    /// Submit the maze to the [`Algorithm`] and iteratively progress through the maze driven by said algorithm.
    pub fn run(&mut self) {
        let Self {
            maze,
            algorithm,
            delay,
        } = self;
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
            thread::sleep(*delay);
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
            overlay.insert(*position, '???');
        }
        for position in path {
            overlay.insert(*position, '???');
        }
        overlay.insert(maze.start(), 'S');
        overlay.insert(maze.end(), 'E');
        overlay.insert(*path.last().unwrap(), 'G');

        let grid = maze.display(Some(overlay));
        let text = format!("tick {tick}:\n{grid}\n");

        // DIRTY!
        // print the frame on top of the previous one
        if tick > 0 {
            let count = text.lines().count();
            let up = termion::cursor::Up(count as u16);
            print!("{up}")
        }

        print!("{text}");
    }
}
