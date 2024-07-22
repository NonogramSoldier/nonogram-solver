use super::*;

#[derive(Debug)]
pub struct LayerRef {
    refer: Rc<RefCell<Layer>>,
}

impl LayerRef {
    pub fn new(parent: Option<LayerRef>) -> Self {
        Self {
            refer: Rc::new(RefCell::new(Layer::new(parent))),
        }
    }
}

#[derive(Debug)]
struct Layer {
    parent: Option<LayerRef>,
    grid: FxHashMap<PixelId, PixelMemo>,
}

impl Layer {
    fn new(parent: Option<LayerRef>) -> Self {
        Self {
            parent,
            grid: FxHashMap::default(),
        }
    }
}

#[derive(Debug)]
pub struct PixelMemo {
    blank_possibility: Possibility,
    paint_possibilities: Vec<Possibility>,
}

#[derive(Debug)]
enum Possibility {
    Possible(Option<LayerRef>),
    Impossible,
}

impl Default for Possibility {
    fn default() -> Self {
        Self::Possible(None)
    }
}
