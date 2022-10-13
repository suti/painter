use skia::Pixmap;
use crate::filter::ApplyFilter;
use crate::paint::color::Color;
use crate::transform::Transform;


#[derive(Copy, Clone, Debug, Default)]
pub struct DropShadow {
    pub offset_x: f32,
    pub offset_y: f32,
    pub blur: f32,
    pub color: Color,
}

impl ApplyFilter for DropShadow {
    fn apply_filter(&self, _source: Pixmap) -> (Pixmap, Transform) {
        todo!()
    }
}

fn calc_effect_box(effect: DropShadow) {
    // let DropShadow{} = effect;
}

