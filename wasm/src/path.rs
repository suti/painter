use std::ops::Add;
use wasm_bindgen::JsValue;
use svgtypes::{PathParser, PathSegment};
use crate::js_console_info;
use crate::painter_core::path::PathData as PD;
use crate::wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct Path2D(PD);

#[wasm_bindgen]
impl Path2D {
    #[wasm_bindgen(constructor)]
    pub fn new(path: JsValue) -> Self {
        let path = path.as_string().unwrap_or(format!(""));
        let pd = str2path(path.as_str()).unwrap_or(PD::new());
        Path2D(pd)
    }

    #[wasm_bindgen(js_name = beginPath)]
    pub fn begin_path(&mut self) {
        self.0.0.clear();
    }

    pub fn arc(&mut self, x: f32, y: f32, radius: f32, start_angle: f32, end_angle: f32, anticlockwise: bool) {
        self.0.arc2(x, y, radius, start_angle, end_angle, anticlockwise)
    }

    #[wasm_bindgen(js_name = moveTo)]
    pub fn move_to(&mut self, x: f32, y: f32) {
        self.0.move_to(x, y);
    }

    #[wasm_bindgen(js_name = lineTo)]
    pub fn line_to(&mut self, x: f32, y: f32) {
        self.0.line_to(x, y);
    }

    #[wasm_bindgen(js_name = bezierCurveTo)]
    pub fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        self.0.curve_to(x1, y1, x2, y2, x, y);
    }

    #[wasm_bindgen(js_name = quadraticCurveTo)]
    pub fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        self.0.quad_to(x1, y1, x, y);
    }

    #[wasm_bindgen(js_name = closePath)]
    pub fn close_path(&mut self) {
        self.0.close();
    }

    #[wasm_bindgen(js_name = toRust)]
    pub fn to_rust(self) -> Vec<f32> {
        self.encode_array()
    }

    #[wasm_bindgen(js_name = encode)]
    pub fn encode_array(&self) -> Vec<f32> {
        (&self.0).into()
    }
}

enum PathSegmentBox {
    Move([f32; 2], usize),
    Line([f32; 2], usize),
    Curve([f32; 6], usize),
    Close,
}

impl PathSegmentBox {
    fn move_() -> PathSegmentBox {
        PathSegmentBox::Move([0.0, 0.0], 0)
    }

    fn line() -> PathSegmentBox {
        PathSegmentBox::Line([0.0, 0.0], 0)
    }

    fn curve() -> PathSegmentBox {
        PathSegmentBox::Curve([0.0, 0.0, 0.0, 0.0, 0.0, 0.0], 0)
    }

    fn close() -> PathSegmentBox {
        PathSegmentBox::Close
    }

    fn push(&mut self, value: f32) -> bool {
        match self {
            PathSegmentBox::Line(data, count) |
            PathSegmentBox::Move(data, count) => {
                unsafe {
                    *data.as_mut_ptr().add(*count) = value;
                    *count += 1;
                }
                *count == 2
            }
            PathSegmentBox::Curve(data, count) => {
                unsafe {
                    *data.as_mut_ptr().add(*count) = value;
                    *count += 1;
                }
                *count == 6
            }
            PathSegmentBox::Close => {
                true
            }
        }
    }

    fn seg(self, pd: &mut PD) {
        match self {
            PathSegmentBox::Line([x, y], _) => {
                pd.line_to(x, y)
            }
            PathSegmentBox::Move([x, y], _) => {
                pd.move_to(x, y)
            }
            PathSegmentBox::Curve([x1, y1, x2, y2, x, y], _) => {
                pd.curve_to(x1, y1, x2, y2, x, y)
            }
            PathSegmentBox::Close => {
                pd.close()
            }
        }
    }
}


pub(crate) fn array2path(input: Vec<f32>) -> Option<PD> {
    let len = input.get(0)?.clone() as usize;
    let mut count = 0usize;
    let mut pd = PD::new();
    let mut psb: Option<PathSegmentBox> = None;
    let ii = &input[1..];
    for item in ii.iter() {
        let mut comp = false;
        {
            if let Some(psb) = psb.as_mut() {
                comp = psb.push(*item);
            } else {
                match *item {
                    0f32 => {
                        psb = Some(PathSegmentBox::move_())
                    }
                    1f32 => {
                        psb = Some(PathSegmentBox::line())
                    }
                    2f32 => {
                        psb = Some(PathSegmentBox::curve())
                    }
                    3f32 => {
                        psb = Some(PathSegmentBox::close());
                        comp = true;
                    }
                    _ => return None
                }
            }
        }
        if comp {
            count += 1;
            psb.unwrap().seg(&mut pd);
            psb = None;
        }
    }
    if pd.len() == len {
        Some(pd)
    } else {
        None
    }
}


fn add_path_segment(p: &mut PD, d: PathSegment) {
    match d {
        PathSegment::MoveTo { x, y, abs: _ } => {
            p.move_to(x as f32, y as f32)
        }
        PathSegment::LineTo { x, y, abs: _ } => {
            p.line_to(x as f32, y as f32)
        }
        PathSegment::CurveTo { x, y, x1, y1, x2, y2, abs: _ } => {
            p.curve_to(x as f32, y as f32, x1 as f32, y1 as f32, x2 as f32, y2 as f32)
        }
        PathSegment::Quadratic { x, y, x1, y1, abs: _ } => {
            p.quad_to(x as f32, y as f32, x1 as f32, y1 as f32)
        }
        PathSegment::ClosePath { abs: _ } => {
            p.close()
        }
        _ => {
            println!("Do not support {:?}", d)
        }
    }
}

fn str2path(input: &str) -> Option<PD> {
    let mut pd = PD::new();
    let result = PathParser::from(input);
    for d_r in result {
        let d = d_r.ok()?;
        add_path_segment(&mut pd, d);
    }
    Some(pd)
}

#[test]
fn test() {
    let arr = vec![7.0,
                   0.0, 1000.0, 200.0,
                   2.0, 239.21167, 752.82623, 653.42926, 200.03024, 346.29065, 423.2123,
                   2.0, 529.8529, 1647.2218, 132.13269, 1082.4402, 249.46776, 1443.5176,
                   2.0, 1470.2867, 1647.2218, 810.2381, 1850.926, 1189.9016, 1850.926,
                   2.0, 1760.928, 752.82623, 1750.6719, 1443.5176, 1868.007, 1082.4402,
                   2.0, 1000.13965, 200.0, 1653.849, 423.2123, 1346.7103, 200.03024,
                   3.0];
    let path = array2path(arr);
    println!("{:?}", path);
}