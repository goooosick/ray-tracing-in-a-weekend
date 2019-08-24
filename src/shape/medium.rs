use super::{HitRecord, Hitable};
use crate::accel::AABB;
use crate::material::Isotropic;
use crate::{Material, Ray, Vec3};

/// constant medium
pub struct ConstantMedium<H, T> {
    boundary: H,
    inverse_density: f32,
    material: Isotropic<T>,
}

impl<H, T> ConstantMedium<H, T> {
    /// construct constant medium with a `boundary`
    pub fn new(boundary: H, density: f32, texture: T) -> Self {
        ConstantMedium {
            boundary,
            inverse_density: density.recip(),
            material: Isotropic::new(texture),
        }
    }
}

impl<H, T> Hitable for ConstantMedium<H, T>
where
    H: Hitable,
    Isotropic<T>: Material,
{
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        use rand::prelude::*;
        use std::f32::{MAX, MIN};

        if let Some(mut rec1) = self.boundary.hit(ray, MIN, MAX) {
            if let Some(mut rec2) = self.boundary.hit(ray, rec1.t + 0.0001, MAX) {
                rec1.t = rec1.t.max(t_min);
                rec2.t = rec2.t.min(t_max);

                if rec1.t < rec2.t {
                    rec1.t = rec1.t.max(0.0);
                    let distance_inside_boundary = (rec2.t - rec1.t) * ray.direction.norm();
                    let hit_distance = -self.inverse_density * thread_rng().gen::<f32>().ln();

                    if hit_distance < distance_inside_boundary {
                        let t = rec1.t + hit_distance / ray.direction.norm();

                        return Some(HitRecord {
                            t,
                            point: ray.point_at(t),
                            normal: Vec3::unit(),
                            material: &self.material,
                            uv: rec2.uv,
                        });
                    }
                }
            }
        }

        None
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        self.boundary.bounding_box(t0, t1)
    }
}
