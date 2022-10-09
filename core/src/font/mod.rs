use crate::transform::Transform;
use crate::paint::FillRule;
use crate::PathData;

pub use crate::ttf::Rect;
pub use crate::ttf::Style as FontStyle;
pub use crate::ttf::{LineMetrics, ScriptMetrics};


#[derive(Default, Clone, Debug)]
pub struct Glyph {
    pub path: PathData,
    pub fill_rule: FillRule,
    pub transform: Transform,
}

#[derive(Default, Clone, Debug)]
pub struct FontStyles {
    pub family: String,
    pub size: f32,
    pub stretch: String,
    pub style: FontStyle,
    pub variant: String,
    pub weight: String,
    pub line_height: f32,
}

pub trait FontBucket {
    fn default_glyph(&self, style: &FontStyles) -> Glyph;

    fn get_glyph(&mut self, style: &FontStyles, text: &str) -> Option<Glyph>;

    fn load_font(&mut self, buf: &[u8]) -> Option<()>;
}