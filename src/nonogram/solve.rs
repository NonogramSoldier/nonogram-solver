mod line_probability;
mod solve_resources;

use crate::priority_queue::FxPriorityQueue;
use anyhow::Ok;
use fxhash::FxHashMap;
use line_probability::LineProbability;
use solve_resources::SolveResources;

use super::*;

pub fn solve(puzzle: &Puzzle) -> Result<bool> {
    let resources = SolveResources::new(puzzle);
    // resources.show_free();
    // for index in 0..resources.get_height() {
    //     println!(
    //         "row({}): {}",
    //         index,
    //         resources.get_binomial(LineId::Row(index))?
    //     );
    // }

    // let length = 15;
    // let color_num = 2;

    // let mut line_probability = LineProbability::new(length, color_num);
    // let mut line_memo = vec![(1 << color_num) - 1; length];

    // line_memo[7] -= 1;
    // line_memo[8] -= 1;
    // line_memo[10] -= 1;
    // line_memo[11] -= 1;

    // println!("{:?}", line_memo);

    // // let line_clue: LineClue = vec![(1, 2), (1, 1), (1, 5)];
    // let line_clue: LineClue = vec![
    //     Description {
    //         color_index: 1,
    //         number: 2,
    //     },
    //     Description {
    //         color_index: 1,
    //         number: 1,
    //     },
    //     Description {
    //         color_index: 1,
    //         number: 5,
    //     },
    // ];

    // let free = {
    //     let d_num = line_clue.len();

    //     if d_num == 0 {
    //         1
    //     } else {
    //         let mut sep_num = 0;
    //         let mut sum = line_clue[0].number;
    //         for i in 1..d_num {
    //             sum += line_clue[i].number;
    //             if line_clue[i - 1].color_index == line_clue[i].color_index {
    //                 sep_num += 1;
    //             }
    //         }
    //         if length < sep_num + sum {
    //             0
    //         } else {
    //             length - sep_num - sum + 1
    //         }
    //     }
    // };

    // for pixel_id in PixelIterator::new(LineId::Row(3), &solve_resources) {
    //     println!("{:?}", pixel_id);
    // }

    let mut layer_solver = LayerSolver::new(None, &resources);
    let mut priority_queue = layer_solver.init()?.unwrap();

    loop {
        match priority_queue.pop() {
            Some(value) => println!("{:?}", value),
            None => break,
        }
    }

    Ok(true)
}

impl Puzzle {
    fn get_height(&self) -> usize {
        self.clues.0.len()
    }

    fn get_width(&self) -> usize {
        self.clues.1.len()
    }

    fn get_color_num(&self) -> usize {
        self.colors.len()
    }
}

#[derive(Debug)]
pub struct LayerSolver<'a> {
    resources: &'a SolveResources<'a>,
    parent: Option<&'a LayerSolver<'a>>,
    grid: FxHashMap<PixelId, usize>,
    grid_cache: FxHashMap<PixelId, usize>,
    line_probabilities: FxHashMap<LineId, LineProbability>,
    line_cache: FxHashMap<LineId, &'a LineProbability>,
}

impl<'a> LayerSolver<'a> {
    fn new(parent: Option<&'a LayerSolver<'a>>, resources: &'a SolveResources<'a>) -> Self {
        Self {
            resources,
            parent,
            grid: Default::default(),
            grid_cache: Default::default(),
            line_probabilities: Default::default(),
            line_cache: Default::default(),
        }
    }

    fn cache_memo(&mut self, pixel_id: PixelId) -> usize {
        match self.grid.get(&pixel_id) {
            Some(&memo) => memo,
            None => match self.grid_cache.get(&pixel_id) {
                Some(&memo) => memo,
                None => {
                    let memo = self.get_ancestral_memo(pixel_id);
                    self.grid_cache.insert(pixel_id, memo);
                    memo
                }
            },
        }
    }

    fn get_ancestral_memo(&self, pixel_id: PixelId) -> usize {
        match self.parent {
            Some(parent) => match parent.grid.get(&pixel_id) {
                Some(&memo) => memo,
                None => match parent.grid_cache.get(&pixel_id) {
                    Some(&memo) => memo,
                    None => parent.get_ancestral_memo(pixel_id),
                },
            },
            None => self.resources.get_uncertain_memo(),
        }
    }

    fn init(&mut self) -> Result<Option<FxPriorityQueue<LineId, Priority>>> {
        let mut vec: Vec<(LineId, u128)> = Vec::new();
        for i in 0..self.resources.get_height() {
            let line_id = LineId::Row(i);
            vec.push((line_id, self.resources.get_binomial(line_id)?));
        }
        for i in 0..self.resources.get_width() {
            let line_id = LineId::Column(i);
            vec.push((line_id, self.resources.get_binomial(line_id)?));
        }

        let mut priority_queue = FxPriorityQueue::new_heapify(vec);
        let mut result: FxPriorityQueue<LineId, Priority> = FxPriorityQueue::new();

        loop {
            match priority_queue.pop() {
                Some(value) => {
                    // println!("{:?}", value);
                    if !self.line_solve(value.0, &mut result)? {
                        return Ok(None);
                    }
                }
                None => {
                    // println!("{:#?}", self.line_probabilities);
                    return Ok(Some(result));
                }
            }
        }
    }

