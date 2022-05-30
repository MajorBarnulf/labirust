use std::str::FromStr;

use clap::Parser;
use labirust::{implementations::*, Executor};

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
    algorithm_kind: Algorithms,
    width: usize,
    height: usize,
    delay: usize,
}

fn main() {
    let params = Parameters::parse();

    let executor = todo!();

    println!("Hello, world!");
}
