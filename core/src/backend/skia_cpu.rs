use skia::{Paint, Path, PathBuilder, Color, FillRule, ClipMask, Stroke, PixmapPaint, Transform};
use crate::backend::PainterBackend;
use crate::f32x2;
use crate::operate::Segment;
use crate::paint::shader::Shader;
use crate::path::{PathData, PathSegment};
use crate::skia::Pixmap;

#[derive(Clone, Debug)]
pub struct SkiaCPURender {
    pixmap: Pixmap,
    anti_alias: bool,
    force_hq_pipeline: bool,
}

impl Default for SkiaCPURender {
    fn default() -> Self {
        SkiaCPURender {
            pixmap: Pixmap::new(20, 20).unwrap(),
            anti_alias: false,
            force_hq_pipeline: false,
        }
    }
}

impl SkiaCPURender {
    pub fn new(width: u32, height: u32) -> Self {
        SkiaCPURender {
            pixmap: Pixmap::new(width, height).unwrap(),
            anti_alias: false,
            force_hq_pipeline: false,
        }
    }

    pub fn force_hq(&mut self) {
        self.force_hq_pipeline = true;
        self.anti_alias = true;
    }

    pub fn force_lq(&mut self) {
        self.force_hq_pipeline = false;
        self.anti_alias = false;
    }

    pub fn save_png<P: AsRef<std::path::Path>>(&self, path: P) -> Option<()> {
        self.pixmap.save_png(path).ok()
    }

    pub fn fill_path(&mut self, path: &PathData, paint: &crate::paint::Paint, fill_rule: &crate::paint::FillRule, clip_mask: Option<crate::paint::ClipMask>) -> Option<()> {
        let paint = SkiaCPURender::build_paint(paint, self.anti_alias, self.force_hq_pipeline);
        let path = SkiaCPURender::build_path(path)?;
        let fill_rule = SkiaCPURender::build_fill_rule(fill_rule);
        let mut clip = ClipMask::new();
        let result = {
            SkiaCPURender::build_clip_mask(&mut clip, clip_mask, self.pixmap.width(), self.pixmap.height())
        };
        let clip_mask = result.and_then(|_| Some(&clip));
        self.pixmap.fill_path(&path, &paint, fill_rule, Default::default(), clip_mask)
    }

    pub fn stroke_path(&mut self, path: &PathData, stroke: &crate::paint::stroke::Stroke, clip_mask: Option<crate::paint::ClipMask>) -> Option<()> {
        let (paint, stroke) = SkiaCPURender::build_stroke(stroke, self.anti_alias, self.force_hq_pipeline);
        let path = SkiaCPURender::build_path(path)?;
        let mut clip = ClipMask::new();
        let result = {
            SkiaCPURender::build_clip_mask(&mut clip, clip_mask, self.pixmap.width(), self.pixmap.height())
        };
        let clip_mask = result.and_then(|_| Some(&clip));
        self.pixmap.stroke_path(&path, &paint, &stroke, Default::default(), clip_mask)
    }

    pub fn draw_pixel(&mut self, pixels: &[u8], size: f32x2, opacity: f32, transform: &crate::transform::Transform, clip_mask: Option<crate::paint::ClipMask>) -> Option<()> {
        let pix =
            {
                let mut pix = Pixmap::new(size.x() as u32, size.y() as u32)?;
                let data = pix.data_mut();
                data.clone_from_slice(&pixels);
                pix
            };
        let mut clip = ClipMask::new();
        let result = {
            SkiaCPURender::build_clip_mask(&mut clip, clip_mask, self.pixmap.width(), self.pixmap.height())
        };
        let clip_mask = result.and_then(|_| Some(&clip));
        let mut paint = PixmapPaint::default();
        paint.opacity = opacity;
        let transform = SkiaCPURender::build_transform(transform);
        self.pixmap.draw_pixmap(0, 0, pix.as_ref(), &paint, transform, clip_mask)
    }


