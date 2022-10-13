use geo_types::{Polygon as GP, LineString, MultiPolygon};
use geo_booleanop::boolean::BooleanOp;
use crate::PathData;

#[derive(Default, Debug, Clone)]
pub struct Polygon {
    points: Vec<(f32, f32)>,
}

impl Polygon {

    pub fn new() -> Self {
        Polygon::default()
    }

    pub fn is_empty(&self) -> bool {
        self.points.len() == 0
    }

    pub fn add_point(&mut self, x: f32, y: f32) {
        self.points.push((x, y))
    }

    pub fn into_path(self) -> PathData {
        let mut path = PathData::new();
        for (x, y) in self.points.iter() {
            if path.is_empty() {
                path.move_to(*x, *y)
            } else {
                path.line_to(*x, *y)
            }
        }
        path.close();
        path
    }

    fn to_gp(&self) -> GP<f32> {
        GP::<f32>::new(LineString::from(self.points.clone()), vec![])
    }

    pub fn union(&self, another: &Self) -> Vec<Self> {
        let result = self.to_gp().union(&another.to_gp());
        multi_trans(result)
    }

    pub fn difference(&self, another: &Self) -> Vec<Self> {
        let result = self.to_gp().difference(&another.to_gp());
        multi_trans(result)
    }

    pub fn xor(&self, another: &Self) -> Vec<Self> {
        let result = self.to_gp().xor(&another.to_gp());
        multi_trans(result)
    }

    pub fn intersection(&self, another: &Self) -> Vec<Self> {
        let result = self.to_gp().intersection(&another.to_gp());
        multi_trans(result)
    }
}

#[inline]
fn multi_trans(mp: MultiPolygon<f32>) -> Vec<Polygon> {
    let mut result = vec![];
    for gp in mp.iter() {
        result.push(
            gp
                .exterior()
                .points_iter()
                .map(|point| (point.x(), point.y()))
                .collect::<Vec<(f32, f32)>>()
                .into()
        );
    }
    result
}

impl From<Vec<(f32, f32)>> for Polygon {
    fn from(points: Vec<(f32, f32)>) -> Self {
        Polygon {
            points
        }
    }
}


