pub mod solve;

use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;

#[derive(Debug, Deserialize)]
pub struct Puzzle {
    background_color: Color,
    paint_colors: Vec<Color>,
    clues: AllClues,
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

#[derive(Debug, Deserialize)]
struct Color {
    name: String,
}

#[derive(Debug, Deserialize)]
struct AllClues {
    row: Vec<LineClue>,
    column: Vec<LineClue>,
}

#[derive(Debug, Deserialize)]
struct LineClue {
    descriptions: Vec<Description>,
}

#[derive(Debug, Deserialize)]
struct Description {
    color_index: usize,
    number: usize,
}
