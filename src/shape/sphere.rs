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
                let point = ray.point_at(temp);
                let normal = (point - self.center) / self.radius;
                return Some(HitRecord {
                    t: temp,
                    point,
                    normal,
                    material: self.material.as_ref(),
                    uv: get_sphere_uv(normal),
                });
            }

            let temp = (-coeff_b + discriminant.sqrt()) / coeff_a;
            if temp < t_max && temp > t_min {
                let point = ray.point_at(temp);
                let normal = (point - self.center) / self.radius;
                return Some(HitRecord {
                    t: temp,
                    point,
                    normal,
                    material: self.material.as_ref(),
                    uv: get_sphere_uv(normal),
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

pub fn get_sphere_uv(point: Vec3) -> (f32, f32) {
    let phi = point.z.atan2(point.x);
    let theta = point.y.asin();
    (
        0.5 - phi / (2.0 * std::f32::consts::PI),
        0.5 + theta / std::f32::consts::PI,
    )
}
