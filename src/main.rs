#![deny(clippy::all)]
#![warn(clippy::pedantic)]

use std::io::stdin;

use anyhow::Result;
use nonogram::{solve::solve, Puzzle};

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

    // println!("{:#?}", puzzle);

    solve(&puzzle)?;

    Ok(())
}
