use crate::paint::blend::BlendMode;
use crate::paint::color::Color;
use crate::paint::shader::Shader;
use crate::PathData;

pub mod color;
pub mod shader;
pub mod blend;
pub mod stroke;

#[derive(Clone, Debug)]
pub struct Paint {
    pub shader: Shader,
    pub blend_mode: BlendMode,
    pub anti_alias: bool,
}

impl Default for Paint {
    fn default() -> Self {
        Paint {
            shader: Shader::SolidColor(Color::BLACK),
            blend_mode: BlendMode::default(),
            anti_alias: false
        }
    }
}

impl Paint {

    pub fn set_color(&mut self, color: Color) {
        self.shader = Shader::SolidColor(color);
    }

    pub fn set_color_rgba8(&mut self, r: u8, g: u8, b: u8, a: u8) {
        self.set_color(Color::from_rgba8(r, g, b, a))
    }

    pub fn is_solid_color(&self) -> bool {
        matches!(self.shader, Shader::SolidColor(_))
    }
}

impl Paint {
    pub fn from_color(color: Color) -> Self {
        let mut paint = Paint::default();
        paint.set_color(color);
        paint
    }

    pub fn from_color_rgba8(r: u8, g: u8, b: u8, a: u8) -> Self {
        let mut paint = Paint::default();
        paint.set_color_rgba8(r, g, b, a);
        paint
    }
}

#[derive(Clone, Copy, Debug)]
pub enum FillRule {
    Nonzero,
    Evenodd,
}

impl Default for FillRule {
    fn default() -> Self {
        FillRule::Nonzero
    }
}

impl From<FillRule> for String {
    fn from(v: FillRule) -> Self {
        match v {
            FillRule::Nonzero => { String::from("nonzero") }
            FillRule::Evenodd => { String::from("evenodd") }
        }
    }
}

#[derive(Clone, Debug)]
pub struct ClipMask {
    pub path: PathData,
    pub fill_rule: FillRule,
}


