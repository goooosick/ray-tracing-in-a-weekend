use crate::{Vec3, Ray, Hitable, Material};
use crate::shape::{HitRecord};

/// sphere hitable object
pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Box<dyn Material>,
}

impl Sphere {
    /// construct new sphere with `center` and `radius`
    pub fn new(center: Vec3, radius: f32, material: Box<dyn Material>) -> Self {
        Sphere {
            center,
            radius,
            material
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
                    normal: (ray.point_at(temp) - self.center) / self.radius,
                    material: self.material.as_ref()
                });
            }

            let temp = (-coeff_b + discriminant.sqrt()) / coeff_a;
            if temp < t_max && temp > t_min {
                return Some(HitRecord {
                    t: temp,
                    point: ray.point_at(temp),
                    normal: (ray.point_at(temp) - self.center) / self.radius,
                    material: self.material.as_ref()
                });
            }
        }

        None
    }
}
