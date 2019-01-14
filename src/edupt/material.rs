use super::vec::Vector;

pub type Color = Vector;

pub enum ReflectionType {
    Diffuse,
    Specular,
    Refraction,
}

pub const IOR: f64 = 1.5;