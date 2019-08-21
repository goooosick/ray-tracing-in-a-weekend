use crate::Hitable;

pub use aabb::AABB;
pub use bvh::BVH;

mod aabb;
mod bvh;

/// accumulate vector of aabbs
pub fn accumulate_aabbs<'a>(list: &Vec<Box<dyn Hitable + 'a>>, t_min: f32, t_max: f32)
    -> Option<AABB>
{
    if list.is_empty() {
        return None;
    }

    list[0]
        .bounding_box(t_min, t_max)
        .and_then(|mut aabb| {
            for obj in list.iter().skip(1) {
                if let Some(bbox) = obj.bounding_box(t_min, t_max) {
                    aabb = AABB::surrouding(aabb, bbox);
                } else {
                    return None;
                }
            }

            Some(aabb)
        })
}