    fn build_path(path: &PathData) -> Option<Path> {
        let mut pb = PathBuilder::new();
        for seg in &path.0 {
            match seg.clone() {
                PathSegment::MoveTo { x, y } => pb.move_to(x, y),
                PathSegment::LineTo { x, y } => pb.line_to(x, y),
                PathSegment::CurveTo { x1, y1, x2, y2, x, y } => pb.cubic_to(x1, y1, x2, y2, x, y),
                PathSegment::ClosePath => pb.close()
            }
        }
        pb.finish()
    }

    fn build_paint(paint: &crate::paint::Paint, anti_alias: bool, force_hq_pipeline: bool) -> Paint {
        let mut pt = Paint::default();
        pt.anti_alias = anti_alias;
        pt.force_hq_pipeline = force_hq_pipeline;
        // todo
        // pt.blend_mode = &paint.blend_mode;
        match &paint.shader {
            Shader::SolidColor(color) => {
                let co = Color::from_rgba(color.r.get(), color.g.get(), color.b.get(), color.a.get()).unwrap();
                pt.set_color(co);
            }
            // todo
            Shader::LinearGradient(_) => {}
            Shader::RadialGradient(_) => {}
            Shader::Pattern => {}
        }
        pt
    }

    fn build_stroke(stroke: &crate::paint::stroke::Stroke, anti_alias: bool, force_hq_pipeline: bool) -> (Paint, Stroke) {
        let paint = SkiaCPURender::build_paint(&stroke.paint, anti_alias, force_hq_pipeline);
        let mut st = Stroke::default();
        st.width = stroke.width;
        st.line_cap = stroke.line_cap;
        st.line_join = stroke.line_join;
        st.dash = stroke.dash.clone();
        st.miter_limit = stroke.miter_limit;
        (paint, st)
    }

    fn build_fill_rule(fill_rule: &crate::paint::FillRule) -> FillRule {
        match fill_rule {
            crate::paint::FillRule::Nonzero => { FillRule::Winding }
            crate::paint::FillRule::Evenodd => { FillRule::EvenOdd }
        }
    }

    fn build_clip_mask(clip: &mut ClipMask, clip_mask: Option<crate::paint::ClipMask>, width: u32, height: u32) -> Option<()> {
        let clip_mask = clip_mask?;
        let fill_rule = SkiaCPURender::build_fill_rule(&clip_mask.fill_rule);
        let path = SkiaCPURender::build_path(&clip_mask.path)?;
        clip.set_path(width, height, &path, fill_rule, false);
        Some(())
    }

    fn build_transform(ts: &crate::transform::Transform) -> Transform {
        let mut transform = Transform::default();
        transform.sx = ts.a;
        transform.kx = ts.b;
        transform.ky = ts.c;
        transform.sy = ts.d;
        transform.tx = ts.e;
        transform.ty = ts.f;
        transform
    }
}

impl PainterBackend for SkiaCPURender {
    fn resize(&mut self, size: f32x2) {
        let pixmap = {
            let pix_ref = self.pixmap.as_ref();
            let mut pix = Pixmap::new(size.x() as u32, size.y() as u32).unwrap();
            pix.draw_pixmap(0, 0, pix_ref, &Default::default(), Default::default(), None);
            pix
        };
        self.pixmap = pixmap;
    }

    fn draw(&mut self, segment: &Segment) {
        match segment {
            Segment::Pixel(ref seg) => {
                self.draw_pixel(&seg.data, seg.size, seg.opacity, &seg.transform, seg.clip.clone())
            }
            Segment::Vector(ref seg) => {
                seg.fill.as_ref().and_then(|paint| self.fill_path(&seg.path, &paint, &seg.fill_rule, seg.clip.clone()));
                seg.stroke.as_ref().and_then(|stroke| self.stroke_path(&seg.path, &stroke, seg.clip.clone()))
            }
        };
    }

    fn finish(&mut self) -> Vec<u8> {
        self.pixmap.encode_png().unwrap()
    }
}

pub struct ImageDataRender(pub SkiaCPURender);

impl PainterBackend for ImageDataRender {
    fn resize(&mut self, size: f32x2) {
        self.0.resize(size)
    }

    fn draw(&mut self, seg: &Segment) {
        self.0.draw(seg)
    }

    fn finish(&mut self) -> Vec<u8> {
        self.0.pixmap.clone().take()
    }
}