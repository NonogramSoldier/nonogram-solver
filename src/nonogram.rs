pub mod solve;

use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;

#[derive(Debug, Deserialize)]
pub struct Puzzle {
    initial_color: Color,
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

    pub fn get_row_length(&self) -> usize {
        self.clues.get_row_length()
    }

    pub fn get_column_length(&self) -> usize {
        self.clues.get_column_length()
    }

    pub fn get_color_num(&self) -> usize {
        self.paint_colors.len()
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

impl AllClues {
    fn get_row_length(&self) -> usize {
        self.column.len()
    }

    fn get_column_length(&self) -> usize {
        self.row.len()
    }
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
