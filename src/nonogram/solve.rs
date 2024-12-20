mod line_probability;
mod solve_resources;

use crate::priority_queue::FxPriorityQueue;
use cpu_time::ProcessTime;
use fxhash::FxHashMap;
use line_probability::LineProbability;
use solve_resources::SolveResources;
use std::time::Instant;

use super::*;

pub fn solve(puzzle: &Puzzle) {
    let cpu_start = ProcessTime::now();
    let start = Instant::now();

    let resources = SolveResources::new(puzzle);

    let mut backtracks = 0;
    let mut nlines = 0;
    let mut layer_solver = LayerSolver::new(
        None,
        &resources,
        vec![vec![resources.uncertain_memo; resources.width]; resources.height],
    );
    if let Some(priority_queue) = layer_solver.init(&mut nlines) {
        let result = layer_solver.solve(priority_queue, &mut backtracks, &mut nlines);
        let duration = start.elapsed();
        let cpu_duration = cpu_start.elapsed();
        match result {
            SolveResult::FullySolved => {
                layer_solver.show_blank_possibility();
                println!("line_solves: {}", nlines);
                println!("backtracks:  {}", backtracks);
            }
            SolveResult::PartiallySolved => {
                println!("kya-");
                layer_solver.show_blank_possibility();
            }
            SolveResult::Conflict => println!("nanndeyanenn"),
        }
        println!("cpu-time: {} sec", cpu_duration.as_secs_f64());
        println!("    time: {} sec", duration.as_secs_f64());
    } else {
        let duration = start.elapsed();
        let cpu_duration = cpu_start.elapsed();
        println!("nanndeyanenn");
        println!("cpu-time: {} sec", cpu_duration.as_secs_f64());
        println!("    time: {} sec", duration.as_secs_f64());
    }
}

