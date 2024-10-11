use num_integer::binomial;

use super::*;


fn calc_free(length: usize, line_clues: &Vec<LineClue>) -> Vec<usize> {
    let mut result: Vec<usize> = Default::default();

    for line_clue in line_clues.iter() {
        let d_num = line_clue.len();

        if d_num == 0 {
            result.push(1);
        } else {
            let mut sep_num = 0;
            let mut sum = line_clue[0].number;
            for i in 1..d_num {
                sum += line_clue[i].number;
                if line_clue[i - 1].color_index == line_clue[i].color_index {
                    sep_num += 1;
                }
            }
            if length < sep_num + sum {
                result.push(0);
            } else {
                result.push(length - sep_num - sum + 1);
            }
        }
    }

    result
}

#[derive(Debug)]
pub struct SolveResources<'a> {
    height: usize,
    width: usize,
    color_num: usize,
    clues: &'a (Vec<LineClue>, Vec<LineClue>),
    free: FxHashMap<LineId, usize>,
}

impl<'a> SolveResources<'a> {
    pub fn new(puzzle: &'a Puzzle) -> Self {
        let height = puzzle.get_height();
        let width = puzzle.get_width();
        let color_num = puzzle.get_color_num();
        let mut free = FxHashMap::default();

        for (index, &value) in calc_free(width, &puzzle.clues.0).iter().enumerate() {
            free.insert(LineId::Row(index), value);
        }

        for (index, &value) in calc_free(height, &puzzle.clues.1).iter().enumerate() {
            free.insert(LineId::Column(index), value);
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

    fn get_line_clue(&self, line_id: LineId) -> &LineClue {
        match line_id {
            LineId::Row(index) => &self.clues.0[index],
            LineId::Column(index) => &self.clues.1[index],
        }
    }

    pub fn get_free(&self, line_id: LineId) -> Result<&usize> {
        self.free
            .get(&line_id)
            .context("this resource does not have the necessary value of free")
    }

    pub fn get_binomial(&self, line_id: LineId) -> Result<u128> {
        let d_num = self.get_line_clue(line_id).len();
        Ok(binomial(
            (self.get_free(line_id)? + d_num - 1) as u128,
            d_num as u128,
        ))
    }

    pub fn show_free(&self) {
        println!("{:#?}", self.free);
    }
}
