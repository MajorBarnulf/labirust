use std::time::Duration;

use crate::{labyrinth::generator::MazeGenerator, Maze};

pub trait MazeState {}
pub trait BuildableMazeState: MazeState {
    fn get(self) -> Maze;
}

pub struct Unprovided;

impl MazeState for Unprovided {}

struct Provided {
    maze: Maze,
}

impl MazeState for Provided {}
impl BuildableMazeState for Provided {
    fn get(self) -> Maze {
        self.maze
    }
}

struct Generated {
    generator: Box<dyn MazeGenerator>,
}

impl MazeState for Generated {}
impl BuildableMazeState for Generated {
    fn get(mut self) -> Maze {
        self.generator.generate()
    }
}

pub struct ExecutorBuilder<MS>
where
    MS: MazeState,
{
    maze_state: MS,
    delay: Duration,
}

pub fn new_builder() -> ExecutorBuilder<Unprovided> {
    ExecutorBuilder {
        maze_state: Unprovided,
        delay: Duration::from_millis(100),
    }
}

impl<MS: BuildableMazeState> ExecutorBuilder<MS> {
    pub fn build(self) -> (Maze, Duration) {
        let maze = self.maze_state.get();
        let delay = self.delay;
        (maze, delay)
    }
}
