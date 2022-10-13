use std::convert::TryFrom;
use std::num::NonZeroU16;

use fontdb::{ID, Database, Query, Family};
use ttf_parser::{Rect, Style};
use painter_core::transform::Transform;

use crate::painter_core::path::PathData;
use crate::painter_core::path::BoundingBox;

#[derive(Debug, Clone)]
pub struct Glyph {
    family: String,

    units_per_em: NonZeroU16,

    // All values below are in font units.
    ascent: i16,
    descent: i16,
    x_height: NonZeroU16,

    underline_position: i16,
    underline_thickness: NonZeroU16,

    // line-through thickness should be the the same as underline thickness
    // according to the TrueType spec:
    // https://docs.microsoft.com/en-us/typography/opentype/spec/os2#ystrikeoutsize
    line_through_position: i16,

    subscript_offset: i16,
    superscript_offset: i16,

    hor_side_bearing: i16,
    ver_side_bearing: i16,

    hor_advance: u16,
    ver_advance: u16,

    italic_angle: Option<f32>,
    style: Style,

    bbox: BoundingBox,

    path: PathData,
}

impl Default for Glyph {
    fn default() -> Self {
        Glyph {
            family: "".to_string(),
            units_per_em: NonZeroU16::new(1000u16).unwrap(),
            ascent: 0,
            descent: 0,
            x_height: NonZeroU16::new(100u16).unwrap(),
            underline_position: 0,
            underline_thickness: NonZeroU16::new(2u16).unwrap(),
            line_through_position: 0,
            subscript_offset: 0,
            superscript_offset: 0,
            hor_side_bearing: 0,
            ver_side_bearing: 0,
            hor_advance: 0,
            ver_advance: 0,
            italic_angle: None,
            style: Default::default(),
            bbox: BoundingBox::new(0.0, 0.0),
            path: Default::default(),
        }
    }
}


pub trait GlyphExt {
    fn glyph<P, T>(&self, family: &str, c: char, f: P) -> Option<T> where P: FnOnce(Glyph) -> T;
    fn find_font_id(&self, family: &str) -> Option<ID>;
}

impl GlyphExt for Database {
    fn glyph<P, T>(&self, family: &str, c: char, f: P) -> Option<T> where P: FnOnce(Glyph) -> T {
        let q = Query {
            families: &[Family::Name(family)],
            weight: Default::default(),
            stretch: Default::default(),
            style: Default::default(),
        };
        let id = self.query(&q)?;
        self.with_face_data(id, |data, face_index| {
            let font = ttf_parser::Face::parse(data, face_index).ok()?;
            let glyph_id = font.glyph_index(c)?;
            let pixels_per_em = font.units_per_em();
            let mut builder = PathBuilder { path: PathData::new() };
            font.outline_glyph(glyph_id, &mut builder);
            // todo: raster_image_impl
            let raster_image = font.glyph_raster_image(glyph_id, pixels_per_em);
            let hor_side_bearing = font.glyph_hor_side_bearing(glyph_id).unwrap_or(0i16);
            let ver_side_bearing = font.glyph_ver_side_bearing(glyph_id).unwrap_or(0i16);
            let hor_advance = font.glyph_hor_advance(glyph_id).unwrap_or(0u16);
            let ver_advance = font.glyph_ver_advance(glyph_id).unwrap_or(0u16);
            let italic_angle = font.italic_angle();
            let style = font.style();
            let bbox = font.glyph_bounding_box(glyph_id).and_then(|rect| {
                let Rect { x_min: x1, y_min: y1, x_max: x2, y_max: y2 } = rect;
                Some(BoundingBox {
                    x1: x1 as f32,
                    y1: y2 as f32,
                    x2: x2 as f32,
                    y2: y2 as f32,
                })
            }).unwrap_or(BoundingBox::new(0.0, 0.0));

            let units_per_em = NonZeroU16::new(font.units_per_em())?;

            let ascent = font.ascender();
            let descent = font.descender();

            let x_height = font.x_height().and_then(|x| u16::try_from(x).ok()).and_then(NonZeroU16::new);
            let x_height = match x_height {
                Some(height) => height,
                None => {
                    // If not set - fallback to height * 45%.
                    // 45% is what Firefox uses.
                    u16::try_from((f32::from(ascent - descent) * 0.45) as i32).ok()
                        .and_then(NonZeroU16::new)?
                }
            };

            let line_through = font.strikeout_metrics();
            let line_through_position = match line_through {
                Some(metrics) => metrics.position,
                None => x_height.get() as i16 / 2,
            };

            let (underline_position, underline_thickness) = match font.underline_metrics() {
                Some(metrics) => {
                    let thickness = u16::try_from(metrics.thickness).ok()
                        .and_then(NonZeroU16::new)
                        // `ttf_parser` guarantees that units_per_em is >= 16
                        .unwrap_or_else(|| NonZeroU16::new(units_per_em.get() / 12).unwrap());

                    (metrics.position, thickness)
                }
                None => {
                    (
                        -(units_per_em.get() as i16) / 9,
                        NonZeroU16::new(units_per_em.get() / 12).unwrap(),
                    )
                }
            };

            // 0.2 and 0.4 are generic offsets used by some applications (Inkscape/librsvg).
            let mut subscript_offset = (units_per_em.get() as f32 / 0.2).round() as i16;
            let mut superscript_offset = (units_per_em.get() as f32 / 0.4).round() as i16;
            if let Some(metrics) = font.subscript_metrics() {
                subscript_offset = metrics.y_offset;
            }

            if let Some(metrics) = font.superscript_metrics() {
                superscript_offset = metrics.y_offset;
            }

            let path = builder.path;
            Some(f(Glyph {
                family: String::from(family),
                units_per_em,
                ascent,
                descent,
                x_height,
                underline_position,
                underline_thickness,
                line_through_position,
                subscript_offset,
                superscript_offset,
                hor_side_bearing,
                ver_side_bearing,
                hor_advance,
                ver_advance,
                italic_angle,
                style,
                bbox,
                path,
            }))
        })?
    }