    fn line_solve(
        &mut self,
        line_id: LineId,
        priority_queue: &mut FxPriorityQueue<LineId, Priority>,
    ) -> Result<bool> {
        let mut line_memo: Vec<usize> = Vec::new();
        for pixel_id in PixelIterator::new(line_id, self.resources.get_length(line_id)) {
            line_memo.push(self.cache_memo(pixel_id));
        }

        if !self
            .line_probabilities
            .entry(line_id)
            .or_insert_with(|| {
                LineProbability::new(
                    self.resources.get_length(line_id),
                    self.resources.get_color_num(),
                )
            })
            .solve(
                &line_memo,
                self.resources.get_line_clue(line_id),
                *self.resources.get_free(line_id)?,
            )
        {
            return Ok(false);
        }
        for (pixel_index, &pixel_memo) in line_memo.iter().enumerate() {
            let mut new_impossible_colors = 0;
            let line_probability = self.line_probabilities.get(&line_id).unwrap();
            for color_index in ColorIterator::new(pixel_memo) {
                if line_probability.get_color_case(pixel_index, color_index) == 0 {
                    new_impossible_colors ^= 1 << color_index;
                }
            }

            if new_impossible_colors != 0 {
                let new_possible_colors = pixel_memo ^ new_impossible_colors;
                self.grid
                    .insert(line_id.to_pixel_id(pixel_index), new_possible_colors);
                let (oppo_line, oppo_index) = line_id.opposite(pixel_index);
                if let Some(line) = self.line_probabilities.get(&oppo_line) {
                    let mut possible_num = 0;
                    let mut impossible_num = 0;
                    for new_possible_color in ColorIterator::new(new_possible_colors) {
                        possible_num += line.get_color_case(oppo_index, new_possible_color)
                    }
                    for new_impossible_color in ColorIterator::new(new_impossible_colors) {
                        impossible_num += line.get_color_case(oppo_index, new_impossible_color);
                    }
                    priority_queue.add_or_insert(
                        oppo_line,
                        (possible_num as f64 / (possible_num + impossible_num) as f64).ln(),
                    );
                }
            }
        }

        Ok(true)
    }

    // fn solve() -> bool {
    //     todo!()
    // }

    // fn show_blank_possibility(&self) {
    //     for row_index in 0..self.resources.get_height() {
    //         for pixel in self
    //             .layer
    //             .get_line_memo(LineId::Row(row_index), self.resources)
    //             .iter()
    //         {
    //             if pixel.contains(&0) {
    //                 print!("  ");
    //             } else {
    //                 print!("$$");
    //             }
    //             // if let Some(pixel_memo) = pixel {
    //             //     if pixel_memo.blank_possibility == Possibility::Impossible {
    //             //         print!("$$");
    //             //         continue;
    //             //     }
    //             // }
    //         }
    //         println!("");
    //     }
    // }

    // fn update_layer(&mut self, line_id: LineId) -> Result<Vec<HashSet<usize>>> {
    //     let mut vec: Vec<HashSet<usize>> = Default::default();
    //     for (index, color_case) in self
    //         .grid_probability
    //         .get_color_cases(line_id)?
    //         .iter()
    //         .enumerate()
    //     {
    //         vec.push(Default::default());
    //         for (color_index, &case) in color_case.iter().enumerate() {
    //             if case == 0 {
    //                 if self.layer.set_pixel_memo(line_id.to_pixel_id(index), 0) {
    //                     vec[index].insert(color_index);
    //                 }
    //             }
    //         }
    //         // if color_case.blank_num == 0 {
    //         //     if self.layer.set_pixel_memo(line_id.to_pixel_id(index), 0) {
    //         //         vec[index].insert(0);
    //         //     }
    //         // }

    //     }
    //     Ok(vec)

    //     // for color_case in self.grid_probability.get_color_cases(line_id)?.iter() {
    //     //     if color_case.blank_num == 0 {
    //     //         self.layer.
    //     //     }
    //     // }
    // }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum LineId {
    Row(usize),
    Column(usize),
}

impl LineId {
    fn to_pixel_id(&self, index: usize) -> PixelId {
        match *self {
            LineId::Row(row_index) => PixelId {
                row_index,
                column_index: index,
            },
            LineId::Column(column_index) => PixelId {
                row_index: index,
                column_index,
            },
        }
    }

    fn opposite(&self, pixel_index: usize) -> (LineId, usize) {
        match *self {
            LineId::Row(row_index) => (LineId::Column(pixel_index), row_index),
            LineId::Column(column_index) => (LineId::Row(pixel_index), column_index),
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct PixelId {
    row_index: usize,
    column_index: usize,
}

struct PixelIterator {
    current: usize,
    end: usize,
    line_id: LineId,
}

impl PixelIterator {
    fn new(line_id: LineId, length: usize) -> Self {
        Self {
            current: 0,
            end: length,
            line_id,
        }
    }
}

impl Iterator for PixelIterator {
    type Item = PixelId;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.end {
            let result = match self.line_id {
                LineId::Row(row_index) => PixelId {
                    row_index,
                    column_index: self.current,
                },
                LineId::Column(column_index) => PixelId {
                    row_index: self.current,
                    column_index,
                },
            };
            self.current += 1;
            Some(result)
        } else {
            None
        }
    }
}

struct ColorIterator {
    current_pixel_memo: usize,
}

impl ColorIterator {
    fn new(pixel_memo: usize) -> Self {
        Self {
            current_pixel_memo: pixel_memo,
        }
    }
}

impl Iterator for ColorIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_pixel_memo != 0 {
            let index = self.current_pixel_memo.trailing_zeros();
            self.current_pixel_memo ^= 1 << index;
            Some(index as usize)
        } else {
            None
        }
    }
}

type Priority = f64;

// FULLY_SOLVED: 完全に解かれた
// PARTIALLY_SOLVED: 部分的に解かれた
// CONFLICT: 矛盾が見つかった
