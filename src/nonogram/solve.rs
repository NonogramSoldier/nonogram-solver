use std::{cell::RefCell, collections::HashMap, rc::Rc};

use fxhash::FxBuildHasher;

use crate::priority_queue::PriorityQueue;
use super::Clues;

pub struct LayerSolver<'a> {
    clues: &'a Clues,
    layer: LayerRef,
    grid_probabilities: GridProbabilities,
    priority_queue: PriorityQueue<LineId, Priority>,
    is_base_layer: bool,
}

struct LayerRef {
    refer: Rc<RefCell<Layer>>,
}

struct Layer {
    parent: Option<LayerRef>,
    pixels: HashMap<LineId, LayerPixel, FxBuildHasher>,
}

struct LayerPixel {
    is_empty_memo: ColorMemo,
    color_memos: Vec<ColorMemo>,
}

enum ColorMemo {
    Certain,
    Possible(Option<LayeredMemo>),
    Impossible,
}

struct LayeredMemo {
    certain: LayerRef,
    impossible: LayerRef,
}

struct GridProbabilities {
    row: Vec<LineProbabilities>,
    column: Vec<LineProbabilities>,
}

struct LineProbabilities {
    line: Vec<ColorProbabilities>,
    painting_count: u128,
}

struct ColorProbabilities {
    empty_probability: Probability,
    color_probabilities: Vec<Probability>,
}

enum Probability {
    Certain,
    Possible(u128),
    Impossible,
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
enum LineId {
    Row(usize),
    Column(usize),
}

type Priority = f32;
