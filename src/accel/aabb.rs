use crate::{Ray, Vec3};

/// axis-aligned bounding box
#[derive(Clone)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

impl AABB {
    /// construct new AABB
    pub fn new(min: Vec3, max: Vec3) -> Self {
        AABB { min, max }
    }

    /// intersect ray with aabb
    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> bool {
        // see also (simd optimization):
        // https://medium.com/@bromanz/another-view-on-the-classic-ray-aabb-intersection-algorithm-for-bvh-traversal-41125138b525
        let mut t_min = t_min;
        let mut t_max = t_max;

        for i in 0..3 {
            let invd = ray.direction[i].recip();

            let mut t0 = (self.min[i] - ray.origin[i]) * invd;
            let mut t1 = (self.max[i] - ray.origin[i]) * invd;
            if invd < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }

            t_min = t0.max(t_min);
            t_max = t1.min(t_max);

            if t_max <= t_min {
                return false;
            }
        }

        true
    }

    /// get minimal aabb that holding two aabbs
    pub fn surrouding(a0: AABB, a1: AABB) -> AABB {
        AABB::new(
            Vec3::new(
                a0.min.x.min(a1.min.x),
                a0.min.y.min(a1.min.y),
                a0.min.z.min(a1.min.z),
            ),
            Vec3::new(
                a0.max.x.max(a1.max.x),
                a0.max.y.max(a1.max.y),
                a0.max.z.max(a1.max.z),
            ),
        )
    }
}
