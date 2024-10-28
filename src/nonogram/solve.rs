mod line_probability;
mod solve_resources;

use crate::priority_queue::FxPriorityQueue;
use anyhow::Ok;
use bitvec::prelude::*;
use fxhash::FxHashMap;
use line_probability::LineProbability;
use solve_resources::SolveResources;

use super::*;

pub fn solve(puzzle: &Puzzle) -> Result<bool> {
    let resources = SolveResources::new(puzzle);

    let mut backtracks = 0;
    let mut nlines = 0;
    let mut layer_solver = LayerSolver::new(&resources, None);
    let priority_queue = layer_solver.init(&mut nlines).unwrap();
    layer_solver.solve(priority_queue, &mut backtracks, &mut nlines);
    layer_solver.show_blank_possibility();
    println!("line_solves: {}", nlines);
    println!("backtracks:  {}", backtracks);

    Ok(true)
}

fn calc_priority(
    line_probability: &LineProbability,
    index: usize,
    new_possible_colors: &BitVec,
    new_impossible_colors: &BitVec,
) -> f64 {
    let mut possible_num = 0;
    for new_possible_color in new_possible_colors.iter_ones() {
        possible_num += line_probability.get_color_case(index, new_possible_color);
    }

    let mut impossible_num = 0;
    for new_impossible_color in new_impossible_colors.iter_ones() {
        impossible_num += line_probability.get_color_case(index, new_impossible_color);
    }

    (possible_num as f64 / (possible_num + impossible_num) as f64).ln()
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
    grid: BitVec,
    line_probabilities: FxHashMap<LineId, LineProbability>,
    line_cache: FxHashMap<LineId, &'a LineProbability>,
}

impl<'a> LayerSolver<'a> {
    fn new(resources: &'a SolveResources<'a>, parent: Option<&'a LayerSolver<'a>>) -> Self {
        let (grid, line_cache) = match parent {
            Some(parent) => {
                let mut line_cache = parent.line_cache.clone();
                for (&line_id, line_probability) in parent.line_probabilities.iter() {
                    line_cache.insert(line_id, line_probability);
                }
                (parent.grid.clone(), line_cache)
            }
            None => (
                bitvec![1; resources.height * resources.width * resources.color_num],
                Default::default(),
            ),
        };
        Self {
            resources,
            grid,
            line_probabilities: Default::default(),
            line_cache,
        }
    }

    fn get_memo(&self, pixel_id: PixelId) -> &BitSlice {
        &self.grid[self.resources.get_range(pixel_id)]
    }

    fn get_line(&self, line_id: LineId) -> Option<&LineProbability> {
        self.line_probabilities
            .get(&line_id)
            .or_else(|| self.line_cache.get(&line_id).copied())
    }

    fn set_pixel_memo(
        &mut self,
        pixel_id: PixelId,
        new_possible_colors: &BitVec,
        new_impossible_colors: &BitVec,
    ) -> FxPriorityQueue<LineId, f64> {
        self.grid[self.resources.get_range(pixel_id)].clone_from_bitslice(new_possible_colors);
        let mut vec: Vec<(LineId, f64)> = Default::default();
        vec.push((
            LineId::Row(pixel_id.row_index),
            calc_priority(
                self.get_line(LineId::Row(pixel_id.row_index)).unwrap(),
                pixel_id.column_index,
                new_possible_colors,
                new_impossible_colors,
            ),
        ));
        vec.push((
            LineId::Column(pixel_id.column_index),
            calc_priority(
                self.get_line(LineId::Column(pixel_id.column_index))
                    .unwrap(),
                pixel_id.row_index,
                new_possible_colors,
                new_impossible_colors,
            ),
        ));
        FxPriorityQueue::new_heapify(vec)
    }

    fn init(&mut self, nlines: &mut u128) -> Option<FxPriorityQueue<LineId, f64>> {
        let mut vec: Vec<(LineId, u128)> = Vec::new();
        for i in 0..self.resources.height {
            let line_id = LineId::Row(i);
            vec.push((line_id, self.resources.get_binomial(line_id)));
        }
        for i in 0..self.resources.width {
            let line_id = LineId::Column(i);
            vec.push((line_id, self.resources.get_binomial(line_id)));
        }

        let mut priority_queue = FxPriorityQueue::new_heapify(vec);
        let mut result: FxPriorityQueue<LineId, f64> = FxPriorityQueue::new();

        loop {
            match priority_queue.pop() {
                Some(value) => {
                    *nlines += 1;
                    if !self.line_solve(value.0, &mut result) {
                        break None;
                    }
                }
                None => break Some(result),
            }
        }
    }

