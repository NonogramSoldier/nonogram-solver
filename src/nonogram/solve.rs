use std::{cell::RefCell, collections::HashMap, rc::Rc};

use fxhash::FxBuildHasher;

use super::Puzzle;
use crate::priority_queue::PriorityQueue;

#[derive(Debug)]
pub struct LayerSolver<'a> {
    puzzle: &'a Puzzle,
    layer: LayerRef,
    grid_probabilities: GridProbabilities,
    is_base_layer: bool,
}

impl<'a> LayerSolver<'a> {
    pub fn new(puzzle: &'a Puzzle, is_base_layer: bool) -> Self {
        Self {
            puzzle,
            layer: LayerRef::new(None),
            grid_probabilities: GridProbabilities::new(
                puzzle.get_row_length(),
                puzzle.get_column_length(),
                puzzle.get_color_num(),
            ),
            is_base_layer,
        }
    }
}

#[derive(Debug)]
struct LayerRef {
    refer: Rc<RefCell<Layer>>,
}

impl LayerRef {
    fn new(parent: Option<LayerRef>) -> Self {
        Self {
            refer: Rc::new(RefCell::new(Layer::new(parent))),
        }
    }
}

#[derive(Debug)]
struct Layer {
    parent: Option<LayerRef>,
    pixels: HashMap<LineId, LayerPixel, FxBuildHasher>,
}

impl Layer {
    fn new(parent: Option<LayerRef>) -> Self {
        Self {
            parent,
            pixels: HashMap::default(),
        }
    }
}

#[derive(Debug)]
struct LayerPixel {
    empty_memo: ColorMemo,
    color_memos: Vec<ColorMemo>,
}

#[derive(Debug)]
enum ColorMemo {
    Certain,
    Possible(Option<LayerRef>),
    Impossible,
}

#[derive(Debug)]
struct GridProbabilities {
    row: Vec<LineProbabilities>,
    column: Vec<LineProbabilities>,
}

impl GridProbabilities {
    fn new(row_length: usize, column_length: usize, color_num: usize) -> Self {
        Self {
            row: vec![LineProbabilities::new(row_length, color_num); column_length],
            column: vec![LineProbabilities::new(column_length, color_num); row_length],
        }
    }
}

#[derive(Debug, Clone)]
struct LineProbabilities {
    line: Vec<ColorProbabilities>,
    painting_count: u128,
}

impl LineProbabilities {
    fn new(length: usize, color_num: usize) -> Self {
        Self {
            line: vec![ColorProbabilities::new(color_num); length],
            painting_count: 0,
        }
    }
}

#[derive(Debug, Clone)]
struct ColorProbabilities {
    color_probabilities: Vec<Probability>,
}

impl ColorProbabilities {
    fn new(color_num: usize) -> Self {
        Self {
            color_probabilities: vec![Probability::default(); color_num],
        }
    }
}

#[derive(Debug, Clone)]
enum Probability {
    Certain,
    Possible(u128),
    Impossible,
}

impl Default for Probability {
    fn default() -> Self {
        Self::Possible(0)
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
enum LineId {
    Row(usize),
    Column(usize),
}

type Priority = f32;
