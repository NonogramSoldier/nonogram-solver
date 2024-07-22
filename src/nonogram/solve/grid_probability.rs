use super::*;

#[derive(Debug)]
pub struct GridProbability {
    row: Vec<LineProbability>,
    column: Vec<LineProbability>,
}

impl GridProbability {
    pub fn new(height: usize, width: usize, paint_num: usize) -> Self {
        Self {
            row: vec![LineProbability::new(width, paint_num); height],
            column: vec![LineProbability::new(height, paint_num); width],
        }
    }

    fn line_solve(
        &mut self,
        line_id: LineId,
        line_memo: Vec<PixelMemo>,
        line_clue: Vec<Description>,
    ) -> bool {
        todo!()
    }
}

#[derive(Debug, Clone)]
struct LineProbability {
    color_cases: Vec<ColorCases>,
    painting_count: u128,
}

impl LineProbability {
    fn new(length: usize, color_num: usize) -> Self {
        Self {
            color_cases: vec![ColorCases::new(color_num); length],
            painting_count: 0,
        }
    }

    fn solve(
        &mut self,
        Line_memo: Vec<PixelMemo>,
        line_clue: Vec<Description>,
        free: usize,
    ) -> bool {
        todo!()
    }
}

#[derive(Debug, Clone)]
struct ColorCases {
    blank_num: usize,
    paint_nums: Vec<usize>,
}

impl ColorCases {
    fn new(color_num: usize) -> Self {
        Self {
            blank_num: 0,
            paint_nums: vec![0; color_num],
        }
    }
}
