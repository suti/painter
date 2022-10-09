use crate::paint::color::Color;

#[derive(Clone, Debug)]
pub enum Shader {
    /// A solid color shader.
    SolidColor(Color),
    /// A linear gradient shader.
    LinearGradient(LinearGradient),
    /// A radial gradient shader.
    RadialGradient(RadialGradient),
    /// A pattern shader.
    Pattern,
}

pub struct Pattern<'a> {
    pub data: &'a [u8]
}

#[derive(Clone, Debug)]
pub struct LinearGradient {

}

#[derive(Clone, Debug)]
pub struct RadialGradient {

}