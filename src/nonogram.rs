use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;

#[derive(Deserialize, Debug)]
pub struct Puzzle {
    initial_color: Color,
    paint_colors: Vec<Color>,
    clues: Clues,
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

#[derive(Deserialize, Debug)]
struct Color {
    name: String,
}

#[derive(Deserialize, Debug)]
struct Clues {
    row: Vec<Vec<Description>>,
    column: Vec<Vec<Description>>,
}

#[derive(Deserialize, Debug)]
struct Description {
    color_index: usize,
    number: usize,
}
