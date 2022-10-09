extern crate painter_core;
extern crate painter_font;

pub use painter_core::font::FontBucket;
pub use painter_core::path::BoundingBox;
pub use painter_core::PathData;
pub use painter_core::transform::Transform;
pub use crate::painter_core::font::FontStyles;
pub use crate::painter_font::Glyph;
pub use crate::painter_font::GlyphExt;

pub use crate::painter_core::backend;
pub use crate::painter_core::context2d::Context as Context2d;

#[cfg(test)]
mod test {
    use painter_font::FontDB;
    use crate::backend::skia_cpu::SkiaCPURender;
    use crate::backend::svg::SvgRender;
    use crate::{Context2d};
    use crate::painter_core::path::PathBuilder;
    use crate::painter_core::paint::Paint;
    use crate::painter_core::transform::Transform;

    fn draw(font_bucket: &mut FontDB) -> Context2d {
        let mut ctx = Context2d::new();
        ctx.set_font_bucket(font_bucket);
        ctx.set_font_family("HappyZcool-2016");
        ctx.set_font_size(160.0);
        ctx.set_stroke_style(Paint::from_color_rgba8(255, 0, 128, 80));
        ctx.set_line_width(5.0);
        ctx.set_fill_style(Paint::from_color_rgba8(0, 128, 255, 100));
        ctx.stroke_text("hello world 你好世界", 400.0, 400.0, Some(1000.0), None);
        ctx.fill_text("hello world 你好世界", 400.0, 400.0, Some(1000.0), None);

        let mut path = PathBuilder::default();
        path.begin_path();
        path.arc(1000.0, 1000.0, 800.0, 0.01, 360.0, false, &Default::default());
        path.close_path();
        ctx.clip(Some(path.into_path_data()), None);

        ctx.set_fill_style(Paint::from_color_rgba8(0, 255, 125, 50));
        ctx.fill_rect(0.0, 0.0, 2000.0, 2000.0);

        ctx.begin_path();
        ctx.move_to(800.0, 800.0);
        ctx.line_to(1200.0, 1200.0);
        ctx.line_to(400.0, 1200.0);
        ctx.close_path();

        ctx.rect(1200.0, 1200.0, 400.0, 400.0);

        ctx.set_stroke_style(Paint::from_color_rgba8(0, 255, 128, 128));
        ctx.set_line_width(80.0);
        ctx.set_fill_style(Paint::from_color_rgba8(128, 0, 255, 128));

        ctx.save();
        ctx.transform(&Transform::new_translate(100.0, 100.0));
        ctx.stroke(None);
        ctx.restore();
        ctx.fill(None, None);
        ctx.translate(250.0, 250.0);
        ctx.rotate(450.0);
        ctx.stroke_rect(200.0, 200.0, 200.0, 200.0);
        ctx.set_line_width(10.0);
        ctx.stroke_rect(400.0, 400.0, 10.0, 200.0);
        ctx.reset_transform();
        ctx.begin_path();
        ctx.set_stroke_style(Paint::from_color_rgba8(255, 128, 0, 255));
        ctx.arc(1000.0, 1000.0, 500.0, 0.0, 270.0, false);
        ctx.stroke(None);

        ctx
    }

    #[test]
    fn render() {
        let file = include_bytes!("../font/out.ttf") as &[u8];
        let mut font_db = FontDB::new();
        font_db.load_font(file);
        let ctx = draw(&mut font_db);
        let mut renderer = SkiaCPURender::new(2000, 2000);
        renderer.force_lq();
        let data = ctx.render(Box::new(renderer));
        std::fs::write("out.png", data).unwrap();
    }

    #[test]
    fn render_svg() {
        let file = include_bytes!("../font/test/zc2016.woff") as &[u8];
        let mut font_db = FontDB::new();
        font_db.load_font(file);
        let ctx = draw(&mut font_db);
        let renderer = SvgRender::new(2000.0, 2000.0);
        let data = ctx.render(Box::new(renderer));
        std::fs::write("out.svg", data).unwrap();
    }
}