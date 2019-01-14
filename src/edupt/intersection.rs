use super::vec::Vector;

pub struct Hitpoint {
    pub distance: f64,
    pub normal: Vector,
    pub position: Vector,
}

pub struct Intersection {
    pub hitpoint: Hitpoint,
    pub object_id: usize,
}