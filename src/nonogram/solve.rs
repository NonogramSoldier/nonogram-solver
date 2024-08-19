mod grid_probability;
mod layer;
mod solve_resources;

use std::{cell::RefCell, rc::Rc};

use fxhash::{FxHashMap, FxHashSet};
use grid_probability::{GridProbability, LineProbability};
use layer::LayerRef;
use num_integer::binomial;
use solve_resources::SolveResources;

use super::*;

pub fn solve(puzzle: &Puzzle) -> bool {
    // let resources = SolveResources::from_puzzle(puzzle);
    // resources.show_free();

    let mut line_probability = LineProbability::new(10, 2);
    let mut line_memo = vec![PixelMemo::new(2); 10];

    line_memo[5].possibles.remove(&0);
    line_memo[7].possibles.remove(&0);

    let line_clue = LineClue {
        descriptions: vec![
            Description {
                color_index: 1,
                number: 1,
            },
            Description {
                color_index: 1,
                number: 3,
            },
            Description {
                color_index: 1,
                number: 2,
            },
        ],
    };

    let free = line_clue.get_free(10);
    if line_probability.solve(line_memo, line_clue.descriptions, free) {
        println!("{:#?}", line_probability);
    } else {
        println!("muri");
    }

    // for pixel_id in PixelIterator::new(LineId::Row(3), &solve_resources) {
    //     println!("{:?}", pixel_id);
    // }

    // let mut layer_solver = LayerSolver::new(&resources, None, None, true);
    // layer_solver.init()?;
    // layer_solver.show_blank_possibility();
    true
}

impl Puzzle {
    fn get_height(&self) -> usize {
        self.clues.get_height()
    }

    fn get_width(&self) -> usize {
        self.clues.get_width()
    }

    fn get_color_num(&self) -> usize {
        self.colors.len()
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
    grid_probability: GridProbability<'a>,
    is_base_layer: bool,
}

impl<'a> LayerSolver<'a> {
    fn new(
        resources: &'a SolveResources,
        layer_parent: Option<LayerRef>,
        probability_parent: Option<&'a GridProbability<'a>>,
        is_base_layer: bool,
    ) -> Self {
        Self {
            resources,
            layer: LayerRef::new(layer_parent),
            grid_probability: GridProbability::new(probability_parent),
            is_base_layer,
        }
    }

    // fn init(&mut self) -> Result<()> {
    //     let mut vec: Vec<(LineId, Reverse<u128>)> = Vec::new();
    //     for i in 0..self.resources.get_height() {
    //         let line_id = LineId::Row(i);
    //         vec.push((line_id, Reverse(self.resources.get_binomial(line_id))))
    //     }
    //     for i in 0..self.resources.get_width() {
    //         let line_id = LineId::Column(i);
    //         vec.push((line_id, Reverse(self.resources.get_binomial(line_id))))
    //     }

    //     let mut priority_queue = FxPriorityQueue::new_heapify(vec);

    //     loop {
    //         if let Some(value) = priority_queue.pop() {
    //             println!("{:?}", value);
    //             if !(self.line_solve(value.0)?) {
    //                 bail!("initial line solve returns false");
    //             }
    //             self.update_layer(value.0);
    //         } else {
    //             return Ok(());
    //         }
    //     }
    // }

    // fn solve() -> bool {
    //     todo!()
    // }

    // fn line_solve(&mut self, line_id: LineId) -> Result<bool> {
    //     self.grid_probability.line_solve(
    //         line_id,
    //         self.layer.get_line_memo(line_id, self.resources),
    //         self.resources.get_clue(line_id),
    //         self.resources,
    //     )
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
    fn new(line_id: LineId, resources: &SolveResources) -> Self {
        Self {
            current: 0,
            end: match line_id {
                LineId::Row(_) => resources.get_width(),
                LineId::Column(_) => resources.get_height(),
            },
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

type Priority = f64;

#[derive(Debug, Clone)]
pub struct PixelMemo {
    possibles: FxHashSet<usize>,
}

impl PixelMemo {
    fn new(color_num: usize) -> Self {
        Self {
            possibles: (0..color_num).collect()
        }
    }
}
