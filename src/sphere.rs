use crate::ray::Ray;
use crate::vec::Vec3;
use crate::hitable::{HitRecord, Hitable};

/// sphere hitable object
#[derive(Debug)]
pub struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Sphere {
    /// construct new sphere with `center` and `radius`
    pub fn new(center: Vec3, radius: f32) -> Self {
        Sphere {
            center,
            radius,
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let coeff_a = ray.direction.dot(ray.direction);
        let coeff_b = oc.dot(ray.direction);
        let coeff_c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = coeff_b * coeff_b - coeff_a * coeff_c;

        if discriminant > 0.0 {
            let temp = (-coeff_b - discriminant.sqrt()) / coeff_a;
            if temp < t_max && temp > t_min {
                return Some(HitRecord {
                    t: temp,
                    point: ray.point_at(temp),
                    normal: (ray.point_at(temp) - self.center) / self.radius
                });
            }

            let temp = (-coeff_b + discriminant.sqrt()) / coeff_a;
            if temp < t_max && temp > t_min {
                return Some(HitRecord {
                    t: temp,
                    point: ray.point_at(temp),
                    normal: (ray.point_at(temp) - self.center) / self.radius
                });
            }
        }

        None
    }
}
