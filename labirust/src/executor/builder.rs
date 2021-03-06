use std::time::Duration;

use crate::{labyrinth::generator::MazeGenerator, Maze};

use self::maze_state::{BuildableMazeState, Generated, MazeState, Provided, Unprovided};

pub mod maze_state {

    use crate::{labyrinth::generator::MazeGenerator, Maze};

    /// Describe the state of the [`Maze`] parameter in the builder of an [`crate::Executor`]. Not ment to be implemented.
    pub trait MazeState {}
    pub trait BuildableMazeState: MazeState {
        fn get(&self) -> Maze;
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
        fn get(&self) -> Maze {
            self.maze.clone()
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

        pub fn new_dyn(generator: Box<dyn MazeGenerator + 'static>) -> Self {
            Self { generator }
        }
    }

    impl MazeState for Generated {}
    impl BuildableMazeState for Generated {
        fn get(&self) -> Maze {
            self.generator.generate()
        }
    }
}

/// Builder for an [`crate::Executor`], needs at least a [`Maze`].
pub struct ExecutorBuilder<MS>
where
    MS: MazeState,
{
    maze_state: MS,
    delay: Duration,
}

pub(crate) fn new_builder() -> ExecutorBuilder<Unprovided> {
    ExecutorBuilder {
        maze_state: Unprovided,
        delay: Duration::from_millis(100),
    }
}

impl<MS: MazeState> ExecutorBuilder<MS> {
    /// Provide a specific [`Maze`] for the execution.
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

    /// Provide a generator to generate a [`Maze`] for the execution.
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

    /// Sets the delay between terminal redraws, default is 100ms.
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
    pub(crate) fn build(self) -> (Maze, Duration) {
        let maze = self.maze_state.get();
        let delay = self.delay;
        (maze, delay)
    }
}

pub enum DynMazeState {
    None,
    Provided(Provided),
    Generated(Generated),
}

impl DynMazeState {
    pub fn get(self) -> Option<Maze> {
        match self {
            DynMazeState::None => None,
            DynMazeState::Provided(provided) => Some(provided.get()),
            DynMazeState::Generated(generated) => Some(generated.get()),
        }
    }
}

pub struct DynExecutorBuilder {
    maze: DynMazeState,
    delay: Duration,
}

impl DynExecutorBuilder {
    pub(crate) fn new() -> Self {
        Self {
            maze: DynMazeState::None,
            delay: Duration::from_millis(100),
        }
    }

    pub fn maze(self, maze: Maze) -> Self {
        let maze = DynMazeState::Provided(Provided::new(maze));
        let Self { maze: _, delay } = self;
        Self { maze, delay }
    }

    pub fn generated(self, generator: Box<dyn MazeGenerator>) -> Self {
        let maze = DynMazeState::Generated(Generated::new_dyn(generator));
        let Self { maze: _, delay } = self;
        Self { delay, maze }
    }

    pub fn delay_ms(self, delay: u64) -> Self {
        let delay = Duration::from_millis(delay);
        let Self { maze, delay: _ } = self;
        Self { maze, delay }
    }

    pub(crate) fn build(self) -> (Maze, Duration) {
        let maze = self.maze.get().expect("no buildable maze provided");
        let delay = self.delay;
        (maze, delay)
    }
}
