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

    // if line_probability.solve(&line_memo, &line_clue, free) {
    //     println!("{:#?}", line_probability);
    // } else {
    //     println!("muri");
    // }

    // let vec = vec![("a", 10), ("s", 8), ("y", 2)];
    // let mut queue: FxPriorityQueue<&str, usize> = FxPriorityQueue::new_heapify(vec);
    let mut root = LayerSolver::new(None, &resources);
    root.grid.insert(
        PixelId {
            row_index: 0,
            column_index: 0,
        },
        10,
    );
    root.grid.insert(
        PixelId {
            row_index: 1,
            column_index: 0,
        },
        20,
    );
    root.grid.insert(
        PixelId {
            row_index: 2,
            column_index: 0,
        },
        30,
    );
    root.grid.insert(
        PixelId {
            row_index: 3,
            column_index: 0,
        },
        40,
    );
    root.grid.insert(
        PixelId {
            row_index: 4,
            column_index: 0,
        },
        50,
    );

    let mut child1 = LayerSolver::new(Some(&root), &resources);
    child1.grid.insert(
        PixelId {
            row_index: 0,
            column_index: 0,
        },
        20,
    );
    child1.grid.insert(
        PixelId {
            row_index: 0,
            column_index: 1,
        },
        30,
    );
    child1.grid.insert(
        PixelId {
            row_index: 0,
            column_index: 2,
        },
        40,
    );
    child1.grid.insert(
        PixelId {
            row_index: 0,
            column_index: 3,
        },
        50,
    );
    child1.grid.insert(
        PixelId {
            row_index: 0,
            column_index: 4,
        },
        60,
    );

    println!(
        "{}",
        child1.cache_memo(PixelId {
            row_index: 0,
            column_index: 0
        })
    );
    println!(
        "{}",
        child1.cache_memo(PixelId {
            row_index: 1,
            column_index: 0
        })
    );
    println!(
        "{}",
        child1.cache_memo(PixelId {
            row_index: 2,
            column_index: 0
        })
    );
    println!(
        "{}",
        child1.cache_memo(PixelId {
            row_index: 3,
            column_index: 0
        })
    );
    println!(
        "{}",
        child1.cache_memo(PixelId {
            row_index: 5,
            column_index: 0
        })
    );
    println!("{:#?}", child1.grid_cache);

    // for pixel_id in PixelIterator::new(LineId::Row(3), &solve_resources) {
    //     println!("{:?}", pixel_id);
    // }

    // let mut layer_solver = LayerSolver::new(&resources, None, None, true);
    // layer_solver.init()?;
    // layer_solver.show_blank_possibility();
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
        // let parent = self
        //     .parent
        //     .with_context(|| format!("Cannot find pixel memo. pixel_id: {:?}", pixel_id))?;
        // match parent.grid.get(&pixel_id) {
        //     Some(&memo) => Ok(memo),
        //     None => match parent.grid_cache.get(&pixel_id) {
        //         Some(&memo) => Ok(memo),
        //         None => parent.get_ancestral_memo(pixel_id),
        //     },
        // }
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

type Priority = f64;

// #[derive(Debug)]
// pub struct PixelMemo {
//     possibles: FxHashSet<usize>,
// }

// impl PixelMemo {
//     fn new(color_num: usize) -> Self {
//         Self {
//             possibles: (0..color_num).collect(),
//         }
//     }
// }
