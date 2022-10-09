use std::ops::{Deref, DerefMut};
use std::str::FromStr;
use svgtypes::Color;

use crate::wasm_bindgen::JsValue;
use crate::wasm_bindgen::prelude::*;

use crate::painter_core::context2d::Context;
use crate::painter_core::{LineCap, LineJoin};
use crate::painter_core::paint::{FillRule, Paint};
use crate::painter_core::transform::Transform;
use crate::painter_core::font::FontBucket;

use crate::path::array2path;
use crate::canvas::Canvas;

#[wasm_bindgen]
pub struct Context2d {
    inner: Canvas,
}

impl Deref for Context2d {
    type Target = Context<'static>;

    fn deref(&self) -> &Self::Target {
        &self.inner.ctx
    }
}

impl DerefMut for Context2d {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner.ctx
    }
}


fn get_fill_rule(value: JsValue) -> Option<FillRule> {
    let value = value.as_string().unwrap_or(format!(""));
    match value.as_str() {
        "nonzero" => Some(FillRule::Nonzero),
        "evenodd" => Some(FillRule::Evenodd),
        _ => None
    }
}

#[wasm_bindgen]
impl Context2d {
    pub fn new(inner: Canvas) -> Self {
        Context2d { inner }
    }

    #[wasm_bindgen(getter)]
    pub fn canvas(self) -> Canvas {
        self.inner
    }

    #[wasm_bindgen(setter = fontFamily)]
    pub fn set_font_family(&mut self, family: &str) {
        self.inner.ctx.set_font_family(family)
    }

    #[wasm_bindgen(setter = fontSize)]
    pub fn set_font_size(&mut self, size: f32) {
        self.inner.ctx.set_font_size(size)
    }

    #[wasm_bindgen(setter = strokeStyle)]
    pub fn set_stroke_style(&mut self, style: &str) {
        let color = Color::from_str(style).ok().unwrap_or(Color::black());
        let p = Paint::from_color_rgba8(color.red, color.green, color.blue, color.alpha);
        self.inner.ctx.set_stroke_style(p);
    }

    #[wasm_bindgen(setter = fillStyle)]
    pub fn set_fill_style(&mut self, style: &str) {
        let color = Color::from_str(style).ok().unwrap_or(Color::black());
        let p = Paint::from_color_rgba8(color.red, color.green, color.blue, color.alpha);
        self.inner.ctx.set_fill_style(p);
    }

    #[wasm_bindgen(setter = lineWidth)]
    pub fn set_line_width(&mut self, width: f32) {
        self.inner.ctx.set_line_width(width);
    }

    #[wasm_bindgen(setter = lineCap)]
    pub fn set_line_cap(&mut self, line_cap: &str) {
        let line_cap = match line_cap {
            "butt" => { LineCap::Butt }
            "round" => { LineCap::Round }
            "square" => { LineCap::Square }
            _ => { LineCap::default() }
        };
        self.inner.ctx.set_line_cap(line_cap);
    }

    #[wasm_bindgen(setter = lineJoin)]
    pub fn set_line_join(&mut self, line_join: &str) {
        let line_join = match line_join {
            "round" => { LineJoin::Round }
            "miter" => { LineJoin::Miter }
            "bevel" => { LineJoin::Bevel }
            _ => { LineJoin::default() }
        };
        self.inner.ctx.set_line_join(line_join);
    }

    #[wasm_bindgen(setter = miterLimit)]
    pub fn set_miter_limit(&mut self, miter_limit: f32) {
        self.inner.ctx.set_miter_limit(miter_limit)
    }

    pub fn set_line_dash_offset(&mut self, line_dash_offset: JsValue) {
        self.inner.ctx.set_line_dash_offset(None)
    }

    pub fn arc(&mut self, x: f32, y: f32, radius: f32, start_angle: f32, end_angle: f32, anticlockwise: bool) {
        self.inner.ctx.arc(x, y, radius, start_angle, end_angle, anticlockwise);
    }

