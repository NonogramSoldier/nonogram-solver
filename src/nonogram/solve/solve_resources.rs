use super::*;

#[derive(Debug)]
pub struct SolveResources<'a> {
    height: usize,
    width: usize,
    color_num: usize,
    clues: &'a AllClues,
    free: FxHashMap<LineId, usize>,
}

impl<'a> SolveResources<'a> {
    pub fn new(puzzle: &'a Puzzle) -> Self {
        let height = puzzle.get_height();
        let width = puzzle.get_width();
        let color_num = puzzle.get_color_num();
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
            color_num,
            clues: &puzzle.clues,
            free,
        }
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_color_num(&self) -> usize {
        self.color_num
    }

    pub fn get_free(&self, line_id: LineId) -> Result<&usize> {
        self.free
            .get(&line_id)
            .context("this resource does not have the necessary value of free")
    }

    pub fn get_binomial(&self, line_id: LineId) -> u128 {
        self.clues.get_binomial(line_id)
    }

    pub fn show_free(&self) {
        println!("{:?}", self.free);
    }
}
