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
    grid: FxHashMap<PixelId, LayerPixelMemo>,
}

impl Layer {
    fn new(parent: Option<LayerRef>) -> Self {
        Self {
            parent,
            grid: Default::default(),
        }
    }
}

#[derive(Debug)]
pub struct LayerPixelMemo {
    possibles: FxHashMap<usize, Option<LayerRef>>,
}

impl LayerPixelMemo {
    fn make_children(&mut self) {}
}
