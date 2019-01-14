use super::vec::Vector;
use super::ray::Ray;
use super::intersection::Intersection;
use super::material::Color;
use super::sphere::Sphere;
use super::material::ReflectionType;

lazy_static! {
    pub static ref spheres: [Sphere; 10] = [
        Sphere::new(1e5, Vector::new(1e5 + 1.0, 40.8, 81.6), Color::zero(), Color::new(0.75, 0.25, 0.25), ReflectionType::Diffuse),
        Sphere::new(1e5, Vector::new(-1e5 + 99.0, 40.8, 81.6), Color::zero(), Color::new(0.25, 0.25, 0.75), ReflectionType::Diffuse),
        Sphere::new(1e5, Vector::new(50.0, 40.8, 1e5), Color::zero(), Color::new(0.75, 0.75, 0.75), ReflectionType::Diffuse),
        Sphere::new(1e5, Vector::new(50.0, 40.8, -1e5 + 250.0), Color::zero(), Color::zero(), ReflectionType::Diffuse),
        Sphere::new(1e5, Vector::new(50.0, 1e5, 81.6), Color::zero(), Color::new(0.75, 0.75, 0.75), ReflectionType::Diffuse),
        Sphere::new(1e5, Vector::new(50.0, -1e5 + 81.6, 81.6), Color::zero(), Color::new(0.75, 0.75, 0.75), ReflectionType::Diffuse),
        Sphere::new(20.0, Vector::new(65.0, 20.0, 20.0), Color::zero(), Color::new(0.25, 0.75, 0.25), ReflectionType::Diffuse),
        Sphere::new(16.5, Vector::new(27.0, 16.5, 47.0), Color::zero(), Color::new(0.99, 0.99, 0.99), ReflectionType::Specular),
        Sphere::new(16.5, Vector::new(77.0, 16.5, 78.0), Color::zero(), Color::new(0.99, 0.99, 0.99), ReflectionType::Refraction),
        Sphere::new(15.0, Vector::new(50.0, 90.0, 81.6), Color::new(36.0, 36.0, 36.0), Color::zero(), ReflectionType::Diffuse),
    ];
}

pub fn intersect_scene(ray: &Ray) -> Option<Intersection> {
    let mut intersection: Option<Intersection> = None;
    for i in 0..spheres.len() {
        let hitpoint = spheres[i].intersect(ray);
        if let Some(hitpoint) = hitpoint {
            match intersection {
                Some(inter) => {
                    if hitpoint.distance < inter.hitpoint.distance {
                        intersection = Some(Intersection {
                            hitpoint,
                            object_id: i,
                        });
                    } else {
                        intersection = Some(inter);
                    }
                },
                None => {
                    intersection = Some(Intersection {
                        hitpoint,
                        object_id: i,
                    });
                }
            }
        }
    }
    intersection
}