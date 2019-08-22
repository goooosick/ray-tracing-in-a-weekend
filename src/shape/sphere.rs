use crate::shape::{HitRecord, AABB};
use crate::{Hitable, Material, Ray, Vec3};

/// sphere hitable object
pub struct Sphere<T> {
    center: Vec3,
    radius: f32,
    material: Box<T>,
}

impl<T> Sphere<T> {
    /// construct new sphere with `center` and `radius`
    pub fn new(center: Vec3, radius: f32, material: T) -> Self {
        Sphere {
            center,
            radius,
            material: Box::new(material),
        }
    }
}

impl<T> Hitable for Sphere<T>
where
    T: Material,
{
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
                    material: self.material.as_ref(),
                });
            }

            let temp = (-coeff_b + discriminant.sqrt()) / coeff_a;
            if temp < t_max && temp > t_min {
                return Some(HitRecord {
                    t: temp,
                    point: ray.point_at(temp),
                    normal: (ray.point_at(temp) - self.center) / self.radius,
                    material: self.material.as_ref(),
                });
            }
        }

        None
    }

    fn bounding_box(&self, _: f32, _: f32) -> Option<AABB> {
        Some(AABB::new(
            self.center - Vec3::new(self.radius, self.radius, self.radius),
            self.center + Vec3::new(self.radius, self.radius, self.radius),
        ))
    }
}