fn calc_priority(
    line: &LineProbability,
    index: usize,
    new_possible_colors: usize,
    new_impossible_colors: usize,
) -> f64 {
    let mut possible_num = 0;
    let mut impossible_num = 0;
    for new_possible_color in ColorIterator::new(new_possible_colors) {
        possible_num += line.get_color_case(index, new_possible_color)
    }
    for new_impossible_color in ColorIterator::new(new_impossible_colors) {
        impossible_num += line.get_color_case(index, new_impossible_color);
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
    parent: Option<&'a LayerSolver<'a>>,
    grid: FxHashMap<PixelId, usize>,
    grid_cache: Vec<Vec<usize>>,
    line_probabilities: FxHashMap<LineId, LineProbability>,
    line_cache: FxHashMap<LineId, &'a LineProbability>,
}

impl<'a> LayerSolver<'a> {
    fn new(
        parent: Option<&'a LayerSolver<'a>>,
        resources: &'a SolveResources<'a>,
        grid_cache: Vec<Vec<usize>>,
    ) -> Self {
        Self {
            resources,
            parent,
            grid: Default::default(),
            grid_cache,
            line_probabilities: Default::default(),
            line_cache: Default::default(),
        }
    }

    fn get_memo(&self, pixel_id: PixelId) -> usize {
        self.grid
            .get(&pixel_id)
            .copied()
            .unwrap_or_else(|| self.grid_cache[pixel_id.row_index][pixel_id.column_index])
    }

    fn cache_line(&mut self, line_id: LineId) -> Option<&LineProbability> {
        match self.line_probabilities.get(&line_id) {
            Some(line) => Some(line),
            None => match self.line_cache.get(&line_id) {
                Some(&line) => Some(line),
                None => match self.get_ancestral_line(line_id) {
                    Some(line) => {
                        self.line_cache.insert(line_id, line);
                        Some(line)
                    }
                    None => None,
                },
            },
        }
    }

    fn get_ancestral_line(&self, line_id: LineId) -> Option<&'a LineProbability> {
        match self.parent {
            Some(parent) => match parent.line_probabilities.get(&line_id) {
                Some(line_probability) => Some(line_probability),
                None => match parent.line_cache.get(&line_id) {
                    Some(&line_probability) => Some(line_probability),
                    None => parent.get_ancestral_line(line_id),
                },
            },
            None => None,
        }
    }

    fn set_pixel_memo(
        &mut self,
        pixel_id: PixelId,
        new_possible_colors: usize,
        new_impossible_colors: usize,
    ) -> FxPriorityQueue<LineId, Priority> {
        self.grid.insert(pixel_id, new_possible_colors);
        let mut vec: Vec<(LineId, Priority)> = Default::default();
        vec.push((
            LineId::Row(pixel_id.row_index),
            calc_priority(
                self.cache_line(LineId::Row(pixel_id.row_index)).unwrap(),
                pixel_id.column_index,
                new_possible_colors,
                new_impossible_colors,
            ),
        ));
        vec.push((
            LineId::Column(pixel_id.column_index),
            calc_priority(
                self.cache_line(LineId::Column(pixel_id.column_index))
                    .unwrap(),
                pixel_id.row_index,
                new_possible_colors,
                new_impossible_colors,
            ),
        ));

        FxPriorityQueue::new_heapify(vec)
    }

    fn init(&mut self, nlines: &mut u128) -> Option<FxPriorityQueue<LineId, Priority>> {
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
        let mut result: FxPriorityQueue<LineId, Priority> = FxPriorityQueue::new();

        loop {
            match priority_queue.pop() {
                Some(value) => {
                    *nlines += 1;
                    if !self.line_solve(value.0, &mut result) {
                        return None;
                    }
                }
                None => {
                    return Some(result);
                }
            }
        }
    }

    fn line_solve(
        &mut self,
        line_id: LineId,
        priority_queue: &mut FxPriorityQueue<LineId, Priority>,
    ) -> bool {
        let mut line_memo: Vec<usize> = Vec::new();
        for pixel_id in PixelIterator::new(line_id, self.resources.get_length(line_id)) {
            line_memo.push(self.get_memo(pixel_id));
        }

        if !self
            .line_probabilities
            .entry(line_id)
            .or_insert_with(|| {
                LineProbability::new(
                    self.resources,
                    line_id,
                    self.line_cache.get(&line_id).copied(),
                )
            })
            .solve(&line_memo, self.resources.get_line_clue(line_id))
        {
            return false;
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
                if let Some(line) = self.cache_line(oppo_line) {
                    priority_queue.add_or_insert(
                        oppo_line,
                        calc_priority(line, oppo_index, new_possible_colors, new_impossible_colors),
                    );
                }
            }
        }

        true
    }

    fn solve(
        &mut self,
        mut priority_queue: FxPriorityQueue<LineId, Priority>,
        backtracks: &mut u128,
        nlines: &mut u128,
    ) -> SolveResult {
        loop {
            if let Some((line_id, _)) = priority_queue.pop() {
                *nlines += 1;
                if !self.line_solve(line_id, &mut priority_queue) {
                    return SolveResult::Conflict;
                }
            } else {
                break;
            }
        }

        let mut min_value: Option<(f64, PixelId, usize)> = None;
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
                                if value < tuple.0 {
                                    min_value = Some((value, pixel_id, color_index))
                                }
                            }
                            None => min_value = Some((value, pixel_id, color_index)),
                        }
                    }
                    2.. => {
                        for color_index in ColorIterator::new(pixel_memo) {
                            let value = self.calc_value(pixel_id, color_index);
                            match min_value {
                                Some(tuple) => {
                                    if value < tuple.0 {
                                        min_value = Some((value, pixel_id, color_index))
                                    }
                                }
                                None => min_value = Some((value, pixel_id, color_index)),
                            }
                        }
                    }
                }
            }
        }

        match min_value {
            Some((_, pixel_id, color_index)) => {
                *backtracks += 1;
                let result1;
                let result2;
                let grid1;
                let grid2;
                {
                    let colors1 = 1 << color_index;
                    let colors2 = self.get_memo(pixel_id) ^ colors1;

                    let mut grid_cache = self.grid_cache.clone();
                    for (&pixel_id, &memo) in self.grid.iter() {
                        grid_cache[pixel_id.row_index][pixel_id.column_index] = memo;
                    }

                    let mut layer_solver1 =
                        LayerSolver::new(Some(&self), self.resources, grid_cache.clone());
                    let mut layer_solver2 =
                        LayerSolver::new(Some(&self), self.resources, grid_cache);

                    let priority_queue1 = layer_solver1.set_pixel_memo(pixel_id, colors1, colors2);
                    let priority_queue2 = layer_solver2.set_pixel_memo(pixel_id, colors2, colors1);

                    result1 = layer_solver1.solve(priority_queue1, backtracks, nlines);
                    result2 = layer_solver2.solve(priority_queue2, backtracks, nlines);
                    grid1 = layer_solver1.grid;
                    grid2 = layer_solver2.grid;
                }

                match (result1, result2) {
                    (SolveResult::FullySolved, SolveResult::FullySolved)
                    | (SolveResult::FullySolved, SolveResult::PartiallySolved)
                    | (SolveResult::PartiallySolved, SolveResult::FullySolved)
                    | (SolveResult::PartiallySolved, SolveResult::PartiallySolved) => {
                        let mut grid1 = grid1;
                        for (pixel_id, &memo2) in grid2.iter() {
                            if let Some(memo1) = grid1.get_mut(pixel_id) {
                                *memo1 |= memo2;
                            }
                        }

                        for (&pixel_id, &memo1) in grid1.iter() {
                            if let Some(memo) = self.grid.get(&pixel_id) {
                                if *memo == memo1 {
                                    self.grid.remove(&pixel_id);
                                } else {
                                    self.grid.insert(pixel_id, memo1);
                                }
                            } else {
                                self.grid.insert(pixel_id, memo1);
                            }
                        }

                        SolveResult::PartiallySolved
                    }
                    (SolveResult::FullySolved, SolveResult::Conflict) => {
                        for (&pixel_id, &memo1) in grid1.iter() {
                            self.grid.insert(pixel_id, memo1);
                        }

                        SolveResult::FullySolved
                    }
                    (SolveResult::Conflict, SolveResult::FullySolved) => {
                        for (&pixel_id, &memo2) in grid2.iter() {
                            self.grid.insert(pixel_id, memo2);
                        }

                        SolveResult::FullySolved
                    }
                    (SolveResult::PartiallySolved, SolveResult::Conflict) => {
                        for (&pixel_id, &memo1) in grid1.iter() {
                            self.grid.insert(pixel_id, memo1);
                        }

                        SolveResult::PartiallySolved
                    }
                    (SolveResult::Conflict, SolveResult::PartiallySolved) => {
                        for (&pixel_id, &memo2) in grid2.iter() {
                            self.grid.insert(pixel_id, memo2);
                        }

                        SolveResult::PartiallySolved
                    }
                    (SolveResult::Conflict, SolveResult::Conflict) => SolveResult::Conflict,
                }
            }
            None => SolveResult::FullySolved,
        }
    }

    fn calc_value(&mut self, pixel_id: PixelId, color_index: usize) -> f64 {
        let row_probability = {
            let line = self.cache_line(LineId::Row(pixel_id.row_index)).unwrap();
            line.get_color_case(pixel_id.column_index, color_index) as f64
                / line.get_painting_count() as f64
        };
        let column_probability = {
            let line = self
                .cache_line(LineId::Column(pixel_id.column_index))
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
                if memo == 1 {
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

enum SolveResult {
    FullySolved,
    PartiallySolved,
    Conflict,
}