    #[wasm_bindgen(js_name = arcTo)]
    pub fn arc_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32) {
        self.inner.ctx.arc_to(x1, y1, x2, y2);
    }

    #[wasm_bindgen(js_name = beginPath)]
    pub fn begin_path(&mut self) {
        self.inner.ctx.begin_path();
    }

    #[wasm_bindgen(js_name = bezierCurveTo)]
    pub fn bezier_curve_to(&mut self, cp1x: f32, cp1y: f32, cp2x: f32, cp2y: f32, x: f32, y: f32) {
        self.inner.ctx.bezier_curve_to(cp1x, cp1y, cp2x, cp2y, x, y);
    }

    #[wasm_bindgen(js_name = clearRect)]
    pub fn clear_rect(&mut self) {
        todo!()
    }

    pub fn clip(&mut self, path: Option<Box<[f32]>>, fill_rule: JsValue) {
        self.inner.ctx.clip(path.and_then(|p| array2path(p.to_vec())), get_fill_rule(fill_rule));
    }

    #[wasm_bindgen(js_name = closePath)]
    pub fn close_path(&mut self) {
        self.inner.ctx.close_path();
    }

    #[wasm_bindgen(js_name = createConicGradient)]
    pub fn create_conic_gradient() {
        todo!()
    }

    #[wasm_bindgen(js_name = createImageData)]
    pub fn create_image_data() {
        todo!()
    }

    #[wasm_bindgen(js_name = createLinearGradient)]
    pub fn create_linear_gradient() {
        todo!()
    }

    #[wasm_bindgen(js_name = createPattern)]
    pub fn create_pattern() {
        todo!()
    }

    #[wasm_bindgen(js_name = createRadialGradient)]
    pub fn create_radial_gradient() {
        todo!()
    }

    #[wasm_bindgen(js_name = drawImage)]
    pub fn draw_image(&mut self) {
        todo!()
    }
    pub fn ellipse(&mut self) {
        todo!()
    }
    pub fn fill(&mut self, path: Option<Box<[f32]>>, fill_rule: JsValue) {
        self.inner.ctx.fill(path.and_then(|p| array2path(p.to_vec())), get_fill_rule(fill_rule));
    }

    #[wasm_bindgen(js_name = fillRect)]
    pub fn fill_rect(&mut self, x: f32, y: f32, width: f32, height: f32) {
        self.inner.ctx.fill_rect(x, y, width, height);
    }

    #[wasm_bindgen(js_name = fillText)]
    pub fn fill_text(&mut self, text: &str, x: f32, y: f32, max_width: Option<f32>) {
        let fb: &mut dyn FontBucket = &mut self.inner.font_db;
        self.inner.ctx.fill_text(text, x, y, max_width, Some(fb));
    }
    pub fn get_context_attributes(&self) {
        todo!()
    }
    pub fn get_image_data(&self) {
        todo!()
    }

    #[wasm_bindgen(getter = lineDash)]
    pub fn get_line_dash(&self) {
        self.inner.ctx.get_line_dash();
    }

    pub fn get_transform(&self) {
        todo!()
    }

    #[wasm_bindgen(js_name = lineTo)]
    pub fn line_to(&mut self, x: f32, y: f32) {
        self.inner.ctx.line_to(x, y)
    }

    #[wasm_bindgen(js_name = measureText)]
    pub fn measure_text() {
        todo!()
    }

    #[wasm_bindgen(js_name = moveTo)]
    pub fn move_to(&mut self, x: f32, y: f32) {
        self.inner.ctx.move_to(x, y)
    }

    #[wasm_bindgen(js_name = putImageData)]
    pub fn put_image_data(&mut self) {
        todo!()
    }

    #[wasm_bindgen(js_name = quadraticCurveTo)]
    pub fn quadratic_curve_to(&mut self, cpx: f32, cpy: f32, x: f32, y: f32) {
        self.inner.ctx.quadratic_curve_to(cpx, cpy, x, y)
    }
    pub fn rect(&mut self, x: f32, y: f32, width: f32, height: f32) {
        self.inner.ctx.rect(x, y, width, height);
    }

    #[wasm_bindgen(js_name = resetTransform)]
    pub fn reset_transform(&mut self) {
        self.inner.ctx.reset_transform();
    }
    pub fn restore(&mut self) {
        self.inner.ctx.restore();
    }
    pub fn rotate(&mut self, angle: f32) {
        self.inner.ctx.rotate(angle)
    }
    pub fn save(&mut self) {
        self.inner.ctx.save();
    }

    #[wasm_bindgen(js_name = setTransform)]
    pub fn set_transform(&mut self, a: f32, b: f32, c: f32, d: f32, e: f32, f: f32) {
        self.inner.ctx.set_transform(&Transform::new(a, b, c, d, e, f))
    }
    pub fn stroke(&mut self, path: Option<Box<[f32]>>) {
        self.inner.ctx.stroke(path.and_then(|p| array2path(p.to_vec())));
    }

    #[wasm_bindgen(js_name = strokeRect)]
    pub fn stroke_rect(&mut self, x: f32, y: f32, width: f32, height: f32) {
        self.inner.ctx.stroke_rect(x, y, width, height)
    }

    #[wasm_bindgen(js_name = strokeText)]
    pub fn stroke_text(&mut self, text: &str, x: f32, y: f32, max_width: Option<f32>) {
        let fb: &mut dyn FontBucket = &mut self.inner.font_db;
        self.inner.ctx.stroke_text(text, x, y, max_width, Some(fb));
    }
    pub fn transform(&mut self, a: f32, b: f32, c: f32, d: f32, e: f32, f: f32) {
        self.inner.ctx.transform(&Transform::new(a, b, c, d, e, f))
    }
    pub fn translate(&mut self, x: f32, y: f32) {
        self.inner.ctx.translate(x, y)
    }
}