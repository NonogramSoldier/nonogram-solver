use num_integer::binomial;
use std::ops::Range;

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
    pub height: usize,
    pub width: usize,
    pub color_num: usize,
    pub uncertain_memo: usize,
    clues: &'a (Vec<LineClue>, Vec<LineClue>),
    free: (Vec<usize>, Vec<usize>),
    range: Vec<Vec<Range<usize>>>,
}

impl<'a> SolveResources<'a> {
    pub fn new(puzzle: &'a Puzzle) -> Self {
        let height = puzzle.get_height();
        let width = puzzle.get_width();
        let color_num = puzzle.get_color_num();
        let uncertain_memo = (1 << color_num) - 1;
        let mut range: Vec<Vec<Range<usize>>> = Default::default();
        let mut start: usize = 0;
        for _ in 0..width {
            let mut row_range: Vec<Range<usize>> = Default::default();
            for _ in 0..height {
                row_range.push(start..start + color_num);
                start += color_num;
            }
            range.push(row_range);
        }

        Self {
            height,
            width,
            color_num,
            uncertain_memo,
            clues: &puzzle.clues,
            free: (
                calc_free(width, &puzzle.clues.0),
                calc_free(height, &puzzle.clues.1),
            ),
            range,
        }
    }

    pub fn get_length(&self, line_id: LineId) -> usize {
        match line_id {
            LineId::Row(_) => self.width,
            LineId::Column(_) => self.height,
        }
    }

    pub fn get_uncertain_memo(&self) -> usize {
        self.uncertain_memo
    }

    pub fn get_line_clue(&self, line_id: LineId) -> &LineClue {
        match line_id {
            LineId::Row(index) => &self.clues.0[index],
            LineId::Column(index) => &self.clues.1[index],
        }
    }

    pub fn get_free(&self, line_id: LineId) -> usize {
        match line_id {
            LineId::Row(index) => self.free.0[index],
            LineId::Column(index) => self.free.1[index],
        }
        // self.free
        //     .get(&line_id)
        //     .context("resource does not have the necessary value of free")
    }

    pub fn get_range(&self, pixel_id: PixelId) -> Range<usize> {
        self.range[pixel_id.row_index][pixel_id.column_index].clone()
    }

    pub fn get_binomial(&self, line_id: LineId) -> u128 {
        let d_num = self.get_line_clue(line_id).len();
        binomial((self.get_free(line_id) + d_num - 1) as u128, d_num as u128)
    }

    // pub fn show_free(&self) {
    //     println!("{:#?}", self.free);
    // }
}
