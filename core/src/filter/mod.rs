use skia::Pixmap;
use crate::filter::drop_shadow::DropShadow;
use crate::transform::Transform;

pub mod drop_shadow;

pub enum FilterType {
    DropShadow(DropShadow)
}

pub trait ApplyFilter {
    fn apply_filter(&self, source: Pixmap) -> (Pixmap, Transform);
}
