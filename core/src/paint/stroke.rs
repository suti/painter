use sk_path::{LineCap, LineJoin, StrokeDash};
use crate::paint::Paint;

#[derive(Clone, Debug)]
pub struct Stroke {
    pub width: f32,
    pub miter_limit: f32,
    pub line_cap: LineCap,
    pub line_join: LineJoin,
    pub dash: Option<StrokeDash>,
    pub paint: Paint,
}

impl Default for Stroke {
    fn default() -> Self {
        Stroke {
            width: 1.0,
            miter_limit: 4.0,
            line_cap: LineCap::default(),
            line_join: LineJoin::default(),
            dash: None,
            paint: Paint::default(),
        }
    }
}

pub fn lint_join_to_string(v: LineJoin) -> String {
    match v {
        LineJoin::Miter => { String::from("miter") }
        LineJoin::Round => { String::from("round") }
        LineJoin::Bevel => { String::from("bevel") }
    }
}

pub fn lint_cap_to_string(v: LineCap) -> String {
    match v {
        LineCap::Butt => { String::from("butt") }
        LineCap::Round => { String::from("round") }
        LineCap::Square => { String::from("square") }
    }
}