    fn find_font_id(&self, family: &str) -> Option<ID> {
        let q = Query {
            families: &[Family::Name(family)],
            weight: Default::default(),
            stretch: Default::default(),
            style: Default::default(),
        };
        self.query(&q)
    }
}

impl Glyph {
    #[inline]
    pub fn scale(&self, font_size: f32) -> f32 {
        font_size / self.units_per_em.get() as f32
    }

    #[inline]
    pub fn ascent(&self, font_size: f32) -> f32 {
        self.ascent as f32 * self.scale(font_size)
    }

    #[inline]
    pub fn descent(&self, font_size: f32) -> f32 {
        self.descent as f32 * self.scale(font_size)
    }

    #[inline]
    pub fn height(&self, font_size: f32) -> f32 {
        self.ascent(font_size) - self.descent(font_size)
    }

    #[inline]
    pub fn x_height(&self, font_size: f32) -> f32 {
        self.x_height.get() as f32 * self.scale(font_size)
    }

    #[inline]
    pub fn underline_position(&self, font_size: f32) -> f32 {
        self.underline_position as f32 * self.scale(font_size)
    }

    #[inline]
    pub fn underline_thickness(&self, font_size: f32) -> f32 {
        self.underline_thickness.get() as f32 * self.scale(font_size)
    }

    #[inline]
    pub fn line_through_position(&self, font_size: f32) -> f32 {
        self.line_through_position as f32 * self.scale(font_size)
    }

    #[inline]
    pub fn subscript_offset(&self, font_size: f32) -> f32 {
        self.subscript_offset as f32 * self.scale(font_size)
    }

    #[inline]
    pub fn superscript_offset(&self, font_size: f32) -> f32 {
        self.superscript_offset as f32 * self.scale(font_size)
    }

    #[inline]
    pub fn path(&self, font_size: f32) -> PathData {
        let scale = self.scale(font_size);
        let ts = Transform::new_scale(scale, -scale);
        self.path.transform_to(ts)
    }

    pub fn advance_width(&self, font_size: f32) -> f32 {
        self.hor_advance as f32 * self.scale(font_size)
    }

    pub fn advance_height(&self, font_size: f32) -> f32 {
        if self.ver_advance == 0 {
            if self.hor_advance == 0 {
                0.0
            } else {
                (self.ascent - self.descent) as f32 * self.scale(font_size)
            }
        } else {
            self.ver_advance as f32 * self.scale(font_size)
        }
    }
}


struct PathBuilder {
    path: PathData,
}

impl ttf_parser::OutlineBuilder for PathBuilder {
    fn move_to(&mut self, x: f32, y: f32) {
        self.path.move_to(x, y);
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.path.line_to(x, y);
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        self.path.quad_to(
            x1, y1,
            x, y,
        );
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        self.path.curve_to(
            x1, y1,
            x2, y2,
            x, y,
        );
    }

    fn close(&mut self) {
        self.path.close();
    }
}

#[cfg(test)]
mod test {
    use fontdb::{Database, Family, Query};
    use crate::parser::GlyphExt;

    #[test]
    fn font_build() {
        let file = include_bytes!("../test/zc2016.woff") as &[u8];
        let mut db = Database::new();
        db.load_font_data(file.to_vec());
        let fonts = db.faces();
        let query = Query {
            families: &[Family::Name("HappyZcool-2016")],
            weight: Default::default(),
            stretch: Default::default(),
            style: Default::default(),
        };
    }


    #[test]
    fn g() {
        let file = include_bytes!("../test/zc2016.woff") as &[u8];
        let mut db = Database::new();
        db.load_font_data(file.to_vec());
        let g = db.glyph("HappyZcool-2016", "家".chars().next().unwrap(), |g| g);
        println!("{:?}", g)
    }
}
