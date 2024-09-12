pub mod solve;

use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;

#[derive(Debug, Deserialize)]
pub struct Puzzle {
    colors: Vec<String>,
    clues: (Vec<LineClue>, Vec<LineClue>),
}

impl Puzzle {
    pub fn new(puzzle_name: &str) -> Self {
        let path = &format!("puzzles/{}.json", puzzle_name);
        let file = File::open(path).expect(&format!("Cannot open the path {}", path));

        let reader = BufReader::new(file);

        let puzzle: Puzzle =
            serde_json::from_reader(reader).expect("The JSON file has an unexpected structure.");

        puzzle
    }
}

type LineClue = Vec<(usize, usize)>;
