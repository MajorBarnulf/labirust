use std::collections::HashSet;

use crate::{Algorithm, Context, Guess, Insight, Pos};

/// Frame of the stack used by a [`DepthFirst`] to retain its path and possible branches.
pub struct Frame {
    position: Pos,
    remaining_branches: Vec<Pos>,
}

/// [`Algorithm`] driving the resolution of a [`crate::Maze`] traversing it as a common graph in a depth-first fashion.
/// Stores the current path and possible branches in a stack.
pub struct DepthFirst {
    visited: HashSet<Pos>,
    stack: Vec<Frame>,
}

impl DepthFirst {
    /// constructor.
    pub fn new() -> Self {
        Self {
            visited: HashSet::new(),
            stack: Vec::new(),
        }
    }
}

impl Algorithm for DepthFirst {
    fn progress(&mut self, insight: &Insight, ctx: &mut Context) -> Guess {
        let position = insight.position();
        let branches = insight.paths().iter().cloned().collect();

        self.visited.insert(position);
        self.stack.push(Frame {
            position,
            remaining_branches: branches,
        });

        loop {
            let last = self.stack.last_mut().expect("no more options");
            if let Some(branch) = last.remaining_branches.pop() {
                if !self.visited.contains(&branch) {
                    let mut path: Vec<_> = self.stack.iter().map(|f| f.position).collect();
                    path.push(branch);
                    return ctx.guess(path);
                }
            } else {
                self.stack.pop();
            }
        }
    }
}
