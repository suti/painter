use std::f32;

#[derive(Clone, Copy, PartialEq, Debug)]
#[allow(missing_docs)]
pub struct Transform {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
    pub e: f32,
    pub f: f32,
}

impl Transform {
    /// Constructs a new transform.
    pub fn new(a: f32, b: f32, c: f32, d: f32, e: f32, f: f32) -> Self {
        Transform { a, b, c, d, e, f, }
    }

    /// Constructs a new translate transform.
    pub fn new_translate(x: f32, y: f32) -> Self {
        Transform::new(1.0, 0.0, 0.0, 1.0, x, y)
    }

    /// Constructs a new scale transform.
    pub fn new_scale(sx: f32, sy: f32) -> Self {
        Transform::new(sx, 0.0, 0.0, sy, 0.0, 0.0)
    }

    /// Constructs a new rotate transform.
    pub fn new_rotate(angle: f32) -> Self {
        let v = angle / 180.0f32 * (f32::consts::PI as f32);
        let a =  v.cos();
        let b =  v.sin();
        let c = -b;
        let d =  a;
        Transform::new(a, b, c, d, 0.0, 0.0)
    }

    /// Constructs a new rotate transform at the specified position.
    pub fn new_rotate_at(angle: f32, x: f32, y: f32) -> Self {
        let mut ts = Self::default();
        ts.translate(x, y);
        ts.rotate(angle);
        ts.translate(-x, -y);
        ts
    }

    /// Constructs a new skew transform along then X axis.
    pub fn new_skew_x(angle: f32) -> Self {
        let c = ((angle / 180.0) * (f32::consts::PI as f32)).tan();
        Transform::new(1.0, 0.0, c, 1.0, 0.0, 0.0)
    }

    /// Constructs a new skew transform along then Y axis.
    pub fn new_skew_y(angle: f32) -> Self {
        let b = ((angle / 180.0) * (f32::consts::PI as f32)).tan();
        Transform::new(1.0, b, 0.0, 1.0, 0.0, 0.0)
    }

    /// Translates the current transform.
    pub fn translate(&mut self, x: f32, y: f32) {
        self.append(&Transform::new_translate(x, y));
    }

    /// Scales the current transform.
    pub fn scale(&mut self, sx: f32, sy: f32) {
        self.append(&Transform::new_scale(sx, sy));
    }

    /// Rotates the current transform.
    pub fn rotate(&mut self, angle: f32) {
        self.append(&Transform::new_rotate(angle));
    }

    /// Rotates the current transform at the specified position.
    pub fn rotate_at(&mut self, angle: f32, x: f32, y: f32) {
        self.translate(x, y);
        self.rotate(angle);
        self.translate(-x, -y);
    }

    /// Skews the current transform along the X axis.
    pub fn skew_x(&mut self, angle: f32) {
        self.append(&Transform::new_skew_x(angle));
    }

    /// Skews the current transform along the Y axis.
    pub fn skew_y(&mut self, angle: f32) {
        self.append(&Transform::new_skew_y(angle));
    }

    /// Appends transform to the current transform.
    pub fn append(&mut self, other: &Transform) {
        let ts = multiply(self, other);
        self.a = ts.a;
        self.b = ts.b;
        self.c = ts.c;
        self.d = ts.d;
        self.e = ts.e;
        self.f = ts.f;
    }

    /// Prepends transform to the current transform.
    pub fn prepend(&mut self, other: &Transform) {
        let ts = multiply(other, self);
        self.a = ts.a;
        self.b = ts.b;
        self.c = ts.c;
        self.d = ts.d;
        self.e = ts.e;
        self.f = ts.f;
    }

    /// Returns transform's translate part.
    pub fn get_translate(&self) -> (f32, f32) {
        (self.e, self.f)
    }

    /// Returns transform's scale part.
    pub fn get_scale(&self) -> (f32, f32) {
        let x_scale = (self.a * self.a + self.c * self.c).sqrt();
        let y_scale = (self.b * self.b + self.d * self.d).sqrt();
        (x_scale, y_scale)
    }

    /// Returns transform's skew part.
    pub fn get_skew(&self) -> (f32, f32) {
        let rad = 180.0 / (f32::consts::PI as f32);
        let skew_x = rad * (self.d).atan2(self.c) - 90.0;
        let skew_y = rad * (self.b).atan2(self.a);
        (skew_x, skew_y)
    }

    /// Returns transform's rotate part.
    pub fn get_rotate(&self) -> f32 {
        let rad = 180.0 / (f32::consts::PI as f32);
        let mut angle = (-self.b/self.a).atan() * rad;
        if self.b < self.c || self.b > self.c {
            angle = -angle;
        }
        angle
    }

    /// Applies transform to selected coordinates.
    pub fn apply(&self, x: f32, y: f32) -> (f32, f32) {
        let new_x = self.a * x + self.c * y + self.e;
        let new_y = self.b * x + self.d * y + self.f;
        (new_x, new_y)
    }

    /// Applies transform to selected coordinates.
    pub fn apply_to(&self, x: &mut f32, y: &mut f32) {
        let tx = *x;
        let ty = *y;
        *x = self.a * tx + self.c * ty + self.e;
        *y = self.b * tx + self.d * ty + self.f;
    }
}

#[inline]
fn multiply(ts1: &Transform, ts2: &Transform) -> Transform {
    Transform {
        a: ts1.a * ts2.a + ts1.c * ts2.b,
        b: ts1.b * ts2.a + ts1.d * ts2.b,
        c: ts1.a * ts2.c + ts1.c * ts2.d,
        d: ts1.b * ts2.c + ts1.d * ts2.d,
        e: ts1.a * ts2.e + ts1.c * ts2.f + ts1.e,
        f: ts1.b * ts2.e + ts1.d * ts2.f + ts1.f,
    }
}

impl Default for Transform {
    fn default() -> Transform {
        Transform::new(1.0, 0.0, 0.0, 1.0, 0.0, 0.0)
    }
}