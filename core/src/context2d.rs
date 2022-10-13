use std::f32::consts::PI;
use std::fmt::{Debug, Formatter};
use crate::paint::{ClipMask, FillRule, Paint};
use crate::path::{PathBuilder, PathData, quad_to_curve};
use crate::transform::Transform;

use tiny_skia_path::{LineCap, LineJoin, StrokeDash};
use crate::backend::PainterBackend;
use crate::font::{FontBucket, Glyph};
use crate::operate::Operates;
use crate::paint::stroke::Stroke;
use crate::style_bucket::{StyleBucket, StyleStore};


#[derive(Default)]
pub struct Context<'a> {
    width: f32,
    height: f32,
    style_bucket: StyleStore,
    path_cache: PathBuilder,
    operate_queue: Operates,
    font_bucket: Option<&'a mut dyn FontBucket>,
}

impl<'a> Context<'a> {
    pub fn new() -> Self {
        Context::new_wh(200.0, 200.0)
    }

    pub fn new_wh(width: f32, height: f32) -> Self {
        let mut ctx = Context::default();
        ctx.width = width;
        ctx.height = height;
        ctx
    }

    pub fn reset_wh(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
    }

    pub fn set_font_family(&mut self, family: &str) {
        self.style_bucket.font_family = String::from(family);
    }

    pub fn set_font_size(&mut self, size: f32) {
        self.style_bucket.font_size = size;
    }

    pub fn set_font_bucket(&mut self, bucket: &'a mut impl FontBucket) {
        self.font_bucket = Some(bucket)
    }

    pub fn load_font(&mut self, buf: &[u8]) -> Option<()> {
        self.font_bucket.as_mut().and_then(|fb| fb.load_font(buf))
    }

    pub fn set_stroke_style(&mut self, paint: Paint) {
        self.style_bucket.stroke = paint
    }

    pub fn set_fill_style(&mut self, paint: Paint) {
        self.style_bucket.fill = paint
    }

    pub fn set_line_width(&mut self, width: f32) {
        self.style_bucket.line_width = width
    }

    pub fn set_line_cap(&mut self, line_cap: LineCap) {
        self.style_bucket.line_cap = line_cap
    }

    pub fn set_line_join(&mut self, line_join: LineJoin) {
        self.style_bucket.line_join = line_join
    }

    pub fn set_miter_limit(&mut self, miter_limit: f32) {
        self.style_bucket.miter_limit = miter_limit
    }

    pub fn set_line_dash_offset(&mut self, line_dash_offset: Option<StrokeDash>) {
        self.style_bucket.line_dash_offset = line_dash_offset
    }

