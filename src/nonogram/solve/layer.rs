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

    fn set_pixel_memo(&mut self, pixel_id: PixelId, color_index: usize) -> bool {
        todo!()
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

    fn set_pixel_memo(&mut self, pixel_id: PixelId, color_index: usize) -> bool {
        todo!()
    }
}

#[derive(Debug)]
pub struct LayerPixelMemo {
    possibles: FxHashSet<usize>,
    children: FxHashMap<usize, LayerRef>,
}
