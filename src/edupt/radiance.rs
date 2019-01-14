use super::vec::*;
use super::ray::Ray;
use super::random::Random;
use super::constant::*;
use super::material::Color;
use super::material::ReflectionType;
use super::material::IOR;
use super::scene::spheres;
use super::scene::intersect_scene;

const DEPTH: u32 = 5;
const DEPTH_LIMIT: u32 = 64; 

pub fn radiance(ray: &Ray, rnd: &mut Random, depth: u32) -> Color {
    let intersection = intersect_scene(ray);
    if let None = intersection {
        return Color::zero();
    }
    let intersection = intersection.unwrap();

    let now_object = &spheres[intersection.object_id as usize];
    let hitpoint = intersection.hitpoint;
    let orienting_normal = if dot(&hitpoint.normal, &ray.dir) < 0.0 { hitpoint.normal } else { -1.0 * hitpoint.normal };
    let mut russian_roulette_probability = now_object.color.x.max(now_object.color.y.max(now_object.color.z));

    if depth > DEPTH_LIMIT {
        russian_roulette_probability *= (0.5f64).powf((depth - DEPTH_LIMIT) as f64);
    }

    if depth > DEPTH {
        if rnd.next01() >= russian_roulette_probability {
            return now_object.emission;
        }
    } else {
        russian_roulette_probability = 1.0;
    }

    let incoming_radiance;
    let weight;

    match now_object.reflection_type {
        ReflectionType::Diffuse => {
            let w = orienting_normal;
            let u = if w.x.abs() > EPS {
                normalize(&mut cross(&Vector::new(0.0, 1.0, 0.0), &w))
            } else {
                normalize(&mut cross(&Vector::new(1.0, 0.0, 0.0), &w))
            };
            let v = cross(&w, &u);
            let r1 = 2.0 * PI * rnd.next01();
            let r2 = rnd.next01();
            let r2s = r2.sqrt();
            let dir = normalize(&mut (
                u * f64::cos(r1) * r2s +
                v * f64::sin(r1) * r2s +
                w * (1.0 - r2).sqrt()
            ));
            incoming_radiance = radiance(&Ray::new(hitpoint.position, dir), rnd, depth + 1);
            weight = now_object.color / russian_roulette_probability;
        },
        ReflectionType::Specular => {
            incoming_radiance = radiance(
                &Ray::new(hitpoint.position, ray.dir - hitpoint.normal * 2.0 * dot(&hitpoint.normal, &ray.dir)),
                rnd, depth + 1);
            weight = now_object.color / russian_roulette_probability;
        },
        ReflectionType::Refraction => {
            let reflection_ray = Ray::new(
                hitpoint.position,
                ray.dir - hitpoint.normal * 2.0 * dot(&hitpoint.normal, &ray.dir)
            );
            let into = dot(&hitpoint.normal, &orienting_normal) > 0.0;

            let nc = 1.0;
            let nt = IOR;
            let nnt = if into { nc / nt } else { nt / nc };
            let ddn = dot(&ray.dir, &orienting_normal);
            let cos2t = 1.0 - nnt * nnt * (1.0 - ddn * ddn);

            if cos2t < 0.0 {
                incoming_radiance = radiance(&reflection_ray, rnd, depth + 1);
                weight = now_object.color / russian_roulette_probability;
            } else {
                let refraction_ray = Ray::new(
                    hitpoint.position,
                    normalize(&mut(ray.dir * nnt - hitpoint.normal * (if into { 1.0 } else { -1.0 }) * (ddn * nnt + cos2t.sqrt())))
                );

                let a = nt - nc;
                let b = nt + nc;
                let r0 = (a * a) / (b * b);

                let c = 1.0 - (if into { -ddn } else { dot(&refraction_ray.dir, &(-1.0 * orienting_normal)) });
                let re = r0 + (1.0 - r0) * c.powf(5.0);
                let nnt2 = (if into { nc / nt } else { nt / nc }).powf(2.0);
                let tr = (1.0 - re) * nnt2;

                let probability = 0.25 + 0.5 * re;
                if depth > 2 {
                    if rnd.next01() < probability {
                        incoming_radiance = radiance(&reflection_ray, rnd, depth + 1) * re;
                        weight = now_object.color / (probability * russian_roulette_probability);
                    } else {
                        incoming_radiance = radiance(&refraction_ray, rnd, depth + 1) * tr;
                        weight = now_object.color / ((1.0 - probability) * russian_roulette_probability);
                    }
                } else {
                    incoming_radiance =
                        radiance(&reflection_ray, rnd, depth + 1) * re +
                        radiance(&refraction_ray, rnd, depth + 1) * tr;
                    weight = now_object.color / russian_roulette_probability;
                }
            }
        }
    }

    now_object.emission + multiply(&weight, &incoming_radiance)
}