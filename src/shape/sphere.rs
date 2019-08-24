use crate::shape::{HitRecord, AABB};
use crate::{Hitable, Material, Ray, Vec3};

/// sphere hitable object
pub struct Sphere<T> {
    center: Vec3,
    radius: f32,
    material: T,
}

impl<T> Sphere<T> {
    /// construct new sphere with `center` and `radius`
    pub fn new(center: Vec3, radius: f32, material: T) -> Self {
        Sphere {
            center,
            radius,
            material,
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
                    material: &self.material,
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
                    material: &self.material,
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

/// a moving sphere hitable object
pub struct MovingSphere<T> {
    center: Vec3,
    offset: Vec3,
    time_start: f32,
    time_interval: f32,
    radius: f32,
    material: T,
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
        assert!(time_interval != 0.0);
        MovingSphere {
            center,
            offset,
            time_start,
            time_interval,
            radius,
            material,
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
                let point = ray.point_at(temp);
                let normal = (point - self.center) / self.radius;
                return Some(HitRecord {
                    t: temp,
                    point,
                    normal,
                    material: &self.material,
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
                    material: &self.material,
                    uv: get_sphere_uv(normal),
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

pub fn get_sphere_uv(point: Vec3) -> (f32, f32) {
    let phi = point.z.atan2(point.x);
    let theta = point.y.asin();
    (
        0.5 - phi / (2.0 * std::f32::consts::PI),
        0.5 + theta / std::f32::consts::PI,
    )
}
