pub mod solve;

use anyhow::{Context, Result};
use serde::Deserialize;
use serde_tuple::Deserialize_tuple;
use std::fs::File;
use std::io::BufReader;

#[derive(Debug, Deserialize)]
pub struct Puzzle {
    colors: Vec<String>,
    clues: (Vec<LineClue>, Vec<LineClue>),
}

impl Puzzle {
    pub fn from_json(puzzle_name: &str) -> Result<Self> {
        let path = &format!("puzzles/{}.json", puzzle_name);
        let file = File::open(path).with_context(|| format!("Cannot open the path {}", path))?;

        let reader = BufReader::new(file);

        serde_json::from_reader(reader).context("The JSON file has an unexpected structure")
    }
}

type LineClue = Vec<Description>;

#[derive(Debug, Deserialize_tuple)]
struct Description {
    color_index: usize,
    number: usize,
}
