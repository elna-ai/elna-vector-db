use instant_distance::{Builder, HnswMap, Point};
use nalgebra::ComplexField;
use nalgebra::DVector;

pub fn generate_index(points: Vec<Vector>, values: Vec<String>) -> HnswMap<Vector, String> {
    Builder::default().build(points, values)
}

#[derive(Clone, Debug)]
pub struct Vector {
    data: DVector<f32>,
}

impl Point for Vector {
    fn distance(&self, other: &Self) -> f32 {
        let diff = self.data.clone() - other.data.clone();
        diff.dot(&diff).norm1()
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl From<Vec<f32>> for Vector {
    fn from(value: Vec<f32>) -> Self {
        let svec = DVector::from_vec(value);
        Vector { data: svec }
    }
}

impl Vector {
    pub fn cos_sim(&self, other: &Vector) -> f32 {
        self.data.dot(&other.data) / (self.data.norm() * other.data.norm())
    }
}
