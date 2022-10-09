use std::error::Error;
use tiny_skia_path::f32x2;
use crate::operate::Segment;

pub trait PainterBackend {
    fn resize(&mut self, size: f32x2);
    fn draw(&mut self, seg: &Segment);
    fn finish(&mut self) -> Vec<u8>;
}

pub mod svg;
pub mod skia_cpu;