    fn line_solve(
        &mut self,
        line_id: LineId,
        priority_queue: &mut FxPriorityQueue<LineId, f64>,
    ) -> bool {
        if !self
            .line_probabilities
            .entry(line_id)
            .or_insert_with(|| {
                LineProbability::new(self.resources.get_length(line_id), self.resources.color_num)
            })
            .solve(&self.grid, line_id, self.resources)
        {
            return false;
        }

        for (pixel_index, pixel_id) in
            PixelIterator::new(line_id, self.resources.get_length(line_id)).enumerate()
        {
            let mut new_impossible_colors = bitvec![0; self.resources.color_num];
            let line_probability = self.line_probabilities.get(&line_id).unwrap();
            let pixel_memo = &self.grid[self.resources.get_range(pixel_id)];
            for color_index in pixel_memo.iter_ones() {
                if line_probability.get_color_case(pixel_index, color_index) == 0 {
                    new_impossible_colors.set(color_index, true);
                }
            }

            if new_impossible_colors.any() {
                let new_possible_colors = pixel_memo.to_bitvec() ^ &new_impossible_colors;
                self.grid[self.resources.get_range(pixel_id)]
                    .copy_from_bitslice(&new_possible_colors);
                let (oppo_line, oppo_index) = line_id.opposite(pixel_index);
                if let Some(line) = self.get_line(oppo_line) {
                    priority_queue.add_or_insert(
                        oppo_line,
                        calc_priority(
                            line,
                            oppo_index,
                            &new_possible_colors,
                            &new_impossible_colors,
                        ),
                    );
                }
            }
        }

        true
    }

    fn solve(
        &mut self,
        mut priority_queue: FxPriorityQueue<LineId, f64>,
        backtracks: &mut u128,
        nlines: &mut u128,
    ) {
        loop {
            if let Some((line_id, _)) = priority_queue.pop() {
                *nlines += 1;
                if !self.line_solve(line_id, &mut priority_queue) {
                    self.grid.fill(false);
                    return;
                }
            } else {
                break;
            }
        }

        if !(self.grid.count_ones() == self.resources.height * self.resources.width) {
            let mut min_value: Option<(PixelId, usize, f64)> = None;
            for row_index in 0..self.resources.height {
                for pixel_id in PixelIterator::new(LineId::Row(row_index), self.resources.width) {
                    let pixel_memo = self.get_memo(pixel_id);
                    match pixel_memo.count_ones() {
                        ..=1 => continue,
                        2 => {
                            let color_index = pixel_memo.trailing_zeros() as usize;
                            let value = self.calc_value(pixel_id, color_index);
                            match min_value {
                                Some(tuple) => {
                                    if value < tuple.2 {
                                        min_value = Some((pixel_id, color_index, value))
                                    }
                                }
                                None => min_value = Some((pixel_id, color_index, value)),
                            }
                        }
                        2.. => {
                            for color_index in pixel_memo.iter_ones() {
                                let value = self.calc_value(pixel_id, color_index);
                                match min_value {
                                    Some(tuple) => {
                                        if value < tuple.2 {
                                            min_value = Some((pixel_id, color_index, value));
                                        }
                                    }
                                    None => min_value = Some((pixel_id, color_index, value)),
                                }
                            }
                        }
                    }
                }
            }

            if let Some((pixel_id, color_index, _)) = min_value {
                *backtracks += 1;

                let mut colors1 = bitvec![0; self.resources.color_num];
                colors1.set(color_index, true);
                let colors2 = self.get_memo(pixel_id).to_bitvec() ^ &colors1;

                let mut layer_solver1 = LayerSolver::new(self.resources, Some(&self));
                let mut layer_solver2 = LayerSolver::new(self.resources, Some(&self));

                let priority_queue1 = layer_solver1.set_pixel_memo(pixel_id, &colors1, &colors2);
                let priority_queue2 = layer_solver2.set_pixel_memo(pixel_id, &colors2, &colors1);

                layer_solver1.solve(priority_queue1, backtracks, nlines);
                layer_solver2.solve(priority_queue2, backtracks, nlines);

                self.grid = layer_solver1.grid | layer_solver2.grid;
            }
        }
    }

    fn calc_value(&self, pixel_id: PixelId, color_index: usize) -> f64 {
        let row_probability = {
            let line = self.get_line(LineId::Row(pixel_id.row_index)).unwrap();
            line.get_color_case(pixel_id.column_index, color_index) as f64
                / line.get_painting_count() as f64
        };
        let column_probability = {
            let line = self
                .get_line(LineId::Column(pixel_id.column_index))
                .unwrap();
            line.get_color_case(pixel_id.row_index, color_index) as f64
                / line.get_painting_count() as f64
        };

        (row_probability - 0.5) * (column_probability - 0.5)
    }

    fn show_blank_possibility(&mut self) {
        print!(" ");
        for _ in 0..self.resources.width {
            print!("__");
        }
        println!();

        for row_index in 0..self.resources.height {
            print!("|");
            for pixel_id in PixelIterator::new(LineId::Row(row_index), self.resources.width) {
                let memo = self.get_memo(pixel_id);
                if memo[0] && memo[1..].not_all() {
                    print!("$$");
                } else if memo.count_ones() == 1 {
                    print!("  ");
                } else {
                    print!("..");
                }
            }
            println!("|");
        }

        print!(" ");
        for _ in 0..self.resources.width {
            print!("‾‾");
        }
        println!();
    }
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
