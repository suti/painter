#[derive(Clone, Debug)]
pub enum BlendMode {
    SourceOver
}

impl Default for BlendMode {
    fn default() -> Self {
        BlendMode::SourceOver
    }
}