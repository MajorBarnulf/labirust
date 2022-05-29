use std::collections::{HashSet, VecDeque};

use crate::{
    algorithm::{Context, Guess, Insight},
    Algorithm, Pos,
};

/// [`Algorithm`] traversing the [`crate::Maze`] as a common graph.
/// Storing each possible paths form shortest to longest and extending the shortest ones first.
/// Most effective when the resolution is among the shortest possible paths.
pub struct BreathFirst {
    paths: VecDeque<Vec<Pos>>,
    visited: HashSet<Pos>,
    last_path: Vec<Pos>,
}

impl BreathFirst {
    /// Constructor.
    pub fn new() -> Self {
        Self {
            paths: VecDeque::new(),
            visited: HashSet::new(),
            last_path: Vec::new(),
        }
    }
}

impl Algorithm for BreathFirst {
    fn progress(&mut self, insight: &Insight, ctx: &mut Context) -> Guess {
        self.visited.insert(insight.position());
        let path = self.last_path.clone();
        for &branch in insight.paths() {
            if self.visited.contains(&branch) {
                continue;
            }
            let mut new_path = path.clone();
            new_path.push(branch);
            self.paths.push_back(new_path);
        }

        let path = self.paths.pop_front().expect("no more options");
        self.last_path = path.clone();
        ctx.guess(path)
    }
}
