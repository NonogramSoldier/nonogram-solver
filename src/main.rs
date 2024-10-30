#![deny(clippy::all)]
#![warn(clippy::pedantic)]

use anyhow::Result;
use cpu_time::ProcessTime;
use nonogram::{solve::solve, Puzzle};
use std::{io::stdin, time::Instant};

mod nonogram;
mod priority_queue;

fn main() -> Result<()> {
    println!("Type a puzzle name.");
    let mut puzzle_name = String::new();
    stdin()
        .read_line(&mut puzzle_name)
        .expect("Faild to read line.");
    let puzzle_name = puzzle_name.trim();

    let puzzle = Puzzle::from_json(puzzle_name)?;

    let cpu_start = ProcessTime::now();
    let start = Instant::now();
    solve(&puzzle);
    let duration = start.elapsed();
    let cpu_duration = cpu_start.elapsed();
    println!("cpu-time: {:?}", cpu_duration);
    println!("    time: {:?}", duration);

    Ok(())
}
