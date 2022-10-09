use std::io;
use skia_safe::{Paint, Path, PathBuilder, Color, Surface, ClipMask, Stroke, PixmapPaint, Transform, Pixmap, EncodedImageFormat};
use crate::painter_core::backend::PainterBackend;
use crate::painter_core::f32x2;
use crate::painter_core::operate::Segment;
use crate::painter_core::paint::shader::Shader;
use crate::painter_core::path::{PathData, PathSegment};

#[derive(Debug)]
pub struct SkiaRender {
    surface: Surface,
}


impl SkiaRender {
    pub fn new(width: u32, height: u32) -> Self {
        SkiaRender {
            surface: Surface::new_raster_n32_premul((width as i32, height as i32)).expect("surface create failed")
        }
    }


    pub fn fill_path(&mut self, path: &PathData, paint: &painter_core::paint::Paint, fill_rule: &painter_core::paint::FillRule, clip_mask: Option<painter_core::paint::ClipMask>) -> Option<()> {
        let paint = SkiaRender::build_paint(paint);
        let path = SkiaRender::build_path(path)?;
        let fill_rule = SkiaRender::build_fill_rule(fill_rule);
        let mut clip = ClipMask::new();
        let result = {
            SkiaRender::build_clip_mask(&mut clip, clip_mask, self.pixmap.width(), self.pixmap.height())
        };
        let clip_mask = result.and_then(|_| Some(&clip));
        // self.surface.canvas().clip_path();
        self.surface.canvas().draw_path(&path, &paint);
        Some(())
    }

    pub fn stroke_path(&mut self, path: &PathData, stroke: &painter_core::paint::stroke::Stroke, clip_mask: Option<painter_core::paint::ClipMask>) -> Option<()> {
        let (paint, stroke) = SkiaRender::build_stroke(stroke, self.anti_alias, self.force_hq_pipeline);
        let path = SkiaRender::build_path(path)?;
        let mut clip = ClipMask::new();
        let result = {
            SkiaRender::build_clip_mask(&mut clip, clip_mask, self.pixmap.width(), self.pixmap.height())
        };
        let clip_mask = result.and_then(|_| Some(&clip));
        self.pixmap.stroke_path(&path, &paint, &stroke, Default::default(), clip_mask)
    }

    pub fn draw_pixel(&mut self, pixels: &[u8], size: f32x2, opacity: f32, transform: &painter_core::transform::Transform, clip_mask: Option<painter_core::paint::ClipMask>) -> Option<()> {
        let pix =
            {
                let mut pix = Pixmap::new(size.x() as u32, size.y() as u32)?;
                let data = pix.data_mut();
                data.clone_from_slice(&pixels);
                pix
            };
        let mut clip = ClipMask::new();
        let result = {
            SkiaRender::build_clip_mask(&mut clip, clip_mask, self.pixmap.width(), self.pixmap.height())
        };
        let clip_mask = result.and_then(|_| Some(&clip));
        let mut paint = PixmapPaint::default();
        paint.opacity = opacity;
        let transform = SkiaRender::build_transform(transform);
        self.pixmap.draw_pixmap(0, 0, pix.as_ref(), &paint, transform, clip_mask)
    }


    fn build_path(path: &PathData) -> Option<Path> {
        let mut pb = PathBuilder::new();
        for seg in &path.0 {
            match seg.clone() {
                PathSegment::MoveTo { x, y } => {
                    pb.move_to((x, y));
                },
                PathSegment::LineTo { x, y } => {
                    pb.line_to((x, y));
                },
                PathSegment::CurveTo { x1, y1, x2, y2, x, y } =>{
                    pb.cubic_to((x1, y1), (x2, y2), (x, y));
                } ,
                PathSegment::ClosePath => {
                    pb.close();
                }
            }
        }
        Some(pb.detach())
    }

    fn build_paint(paint: &painter_core::paint::Paint) -> Paint {
        let mut pt = Paint::default();
        // todo
        // pt.blend_mode = &paint.blend_mode;
        match &paint.shader {
            Shader::SolidColor(color) => {
                let color_u8 = color.to_color_u8();
                let co = Color::from_argb(color_u8.alpha(), color_u8.red(), color_u8.green(), color_u8.blue());
                pt.set_color(co);
            }
            // todo
            Shader::LinearGradient(_) => {}
            Shader::RadialGradient(_) => {}
            Shader::Pattern => {}
        }
        pt.set_mask_filter();
        pt
    }

    fn build_stroke(stroke: &painter_core::paint::stroke::Stroke, anti_alias: bool, force_hq_pipeline: bool) -> (Paint, Stroke) {
        let paint = SkiaRender::build_paint(&stroke.paint, anti_alias, force_hq_pipeline);
        let mut st = Stroke::default();
        st.width = stroke.width;
        st.line_cap = stroke.line_cap;
        st.line_join = stroke.line_join;
        st.dash = stroke.dash.clone();
        st.miter_limit = stroke.miter_limit;
        (paint, st)
    }

    fn build_fill_rule(fill_rule: &painter_core::paint::FillRule) -> FillRule {
        match fill_rule {
            painter_core::paint::FillRule::Nonzero => { FillRule::Winding }
            painter_core::paint::FillRule::Evenodd => { FillRule::EvenOdd }
        }
    }

    fn build_clip_mask(clip: &mut ClipMask, clip_mask: Option<painter_core::paint::ClipMask>, width: u32, height: u32) -> Option<()> {
        let clip_mask = clip_mask?;
        let fill_rule = SkiaRender::build_fill_rule(&clip_mask.fill_rule);
        let path = SkiaRender::build_path(&clip_mask.path)?;
        clip.set_path(width, height, &path, fill_rule, false);
        Some(())
    }

    fn build_transform(ts: &painter_core::transform::Transform) -> Transform {
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

impl PainterBackend for SkiaRender {
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
        println!("finished!");
        self.pixmap.encode_png().unwrap()
    }
}