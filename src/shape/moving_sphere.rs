use crate::shape::{HitRecord, AABB};
use crate::{Hitable, Material, Ray, Vec3};

/// a moving sphere hitable object
pub struct MovingSphere<T> {
    center: Vec3,
    offset: Vec3,
    time_start: f32,
    time_interval: f32,
    radius: f32,
    material: Box<T>,
}

impl<T> MovingSphere<T> {
    /// construct new sphere with `center` period and `radius`
    pub fn new(
        center: Vec3,
        offset: Vec3,
        time_start: f32,
        time_interval: f32,
        radius: f32,
        material: T,
    ) -> Self {
        MovingSphere {
            center,
            offset,
            time_start,
            time_interval,
            radius,
            material: Box::new(material),
        }
    }

    #[inline]
    fn center(&self, t: f32) -> Vec3 {
        self.center + ((t - self.time_start) / self.time_interval) * self.offset
    }
}

impl<T> Hitable for MovingSphere<T>
where
    T: Material,
{
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

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        let box0 = AABB::new(
            self.center(t0) - Vec3::new(self.radius, self.radius, self.radius),
            self.center(t0) + Vec3::new(self.radius, self.radius, self.radius),
        );
        let box1 = AABB::new(
            self.center(t1) - Vec3::new(self.radius, self.radius, self.radius),
            self.center(t1) + Vec3::new(self.radius, self.radius, self.radius),
        );

        Some(AABB::surrouding(box0, box1))
    }
}
