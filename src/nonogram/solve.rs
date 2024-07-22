mod layer;
mod grid_probability;
mod solve_resources;

use std::{cell::RefCell, cmp::Reverse, rc::Rc};

use fxhash::FxHashMap;
use grid_probability::GridProbability;
use layer::{LayerRef, PixelMemo};
use num_integer::binomial;
use solve_resources::SolveResources;

use super::*;
use crate::priority_queue::FxPriorityQueue;

pub fn solve(puzzle: &Puzzle) {
    
}

impl Puzzle {
    fn get_height(&self) -> usize {
        self.clues.get_height()
    }

    fn get_width(&self) -> usize {
        self.clues.get_width()
    }

    fn get_paint_num(&self) -> usize {
        self.paint_colors.len()
    }
}

impl AllClues {
    fn get_height(&self) -> usize {
        self.row.len()
    }

    fn get_width(&self) -> usize {
        self.column.len()
    }

    fn get_binomial(&self, line_id: LineId) -> u128 {
        match line_id {
            LineId::Row(i) => self.row[i].get_binomial(self.get_width()),
            LineId::Column(i) => self.column[i].get_binomial(self.get_height()),
        }
    }
}

impl LineClue {
    fn get_binomial(&self, length: usize) -> u128 {
        let d_num = self.descriptions.len();

        if d_num == 0 {
            1
        } else {
            let mut sep_num = 0;
            let mut sum = self.descriptions[0].number;
            for i in 1..d_num {
                sum += self.descriptions[i].number;
                if self.descriptions[i - 1].color_index == self.descriptions[i].color_index {
                    sep_num += 1;
                }
            }

            if length < sep_num + sum {
                0
            } else {
                binomial((length - sep_num - sum + d_num) as u128, d_num as u128)
            }
        }
    }

    fn get_free(&self, length: usize) -> usize {
        let d_num = self.descriptions.len();

        if d_num == 0 {
            1
        } else {
            let mut sep_num = 0;
            let mut sum = self.descriptions[0].number;
            for i in 1..d_num {
                sum += self.descriptions[i].number;
                if self.descriptions[i - 1].color_index == self.descriptions[i].color_index {
                    sep_num += 1;
                }
            }
            if length < sep_num + sum {
                0
            } else {
                length - sep_num - sum + 1
            }
        }
    }
}

#[derive(Debug)]
pub struct LayerSolver<'a> {
    resources: &'a SolveResources<'a>,
    layer: LayerRef,
    grid_probability: GridProbability,
    is_base_layer: bool,
}

impl<'a> LayerSolver<'a> {
    pub fn new(resources: &'a SolveResources, is_base_layer: bool) -> Self {
        Self {
            resources,
            layer: LayerRef::new(None),
            grid_probability: GridProbability::new(
                resources.height,
                resources.width,
                resources.paint_num,
            ),
            is_base_layer,
        }
    }

    pub fn init(&mut self) {
        let mut vec: Vec<(LineId, Reverse<u128>)> = Vec::new();
        for i in 0..self.resources.height {
            let line_id = LineId::Row(i);
            vec.push((line_id, Reverse(self.resources.get_binomial(line_id))))
        }
        for i in 0..self.resources.width {
            let line_id = LineId::Column(i);
            vec.push((line_id, Reverse(self.resources.get_binomial(line_id))))
        }

        let mut priority_queue = FxPriorityQueue::new_heapify(vec);

        loop {
            if let Some(value) = priority_queue.pop() {
                // println!("{:?}", value);
                // self.line_solve(value.0);
            } else {
                break;
            }
        }
    }

    // fn line_solve(&mut self, line_id: LineId) -> bool {
    //     self.grid_probability.line_solve(self.layer.line(line_id), self.puzzle.clues.get_line(line_id));
    // }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum LineId {
    Row(usize),
    Column(usize),
}

#[derive(Debug, Hash)]
struct PixelId {
    row_index: usize,
    column_index: usize,
}

type Priority = f64;
