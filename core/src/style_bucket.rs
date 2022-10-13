use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};
use tiny_skia_path::{LineCap, LineJoin, StrokeDash};
use crate::font::FontStyles;
use crate::paint::blend::BlendMode;
use crate::paint::color::Color;
use crate::paint::{ClipMask, FillRule, Paint};
use crate::paint::stroke::Stroke;
use crate::PathData;
use crate::transform::Transform;

#[derive(Clone, Debug)]
pub enum TextAlign {
    Left,
    Right,
    Center,
    Justify,
    Start,
    End,
}

#[derive(Clone, Debug)]
pub enum TextBaseLine {
    Top,
    Hanging,
    Middle,
    Alphabetic,
    Ideographic,
    Bottom,
}

#[derive(Clone, Debug)]
pub enum TextDirection {
    Ltr,
    Rtl,
}

#[derive(Clone, Debug)]
pub struct StyleBucket {
    pub transform: Transform,
    pub fill: Paint,
    pub stroke: Paint,
    pub line_width: f32,
    pub line_cap: LineCap,
    pub line_join: LineJoin,
    pub miter_limit: f32,
    pub line_dash_offset: Option<StrokeDash>,
    pub shadow_offset_x: f32,
    pub shadow_offset_y: f32,
    pub shadow_blur: f32,
    pub shadow_color: Color,
    pub font_family: String,
    pub font_size: f32,
    pub font_style: String,
    pub text_align: TextAlign,
    pub text_base_line: TextBaseLine,
    pub direction: TextDirection,
    pub image_smoothing_enabled: bool,
    pub global_composite_operation: BlendMode,
    pub clip_mask: Option<ClipMask>,
}

impl Default for StyleBucket {
    fn default() -> Self {
        StyleBucket {
            transform: Transform::default(),
            fill: Paint::default(),
            stroke: Paint::default(),
            line_width: 1.0,
            line_cap: LineCap::default(),
            line_join: LineJoin::default(),
            miter_limit: 4.0,
            line_dash_offset: None,
            shadow_offset_x: 0.0,
            shadow_offset_y: 0.0,
            shadow_blur: 0.0,
            shadow_color: Color::from_rgba8(0, 0, 0, 0),
            font_family: "".to_string(),
            font_size: 16.0,
            font_style: "".to_string(),
            text_align: TextAlign::Left,
            text_base_line: TextBaseLine::Alphabetic,
            direction: TextDirection::Ltr,
            image_smoothing_enabled: false,
            global_composite_operation: BlendMode::default(),
            clip_mask: None,
        }
    }
}

impl StyleBucket {
    pub fn set_transform(&mut self, ts: &Transform) {
        self.transform.clone_from(ts);
    }

    pub fn transform(&mut self, ts: &Transform) {
        self.transform.append(&ts)
    }
}

impl StyleBucket {
    pub fn get_fill(&self) -> Paint {
        self.fill.clone()
    }

    pub fn get_stroke(&self) -> Stroke {
        Stroke {
            width: self.line_width,
            miter_limit: self.miter_limit,
            line_cap: self.line_cap,
            line_join: self.line_join,
            dash: self.line_dash_offset.clone(),
            paint: self.stroke.clone(),
        }
    }

    pub fn get_font_style(&self) -> FontStyles {
        let mut fs = FontStyles::default();
        fs.size = self.font_size;
        fs.line_height = 1.2;
        fs.family = self.font_family.clone();
        fs
    }

    pub fn set_clip(&mut self, path: PathData, fill_rule: Option<FillRule>) {
        let clip_mask = ClipMask {
            path,
            fill_rule: fill_rule.unwrap_or(FillRule::Nonzero),
        };
        self.clip_mask = Some(clip_mask)
    }

    pub fn get_font(&self) {
        todo!()
    }
}

#[derive(Clone, Debug, Default)]
pub struct StyleStore {
    inner: StyleBucket,
    back: StyleBucket,
}

impl StyleStore {
    pub fn save(&mut self) {
        self.back.clone_from(&self.inner)
    }

    pub fn restore(&mut self) {
        self.inner.clone_from(&self.back)
    }
}

impl Deref for StyleStore {
    type Target = StyleBucket;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for StyleStore {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

