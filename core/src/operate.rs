use std::error::Error;
use std::ops::Deref;
use tiny_skia_path::{f32x2, LineCap, LineJoin, StrokeDash};
use crate::paint::blend::BlendMode;
use crate::paint::{ClipMask, FillRule, Paint};
use crate::paint::stroke::Stroke;
use crate::path::PathData;
use crate::transform::Transform;

#[derive(Clone, Debug)]
pub struct VectorSegment {
    pub path: PathData,
    pub fill_rule: FillRule,
    pub fill: Option<Paint>,
    pub stroke: Option<Stroke>,
    pub clip: Option<ClipMask>,
}

impl Default for VectorSegment {
    fn default() -> Self {
        VectorSegment {
            path: PathData::default(),
            fill_rule: FillRule::Nonzero,
            fill: None,
            stroke: None,
            clip: None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct PixelSegment {
    pub data: Vec<u8>,
    pub blend_mode: BlendMode,
    pub size: f32x2,
    pub opacity: f32,
    pub transform: Transform,
    pub clip: Option<ClipMask>,
}

impl PixelSegment {
    pub fn new(data: &[u8], size: f32x2) -> Self {
        PixelSegment {
            data: data.to_vec(),
            blend_mode: BlendMode::default(),
            size,
            opacity: 1.0,
            transform: Transform::default(),
            clip: None,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Segment {
    Vector(VectorSegment),
    Pixel(PixelSegment),
}

impl Default for Segment {
    fn default() -> Self {
        Segment::Vector(VectorSegment::default())
    }
}

#[derive(Clone, Debug)]
pub struct OperateSegment {
    inner: Segment,
}

impl Default for OperateSegment {
    fn default() -> Self {
        OperateSegment {
            inner: Segment::default()
        }
    }
}

impl Deref for OperateSegment {
    type Target = Segment;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl OperateSegment {
    pub fn detail(&self) -> String {
        format!("{:?}", self.inner)
    }

    pub fn width(&self) -> Option<f32> {
        match &self.inner {
            Segment::Vector(seg) => {
                Some(seg.path.get_bounding_box()?.get_width())
            }
            Segment::Pixel(seg) => {
                Some(seg.size.x())
            }
        }
    }

    pub fn height(&self) -> Option<f32> {
        match &self.inner {
            Segment::Vector(seg) => {
                Some(seg.path.get_bounding_box()?.get_height())
            }
            Segment::Pixel(seg) => {
                Some(seg.size.y())
            }
        }
    }

    pub fn create_vector() -> Self {
        OperateSegment::default()
    }

    pub fn create_pixel(data: &[u8], size: f32x2) -> Self {
        OperateSegment {
            inner: Segment::Pixel(PixelSegment::new(data, size))
        }
    }
}

pub struct OperateAppender<'a> {
    operates: &'a mut Operates,
    segment: Option<OperateSegment>,
}

pub struct VectorAppender<'a> {
    operates: &'a mut Operates,
    segment: VectorSegment,
}

impl<'a> VectorAppender<'a> {
    pub fn finish(&'a mut self) {
        let seg = OperateSegment {
            inner: Segment::Vector(self.segment.clone())
        };
        self.operates.queue.push(seg);
    }

    pub fn set_path(&'a mut self, path_data: PathData) -> &mut Self {
        self.segment.path = path_data;
        self
    }

    pub fn set_fill_rule(&'a mut self, fill_rule: FillRule) -> &mut Self {
        self.segment.fill_rule = fill_rule;
        self
    }

    pub fn set_fill(&'a mut self, paint: Paint) -> &mut Self {
        self.segment.fill = Some(paint);
        self
    }

    pub fn set_stroke(&'a mut self, stroke: Stroke) -> &mut Self {
        self.segment.stroke = Some(stroke);
        self
    }

    pub fn set_clip(&'a mut self, clip: Option<ClipMask>) -> &mut Self {
        self.segment.clip = clip;
        self
    }
}

pub struct PixelAppender<'a> {
    operates: &'a mut Operates,
    segment: PixelSegment,
}

impl<'a> PixelAppender<'a> {
    pub fn finish(&'a mut self) {
        let seg = OperateSegment {
            inner: Segment::Pixel(self.segment.clone())
        };
        self.operates.queue.push(seg);
    }

    pub fn set_size(&'a mut self, size: f32x2) -> &mut Self {
        self.segment.size = size;
        self
    }

    pub fn set_blend_mode(&'a mut self, blend_mode: BlendMode) -> &mut Self {
        self.segment.blend_mode = blend_mode;
        self
    }

    pub fn set_transform(&'a mut self, transform: Transform) -> &mut Self {
        self.segment.transform = transform;
        self
    }

    pub fn set_clip(&'a mut self, clip: ClipMask) -> &mut Self {
        self.segment.clip = Some(clip);
        self
    }
}

impl<'a> OperateAppender<'a> {
    pub fn vector(&'a mut self) -> VectorAppender {
        VectorAppender {
            operates: self.operates,
            segment: VectorSegment::default(),
        }
    }
    pub fn pixel(&'a mut self, data: &[u8], size: f32x2) -> PixelAppender {
        PixelAppender {
            operates: self.operates,
            segment: PixelSegment::new(data, size),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Operates {
    queue: Vec<OperateSegment>,
}

impl Deref for Operates {
    type Target = Vec<OperateSegment>;

    fn deref(&self) -> &Self::Target {
        &self.queue
    }
}

impl Default for Operates {
    fn default() -> Self {
        Operates { queue: vec![] }
    }
}

impl Operates {
    pub fn len(&self) -> usize {
        self.queue.len()
    }
    pub fn append(&mut self) -> OperateAppender {
        OperateAppender {
            operates: self,
            segment: None,
        }
    }
}

#[test]
fn test() {
    let ctx = {
        let mut ops = Operates::default();
        {
            let mut path = PathData::new();
            path.move_to(1.0, 1.0);
            path.line_to(2.0, 2.0);
            path.close();
            ops.append().vector().set_path(path).finish();
            ops.append().vector().set_path(PathData::default()).finish();
            ops
        }
    };
    println!("{:?}", &ctx);
}