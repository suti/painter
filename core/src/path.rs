use super::transform::Transform;
use kurbo;
use sk_path::Rect;
use std::f32::consts::PI;

#[derive(Clone, Copy, Debug)]
pub enum PathSegment {
    MoveTo {
        x: f32,
        y: f32,
    },
    LineTo {
        x: f32,
        y: f32,
    },
    CurveTo {
        x: f32,
        y: f32,
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
    },
    ClosePath,
}

impl From<&PathSegment> for String {
    fn from(path: &PathSegment) -> Self {
        let result = match path {
            PathSegment::MoveTo { x, y } => {
                format!("M {:?} {:?}", *x, *y)
            }
            PathSegment::LineTo { x, y } => {
                format!("L {:?} {:?}", *x, *y)
            }
            PathSegment::CurveTo { x, y, x1, y1, x2, y2 } => {
                format!("C {:?} {:?} {:?} {:?} {:?} {:?}", *x1, *y1, *x2, *y2, *x, *y)
            }
            PathSegment::ClosePath => {
                "Z".to_string()
            }
        };
        result
    }
}

impl From<&PathSegment> for Vec<f32> {
    fn from(item: &PathSegment) -> Self {
        match item {
            PathSegment::MoveTo { x, y } => {
                vec![0f32, *x, *y]
            }
            PathSegment::LineTo { x, y } => {
                vec![1f32, *x, *y]
            }
            PathSegment::CurveTo { x, y, x1, y1, x2, y2 } => {
                vec![2f32, *x1, *y1, *x2, *y2, *x, *y]
            }
            PathSegment::ClosePath => {
                vec![3f32]
            }
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct PathData(pub Vec<PathSegment>);

impl From<&PathData> for String {
    fn from(p: &PathData) -> Self {
        let mut result = String::from("");
        for segment in p.iter() {
            result.push_str(&String::from(segment));
            result.push_str(" ");
        }
        if p.len() > 0 { result.pop(); }
        result
    }
}

impl From<&PathData> for Vec<f32> {
    fn from(item: &PathData) -> Self {
        let mut result = Vec::<f32>::new();
        result.push(item.len() as f32);
        for segment in item.iter() {
            let vec: Vec<f32> = segment.into();
            for item in vec.iter() {
                result.push(*item)
            }
        }
        result
    }
}

impl PathData {
    pub fn new() -> Self {
        PathData(Vec::<PathSegment>::new())
    }

    #[inline]
    pub fn move_to(&mut self, x: f32, y: f32) {
        self.push(PathSegment::MoveTo { x, y });
    }

    #[inline]
    pub fn line_to(&mut self, x: f32, y: f32) {
        self.push(PathSegment::LineTo { x, y });
    }

    #[inline]
    pub fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        self.push(PathSegment::CurveTo { x1, y1, x2, y2, x, y });
    }

    #[inline]
    pub fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        let (prev_x, prev_y) = self.last_pos();
        self.push(quad_to_curve_seg(prev_x, prev_y, x1, y1, x, y));
    }

    pub fn arc2(&mut self, x: f32, y: f32, radius: f32, start_angle: f32, end_angle: f32, anticlockwise: bool) {
        let (sx, sy, radius, rotation, large_arc, _sweep_flag, x, y) = describe_arc(x, y, radius, start_angle, end_angle);
        self.move_to(sx, sy);

        let (prev_x, prev_y) = self.last_pos();

        let svg_arc = kurbo::SvgArc {
            from: kurbo::Point::new(prev_x as f64, prev_y as f64),
            to: kurbo::Point::new(x as f64, y as f64),
            radii: kurbo::Vec2::new(radius as f64, radius as f64),
            x_rotation: rotation.to_radians() as f64,
            large_arc,
            sweep: anticlockwise,
        };

        match kurbo::Arc::from_svg_arc(&svg_arc) {
            Some(arc) => {
                arc.to_cubic_beziers(0.1, |p1, p2, p| {
                    self.curve_to(p1.x as f32, p1.y as f32, p2.x as f32, p2.y as f32, p.x as f32, p.y as f32);
                });
            }
            None => {
                self.line_to(x, y);
            }
        }
    }

    #[inline]
    pub fn arc_to(
        &mut self,
        rx: f32, ry: f32,
        x_axis_rotation: f32,
        large_arc: bool,
        sweep: bool,
        x: f32, y: f32,
    ) {
        let (prev_x, prev_y) = self.last_pos();

        let svg_arc = kurbo::SvgArc {
            from: kurbo::Point::new(prev_x as f64, prev_y as f64),
            to: kurbo::Point::new(x as f64, y as f64),
            radii: kurbo::Vec2::new(rx as f64, ry as f64),
            x_rotation: x_axis_rotation.to_radians() as f64,
            large_arc,
            sweep,
        };

        match kurbo::Arc::from_svg_arc(&svg_arc) {
            Some(arc) => {
                arc.to_cubic_beziers(0.1, |p1, p2, p| {
                    self.curve_to(p1.x as f32, p1.y as f32, p2.x as f32, p2.y as f32, p.x as f32, p.y as f32);
                });
            }
            None => {
                self.line_to(x, y);
            }
        }
    }

    #[inline]
    pub fn close(&mut self) {
        self.push(PathSegment::ClosePath);
    }

    #[inline]
    pub fn last_pos(&self) -> (f32, f32) {
        let seg = self.last().expect("path must not be empty").clone();
        match seg {
            PathSegment::MoveTo { x, y }
            | PathSegment::LineTo { x, y }
            | PathSegment::CurveTo { x, y, .. } => {
                (x, y)
            }
            PathSegment::ClosePath => {
                panic!("the previous segment must be M/L/C")
            }
        }
    }

    #[inline]
    pub fn transform(&mut self, ts: Transform) {
        transform_path(self, ts);
    }

    pub fn transform_to(&self, ts: Transform) -> PathData {
        let mut path = self.clone();
        transform_path(&mut path, ts);
        path
    }

    #[inline]
    pub fn transform_from(&mut self, offset: usize, ts: Transform) {
        transform_path(&mut self[offset..], ts);
    }

    pub fn get_bounding_box(&self) -> Option<BoundingBox> {
        let first = self.0.get(0);
        if first.is_none() { return None; }
        let first = first.unwrap();
        let bbox = match first {
            &PathSegment::MoveTo { ref x, ref y } => { Some(BoundingBox::new(*x, *y)) }
            &PathSegment::LineTo { ref x, ref y } => { Some(BoundingBox::new(*x, *y)) }
            &PathSegment::CurveTo { x: _, y: _, ref x1, ref y1, x2: _, y2: _ } => { Some(BoundingBox::new(*x1, *y1)) }
            _ => return None
        };
        if bbox.is_none() { return None; }
        let mut bbox = bbox.unwrap();
        let mut start_x = 0f32;
        let mut start_y = 0f32;
        let mut prev_x = 0f32;
        let mut prev_y = 0f32;
        for command in self.0.iter() {
            match command {
                &PathSegment::MoveTo { ref x, ref y } => {
                    bbox.add_point(*x, *y);
                    start_x = *x;
                    prev_x = *x;
                    start_y = *y;
                    prev_y = *y;
                }
                &PathSegment::LineTo { ref x, ref y } => {
                    bbox.add_point(*x, *y);
                    prev_x = *x;
                    prev_y = *y;
                }
                &PathSegment::CurveTo { ref x, ref y, ref x1, ref y1, ref x2, ref y2 } => {
                    bbox.add_bezier(prev_x, prev_y, *x1, *y1, *x2, *y2, *x, *y);
                    prev_x = *x;
                    prev_y = *y;
                }
                &PathSegment::ClosePath => {
                    prev_x = start_x;
                    prev_y = start_y;
                }
            }
        }
        Some(bbox)
    }
}

impl PathData {
    pub fn create_rect(x: f32, y: f32, w: f32, h: f32) -> Option<PathData> {
        Rect::from_xywh(x, y, w, h).and_then(|rect| Some({
            let mut path = PathData::new();
            path.move_to(rect.left(), rect.top());
            path.line_to(rect.right(), rect.top());
            path.line_to(rect.right(), rect.bottom());
            path.line_to(rect.left(), rect.bottom());
            path.close();
            path
        }))
    }
}

impl std::ops::Deref for PathData {
    type Target = Vec<PathSegment>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for PathData {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub fn quad_to_curve(px: f32, py: f32, x1: f32, y1: f32, x: f32, y: f32) -> (f32, f32, f32, f32, f32, f32) {
    #[inline]
    fn calc(n1: f32, n2: f32) -> f32 {
        (n1 + n2 * 2.0) / 3.0
    }
    (
        calc(px, x1),
        calc(py, y1),
        calc(x, x1),
        calc(y, y1),
        x,
        y,
    )
}

#[inline]
fn quad_to_curve_seg(px: f32, py: f32, x1: f32, y1: f32, x: f32, y: f32) -> PathSegment {
    let (x1, y1, x2, y2, x, y) = quad_to_curve(px, py, x1, y1, x, y);

    PathSegment::CurveTo {
        x1,
        y1,
        x2,
        y2,
        x,
        y,
    }
}

fn transform_path(segments: &mut [PathSegment], ts: Transform) {
    for seg in segments {
        match seg {
            PathSegment::MoveTo { ref mut x, ref mut y } => {
                ts.apply_to(x, y);
            }
            PathSegment::LineTo { ref mut x, ref mut y } => {
                ts.apply_to(x, y);
            }
            PathSegment::CurveTo { ref mut x1, ref mut y1, ref mut x2, ref mut y2, ref mut x, ref mut y } => {
                ts.apply_to(x1, y1);
                ts.apply_to(x2, y2);
                ts.apply_to(x, y);
            }
            PathSegment::ClosePath => {}
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct BoundingBox {
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
}

impl BoundingBox {
    pub fn new(x: f32, y: f32) -> Self {
        BoundingBox {
            x1: x,
            y1: y,
            x2: x,
            y2: y,
        }
    }

    pub fn get_width(&self) -> f32 {
        self.x2 - self.x1
    }

    pub fn get_height(&self) -> f32 {
        self.y2 - self.y1
    }

    pub fn merge(&self, other: &BoundingBox) -> Self {
        let mut bbox = self.clone();
        if other.x1 < bbox.x1 {
            bbox.x1 = other.x1
        }
        if other.y1 < bbox.y1 {
            bbox.y1 = other.y1
        }
        if other.x2 > bbox.x2 {
            bbox.x2 = other.x2
        }
        if other.y2 > bbox.y2 {
            bbox.y2 = other.y2
        }
        bbox
    }

    pub fn extends(&mut self, width: f32) {
        self.x1 -= width;
        self.y1 -= width;
        self.x2 += width;
        self.y2 += width;
    }

    pub fn move_t(&mut self, x: f32, y: f32) {
        self.x1 += x;
        self.y1 += y;
        self.x2 += x;
        self.y2 += y;
    }

    pub fn add_point(&mut self, x: f32, y: f32) {
        if x < self.x1 {
            self.x1 = x
        }
        if x > self.x2 {
            self.x2 = x
        }
        if y < self.y1 {
            self.y1 = y
        }
        if y > self.y2 {
            self.y2 = y
        }
    }

    pub fn add_point_x(&mut self, x: f32) {
        if x < self.x1 {
            self.x1 = x
        }
        if x > self.x2 {
            self.x2 = x
        }
    }

    pub fn add_point_y(&mut self, y: f32) {
        if y < self.y1 {
            self.y1 = y
        }
        if y > self.y2 {
            self.y2 = y
        }
    }

    pub fn add_bezier(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        self.add_point(x0, y0);
        self.add_point(x, y);

        let mut compute = |p0: f32, p1: f32, p2: f32, p3: f32, i: usize| {
            let b = 6.0 * p0 - 12.0 * p1 + 6.0 * p2;
            let a = -3.0 * p0 + 9.0 * p1 - 9.0 * p2 + 3.0 * p3;
            let c = 3.0 * p1 - 3.0 * p0;

            if a == 0.0 {
                if b == 0.0 { return; }

                let t = -c / b;
                if 0.0 < t && t < 1.0 {
                    if i == 0 {
                        self.add_point_x(derive(p0, p1, p2, p3, t));
                    }

                    if i == 1 {
                        self.add_point_y(derive(p0, p1, p2, p3, t))
                    }
                }
                return;
            }

            let b2ac = b.powf(2.0) - 4.0 * c * a;
            if b2ac < 0.0 {
                return;
            }

            let t1 = (-b + b2ac.sqrt()) / (2.0 * a);
            if 0.0 < t1 && t1 < 1.0 {
                if i == 0 {
                    self.add_point_x(derive(p0, p1, p2, p3, t1));
                }

                if i == 1 {
                    self.add_point_y(derive(p0, p1, p2, p3, t1));
                }
            }
            let t2 = (-b - b2ac.sqrt()) / (2.0 * a);
            if 0.0 < t2 && t2 < 1.0 {
                if i == 0 {
                    self.add_point_x(derive(p0, p1, p2, p3, t2));
                }

                if i == 1 {
                    self.add_point_y(derive(p0, p1, p2, p3, t2));
                }
            }
        };
        compute(x0, x1, x2, x, 0);
        compute(y0, y1, y2, y, 1);
    }

    pub fn add_quad(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, x: f32, y: f32) {
        let cp1x = x0 + 2.0 / 3.0 * (x1 - x0);
        let cp1y = y0 + 2.0 / 3.0 * (y1 - y0);
        let cp2x = cp1x + 1.0 / 3.0 * (x - x0);
        let cp2y = cp1y + 1.0 / 3.0 * (y - y0);
        self.add_bezier(x0, y0, cp1x, cp1y, cp2x, cp2y, x, y);
    }
}

fn derive(v0: f32, v1: f32, v2: f32, v3: f32, t: f32) -> f32 {
    return (1.0 - t).powf(3.0) * v0 +
        3.0 * (1.0 - t).powf(2.0) * t * v1 +
        3.0 * (1.0 - t) * t.powf(2.0) * v2 +
        t.powf(3.0) * v3;
}

#[derive(Debug, Clone, Default)]
pub struct BBox {
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
}

impl BBox {
    pub fn new(x1: f64, y1: f64, x2: f64, y2: f64) -> BBox {
        BBox {
            x1,
            y1,
            x2,
            y2,
        }
    }

    pub fn get_width(&self) -> f64 {
        self.x2
    }

    pub fn get_height(&self) -> f64 {
        self.y2
    }

    pub fn get_real_width(&self) -> f64 {
        self.x2 - self.x1
    }
    pub fn get_real_height(&self) -> f64 {
        self.y2 - self.y1
    }

    pub fn compare(&self, b: &Self) -> bool {
        self.x1 * self.x2 > b.x1 * b.x2
    }
}

impl From<&BBox> for String {
    fn from(BBox { x1, y1, x2, y2 }: &BBox) -> Self {
        format!("[{},{},{},{}]", x1, y1, x2, y2)
    }
}

impl From<&BBox> for Vec<f32> {
    fn from(BBox { x1, y1, x2, y2 }: &BBox) -> Self {
        vec![*x1 as f32, *y1 as f32, *x2 as f32, *y2 as f32]
    }
}


#[derive(Debug, Clone)]
pub struct BBoxes(Vec<BBox>);

impl BBoxes {
    pub fn new() -> Self {
        BBoxes(Vec::<BBox>::new())
    }

    pub fn get_total_box(&self) -> BBox {
        let mut b_box: BBox = Default::default();
        if self.len() > 0 {
            let default_box = BBox { x1: 0.0, y1: 0.0, x2: 0.0, y2: 0.0 };
            let b_box_1 = self.get(0).unwrap_or(&default_box);
            b_box = b_box_1.clone();
            for item in self.iter() {
                if item.x1 < b_box.x1 {
                    b_box.x1 = item.x1
                }
                if item.y1 < b_box.y1 {
                    b_box.y1 = item.y1
                }
                if item.x2 > b_box.x2 {
                    b_box.x2 = item.x2
                }
                if item.y2 > b_box.y2 {
                    b_box.y2 = item.y2
                }
            }
        }
        b_box
    }
}


impl std::ops::Deref for BBoxes {
    type Target = Vec<BBox>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for BBoxes {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<&BBoxes> for String {
    fn from(boxes: &BBoxes) -> Self {
        let mut result = String::from("[");
        for segment in boxes.iter() {
            result.push_str(&String::from(segment));
            result.push_str(",");
        }
        if boxes.len() > 0 { result.pop(); }
        result.push_str("]");
        result.to_string()
    }
}

impl From<&BBoxes> for Vec<f32> {
    fn from(b_boxes: &BBoxes) -> Self {
        let mut result = Vec::<f32>::new();
        result.push(b_boxes.len() as f32);
        for b_box in b_boxes.iter() {
            let vec: Vec<f32> = b_box.into();
            for b in vec.iter() {
                result.push(*b);
            }
        }
        result
    }
}

#[derive(Default, Clone, Debug)]
pub struct PathBuilder(pub PathData);

fn polar_to_cartesian(cx: f32, cy: f32, radius: f32, angle: f32) -> (f32, f32) {
    let angle_in_radians = (angle - 90.0) * PI / 180.0;

    (cx + (radius * angle_in_radians.cos()),
     cy + (radius * angle_in_radians.sin()))
}

fn describe_arc(x: f32, y: f32, radius: f32, start_angle: f32, end_angle: f32) -> (f32, f32, f32, f32, bool, bool, f32, f32) {
    let (sx, sy) = polar_to_cartesian(x, y, radius, end_angle);
    let (ex, ey) = polar_to_cartesian(x, y, radius, start_angle);

    let large_arc_flag = if end_angle - start_angle <= 180.0 { false } else { true };

    (sx, sy, radius, 0.0, large_arc_flag, false, ex, ey)
}

impl PathBuilder {
    pub fn arc(&mut self, x: f32, y: f32, radius: f32, start_angle: f32, end_angle: f32, anticlockwise: bool, transform: &Transform) {
        let (sx, sy, radius, rotation, large_arc, _sweep_flag, x, y) = describe_arc(x, y, radius, start_angle, end_angle);
        self.move_to(sx, sy, &transform);

        let (prev_x, prev_y) = self.0.last_pos();

        let svg_arc = kurbo::SvgArc {
            from: kurbo::Point::new(prev_x as f64, prev_y as f64),
            to: kurbo::Point::new(x as f64, y as f64),
            radii: kurbo::Vec2::new(radius as f64, radius as f64),
            x_rotation: rotation.to_radians() as f64,
            large_arc,
            sweep: anticlockwise,
        };

        match kurbo::Arc::from_svg_arc(&svg_arc) {
            Some(arc) => {
                arc.to_cubic_beziers(0.1, |p1, p2, p| {
                    self.bezier_curve_to(p1.x as f32, p1.y as f32, p2.x as f32, p2.y as f32, p.x as f32, p.y as f32, &transform);
                });
            }
            None => {
                self.line_to(x, y, &transform);
            }
        }
    }
    pub fn arc_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, transform: &Transform) {
        todo!()
    }
    pub fn begin_path(&mut self) {
        self.0.0.clear();
    }
    pub fn bezier_curve_to(&mut self, cp1x: f32, cp1y: f32, cp2x: f32, cp2y: f32, x: f32, y: f32, transform: &Transform) {
        let (x1, y1) = transform.apply(cp1x, cp1y);
        let (x2, y2) = transform.apply(cp2x, cp2y);
        let (x, y) = transform.apply(x, y);
        self.0.curve_to(x1, y1, x2, y2, x, y)
    }
    pub fn close_path(&mut self) {
        self.0.close();
    }
    pub fn ellipse(&mut self) {
        todo!()
    }
    pub fn line_to(&mut self, x: f32, y: f32, transform: &Transform) {
        let (x, y) = transform.apply(x, y);
        self.0.line_to(x, y)
    }
    pub fn move_to(&mut self, x: f32, y: f32, transform: &Transform) {
        let (x, y) = transform.apply(x, y);
        self.0.move_to(x, y)
    }
    pub fn quadratic_curve_to(&mut self, cpx: f32, cpy: f32, x: f32, y: f32, transform: &Transform) {
        let (prev_x, prev_y) = self.0.last_pos();
        let (x1, y1, x2, y2, x, y) = quad_to_curve(prev_x, prev_y, cpx, cpy, x, y);
        let (x1, y1) = transform.apply(x1, y1);
        let (x2, y2) = transform.apply(x2, y2);
        let (x, y) = transform.apply(x, y);
        self.0.curve_to(x1, y1, x2, y2, x, y)
    }
    pub fn clear(&mut self) {
        self.0.clear();
    }
    pub fn close(&mut self) {
        self.0.close();
    }
    pub fn append_path(&mut self, path: &mut PathData) {
        self.0.append(path);
    }
    pub fn transform(&mut self, ts: &Transform) {
        self.0.transform(ts.clone())
    }
    pub fn into_path_data(self) -> PathData { self.0 }
}

#[test]
fn test() {
    let mut pb = PathBuilder::default();
    pb.begin_path();
    pb.arc(1000.0, 1000.0, 800.0, 0.01, 360.0, false, &Default::default());
    pb.close_path();
    let mut path = PathData::new();
    path.arc2(1000.0, 1000.0, 800.0, 0.01, 360.0, false);
    path.close();
    println!("{:?}", String::from(&pb.into_path_data()));
    println!("{:?}", String::from(&path))
}