    pub fn arc(&mut self, x: f32, y: f32, radius: f32, start_angle: f32, end_angle: f32, anticlockwise: bool) {
        self.path_cache.arc(x, y, radius, start_angle, end_angle, anticlockwise, &self.style_bucket.transform);
    }
    pub fn arc_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32) {
        self.path_cache.arc_to(x1, y1, x2, y2, &self.style_bucket.transform);
    }
    pub fn begin_path(&mut self) {
        self.path_cache.clear();
    }
    pub fn bezier_curve_to(&mut self, cp1x: f32, cp1y: f32, cp2x: f32, cp2y: f32, x: f32, y: f32) {
        self.path_cache.bezier_curve_to(cp1x, cp1y, cp2x, cp2y, x, y, &self.style_bucket.transform);
    }
    pub fn clear_rect(&mut self, x: f32, y: f32, width: f32, height: f32) {
        self.operate_queue.append_clear_rect(x, y, width, height, &self.style_bucket.transform, self.width, self.height);
    }
    pub fn clip(&mut self, path: Option<PathData>, fill_rule: Option<FillRule>) {
        let path = path.unwrap_or(self.path_cache.clone().into_path_data());
        self.style_bucket.set_clip(path, fill_rule);
    }
    pub fn close_path(&mut self) {
        self.path_cache.close();
    }
    pub fn create_conic_gradient() {
        todo!()
    }
    pub fn create_image_data() {
        todo!()
    }
    pub fn create_linear_gradient() {
        todo!()
    }
    pub fn create_pattern() {
        todo!()
    }
    pub fn create_radial_gradient() {
        todo!()
    }
    pub fn draw_image(&mut self) {
        todo!()
    }
    pub fn ellipse(&mut self) {
        todo!()
    }
    pub fn fill(&mut self, path: Option<PathData>, fill_rule: Option<FillRule>) {
        let path = path.unwrap_or(self.path_cache.clone().into_path_data());
        let fill_rule = fill_rule.unwrap_or(FillRule::Nonzero);
        self._fill(path, self.style_bucket.get_fill(), fill_rule, self.style_bucket.clip_mask.clone())
    }
    pub fn fill_rect(&mut self, x: f32, y: f32, width: f32, height: f32) {
        PathData::create_rect(x, y, width, height)
            .and_then(|rect| Some(self.fill(Some(rect.transform_to(self.style_bucket.transform.clone())), None)));
    }
    pub fn fill_text<'b>(&mut self, text: &str, x: f32, y: f32, max_width: Option<f32>, mut font_bucket: Option<&'b mut dyn FontBucket>) -> Option<(PathData)> {
        let Glyph { mut path, fill_rule, transform } = {
            if self.font_bucket.is_some() {
                self.font_bucket.as_mut().and_then(|mut fb| fb.get_glyph(&self.style_bucket.get_font_style(), text))
            } else if font_bucket.is_some() {
                font_bucket.as_mut().and_then(|mut fb| fb.get_glyph(&self.style_bucket.get_font_style(), text))
            } else {
                None
            }
        }?;
        path.transform(transform);
        path.transform(Transform::new_translate(x, y));
        self.fill(Some(path.clone()), Some(fill_rule));
        Some((path))
    }
    pub fn get_context_attributes(&self) -> &StyleBucket {
        &self.style_bucket
    }
    pub fn get_image_data(&self) {
        todo!()
    }
    pub fn get_line_dash(&self) -> Option<StrokeDash> {
        self.style_bucket.line_dash_offset.clone()
    }
    pub fn get_transform(&self) -> Transform {
        self.style_bucket.transform.clone()
    }
    pub fn line_to(&mut self, x: f32, y: f32) {
        self.path_cache.line_to(x, y, &self.style_bucket.transform)
    }
    pub fn measure_text() {
        todo!()
    }
    pub fn move_to(&mut self, x: f32, y: f32) {
        self.path_cache.move_to(x, y, &self.style_bucket.transform)
    }
    pub fn put_image_data(&mut self) {
        todo!()
    }
    pub fn quadratic_curve_to(&mut self, cpx: f32, cpy: f32, x: f32, y: f32) {
        self.path_cache.quadratic_curve_to(cpx, cpy, x, y, &self.style_bucket.transform)
    }
    pub fn rect(&mut self, x: f32, y: f32, width: f32, height: f32) -> Option<()> {
        let mut path = PathData::create_rect(x, y, width, height)?;
        path.transform(self.style_bucket.transform.clone());
        self.path_cache.append_path(&mut path);
        Some(())
    }
    pub fn reset_transform(&mut self) {
        self.set_transform(&Transform::default());
    }
    pub fn restore(&mut self) {
        self.style_bucket.restore();
    }
    pub fn rotate(&mut self, angle: f32) {
        self.transform(&Transform::new_rotate(angle))
    }
    pub fn save(&mut self) {
        self.style_bucket.save();
    }
    pub fn set_line_dash(&mut self, line_dash: Option<StrokeDash>) {
        self.style_bucket.line_dash_offset = line_dash;
    }
    pub fn set_transform(&mut self, ts: &Transform) {
        self.style_bucket.set_transform(ts)
    }
    pub fn stroke(&mut self, path: Option<PathData>) {
        let path = path.unwrap_or(self.path_cache.clone().into_path_data());
        self._stroke(path, self.style_bucket.get_stroke(), self.style_bucket.clip_mask.clone())
    }
    pub fn stroke_rect(&mut self, x: f32, y: f32, width: f32, height: f32) {
        PathData::create_rect(x, y, width, height)
            .and_then(|rect| Some(self.stroke(Some(rect.transform_to(self.style_bucket.transform.clone())))));
    }
    pub fn stroke_text<'b>(&mut self, text: &str, x: f32, y: f32, max_width: Option<f32>, mut font_bucket: Option<&'b mut dyn FontBucket>) -> Option<()> {
        let Glyph { mut path, fill_rule, transform } = {
            if self.font_bucket.is_some() {
                self.font_bucket.as_mut().and_then(|mut fb| fb.get_glyph(&self.style_bucket.get_font_style(), text))
            } else if font_bucket.is_some() {
                font_bucket.as_mut().and_then(|mut fb| fb.get_glyph(&self.style_bucket.get_font_style(), text))
            } else {
                None
            }
        }?;
        path.transform(transform);
        path.transform(Transform::new_translate(x, y));
        self.stroke(Some(path));
        Some(())
    }
    pub fn transform(&mut self, ts: &Transform) {
        self.style_bucket.transform(ts);
    }
    pub fn translate(&mut self, x: f32, y: f32) {
        self.style_bucket.transform(&Transform::new_translate(x, y))
    }

    fn _stroke(&mut self, path: PathData, stroke: Stroke, clip_mask: Option<ClipMask>) {
        self.operate_queue.append()
            .vector()
            .set_path(path)
            .set_stroke(stroke)
            .set_clip(clip_mask)
            .finish();
    }

    fn _fill(&mut self, path: PathData, fill: Paint, fill_rule: FillRule, clip_mask: Option<ClipMask>) {
        self.operate_queue.append()
            .vector()
            .set_path(path)
            .set_fill(fill)
            .set_fill_rule(fill_rule)
            .set_clip(clip_mask)
            .finish();
    }

    // fn _text(&mut self, )

    pub fn render(&self, mut backend: Box<dyn PainterBackend>) -> Vec<u8> {
        for seg in (&self.operate_queue).iter() {
            backend.as_mut().draw(seg);
        }
        backend.as_mut().finish()
    }
}

impl Debug for Context<'static> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Context")
            .field("style_bucket", &self.style_bucket)
            .field("path_cache", &self.path_cache)
            .field("operate_queue", &self.operate_queue)
            .finish()
    }
}

#[cfg(test)]
mod test {
    use crate::context2d::Context;
    use crate::transform::Transform;

    #[test]
    fn test() {
        let ctx = {
            let mut ctx = Context::new();
            let ts = Transform::new_translate(1.0, 1.0);
            ctx.transform(&ts);
            ctx
        };
        println!("{:?}", &ctx);
    }
}