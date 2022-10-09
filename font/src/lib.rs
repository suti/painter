extern crate ttf_parser;
extern crate fontdb;
extern crate painter_core;

pub mod woff;
pub mod parser;

pub use parser::{Glyph, GlyphExt};
pub use fontdb::*;
use painter_core::font::{FontBucket, FontStyles};
use painter_core::path::BoundingBox;
use painter_core::PathData;
use painter_core::transform::Transform;

pub mod check {
    //
    const SFNT_VERSION_TRUE_TYPE1: u32 = 0x00010000;
    // true
    const SFNT_VERSION_TRUE_TYPE2: u32 = 0x74727565;
    // typ1
    const SFNT_VERSION_TRUE_TYPE3: u32 = 0x74797031;
    // OTTO
    const SFNT_VERSION_OPEN_TYPE: u32 = 0x4F54544F;
    // wOFF
    const SFNT_VERSION_WOFF: u32 = 0x774f4646;

    fn get_data(data: &[u8], offset: usize) -> Option<u32> {
        let r = data.get(offset..offset + 4)?;
        Some(u32::from_be_bytes([r[0], r[1], r[2], r[3]]))
    }

    pub fn check_type(data: &[u8]) -> Option<(String, bool)> {
        let signature = get_data(data, 0)?;
        if signature == SFNT_VERSION_TRUE_TYPE1 || signature == SFNT_VERSION_TRUE_TYPE2 || signature == SFNT_VERSION_TRUE_TYPE3 {
            Some(("ttf".to_string(), false))
        } else if signature == SFNT_VERSION_OPEN_TYPE {
            Some(("otf".to_string(), false))
        } else if signature == SFNT_VERSION_WOFF {
            let tag = get_data(data, 4)?;
            if tag == SFNT_VERSION_TRUE_TYPE1 {
                Some(("ttf".to_string(), true))
            } else if tag == SFNT_VERSION_OPEN_TYPE {
                Some(("otf".to_string(), true))
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[derive(Clone, Debug)]
pub struct FontDB {
    db: Database,
}

impl FontDB {
    pub fn new() -> Self {
        FontDB {
            db: Database::new()
        }
    }

    pub fn load_font(&mut self, buf: &[u8]) -> Option<()> {
        let (_, need_decompress) = check::check_type(buf)?;
        let buf = if need_decompress {
            woff::decompress_woff(buf)?
        } else {
            buf.to_vec()
        };
        self.db.load_font_data(buf);
        Some(())
    }

    pub fn glyph(&self, style: &FontStyles, text: &str) -> Option<Vec<Glyph>> {
        let mut chars = text.chars();
        let mut result = vec![];
        while let Some(c) = chars.next() {
            result.push(self.db.glyph(&style.family, c, |g| g)?);
        }
        Some(result)
    }

    pub fn typeset() {}
}

impl FontBucket for FontDB {
    fn default_glyph(&self, style: &FontStyles) -> painter_core::font::Glyph {
        todo!()
    }

    fn get_glyph(&mut self, style: &FontStyles, text: &str) -> Option<painter_core::font::Glyph> {
        let list = self.glyph(style, text)?;
        let mut path_data = PathData::new();
        for g in list.iter() {
            let mut path = g.path(style.size);
            let width = path_data.get_bounding_box().unwrap_or(BoundingBox::new(0.0, 0.0)).get_width();
            path.transform(Transform::new_translate(width, 0.0));
            path_data.append(&mut path);
        }
        Some(painter_core::font::Glyph {
            path: path_data,
            fill_rule: Default::default(),
            transform: Default::default(),
        })
    }

    fn load_font(&mut self, buf: &[u8]) -> Option<()> {
        self.load_font(buf)?;
        Some(())
    }
}
