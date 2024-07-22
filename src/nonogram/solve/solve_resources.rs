use super::*;

#[derive(Debug)]
pub struct SolveResources<'a> {
    pub height: usize,
    pub width: usize,
    pub paint_num: usize,
    clues: &'a AllClues,
    free: FxHashMap<LineId, usize>,
}

impl<'a> SolveResources<'a> {
    pub fn from_puzzle(puzzle: &'a Puzzle) -> Self {
        let height = puzzle.get_height();
        let width = puzzle.get_width();
        let paint_num = puzzle.get_paint_num();
        let mut free = FxHashMap::default();

        for (index, line_clue) in puzzle.clues.row.iter().enumerate() {
            free.insert(LineId::Row(index), line_clue.get_free(width));
        }

        for (index, line_clue) in puzzle.clues.column.iter().enumerate() {
            free.insert(LineId::Column(index), line_clue.get_free(height));
        }

        Self {
            height,
            width,
            paint_num,
            clues: &puzzle.clues,
            free,
        }
    }

    pub fn get_binomial(&self, line_id: LineId) -> u128 {
        self.clues.get_binomial(line_id)
    }

    pub fn show_free(&self) {
        println!("{:?}", self.free);
    }
}
