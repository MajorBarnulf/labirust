use std::time::Duration;

use crate::{labyrinth::generator::MazeGenerator, Maze};

use self::maze_state::{BuildableMazeState, Generated, MazeState, Provided, Unprovided};

pub mod maze_state {

    use crate::{labyrinth::generator::MazeGenerator, Maze};

    pub trait MazeState {}
    pub trait BuildableMazeState: MazeState {
        fn get(self) -> Maze;
    }

    pub struct Unprovided;

    impl MazeState for Unprovided {}

    pub struct Provided {
        maze: Maze,
    }

    impl Provided {
        pub fn new(maze: Maze) -> Self {
            Self { maze }
        }
    }

    impl MazeState for Provided {}
    impl BuildableMazeState for Provided {
        fn get(self) -> Maze {
            self.maze
        }
    }

    pub struct Generated {
        generator: Box<dyn MazeGenerator>,
    }

    impl Generated {
        pub fn new<G>(generator: G) -> Self
        where
            G: MazeGenerator + 'static,
        {
            let generator = Box::new(generator);
            Self { generator }
        }
    }

    impl MazeState for Generated {}
    impl BuildableMazeState for Generated {
        fn get(self) -> Maze {
            self.generator.generate()
        }
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

impl<MS: MazeState> ExecutorBuilder<MS> {
    pub fn maze(self, maze: Maze) -> ExecutorBuilder<Provided> {
        let Self {
            delay,
            maze_state: _,
        } = self;
        ExecutorBuilder {
            delay,
            maze_state: Provided::new(maze),
        }
    }
    pub fn generated<G>(self, generator: G) -> ExecutorBuilder<Generated>
    where
        G: MazeGenerator + 'static,
    {
        let Self {
            delay,
            maze_state: _,
        } = self;
        ExecutorBuilder {
            delay,
            maze_state: Generated::new(generator),
        }
    }
    pub fn delay_ms(self, delay: u64) -> Self {
        let delay = Duration::from_millis(delay);
        let Self {
            maze_state,
            delay: _,
        } = self;
        Self { maze_state, delay }
    }
}

impl<MS: BuildableMazeState> ExecutorBuilder<MS> {
    pub fn build(self) -> (Maze, Duration) {
        let maze = self.maze_state.get();
        let delay = self.delay;
        (maze, delay)
    }
}
