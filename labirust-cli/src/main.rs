use std::str::FromStr;

use clap::Parser;
use labirust::{implementations::*, Algorithm, Executor, SimpleGenerator};

enum Algorithms {
    DepthFirst,
    BreathFirst,
}

impl FromStr for Algorithms {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "depth-first" => Ok(Self::DepthFirst),
            "breath-first" => Ok(Self::BreathFirst),
            _ => Err("No right pattern".into()),
        }
    }
}

#[derive(Parser)]
struct Parameters {
    /// Algorithm to use in the simulation.
    /// One of: "depth-first", "breath-first"
    algorithm: Algorithms,

    /// Width of the maze to solve.
    #[clap(short, default_value_t = 40)]
    width: usize,

    /// Height of the maze to solve.
    #[clap(short, default_value_t = 20)]
    height: usize,

    /// Delay between two simulation ticks.
    #[clap(short, default_value_t = 100)]
    delay: usize,
}

fn main() {
    let params = Parameters::parse();

    let algorithm: Box<dyn Algorithm> = match params.algorithm {
        Algorithms::DepthFirst => Box::new(DepthFirst::new()),
        Algorithms::BreathFirst => Box::new(BreathFirst::new()),
    };

    let mut executor = Executor::build_dyn(algorithm, |b| {
        b.generated(Box::new(SimpleGenerator::new(
            params.width as isize,
            params.height as isize,
        )))
    });

    executor.run();
}
