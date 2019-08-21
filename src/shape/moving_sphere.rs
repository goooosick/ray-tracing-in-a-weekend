use crate::{Vec3, Ray, Hitable, Material};
use crate::shape::{HitRecord};

/// a moving sphere hitable object
pub struct MovingSphere {
    center: Vec3,
    offset: Vec3,
    time_start: f32,
    time_interval: f32,
    radius: f32,
    material: Box<dyn Material>,
}

impl MovingSphere {
    /// construct new sphere with `center` period and `radius`
    pub fn new(center: Vec3, offset: Vec3, time_start: f32, time_interval: f32,
        radius: f32, material: Box<dyn Material>
    ) -> Self {
        MovingSphere {
            center,
            offset,
            time_start,
            time_interval,
            radius,
            material
        }
    }

    #[inline]
    fn center(&self, t: f32) -> Vec3 {
        self.center + ((t - self.time_start) / self.time_interval) * self.offset
    }
}

impl Hitable for MovingSphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center(ray.time);
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