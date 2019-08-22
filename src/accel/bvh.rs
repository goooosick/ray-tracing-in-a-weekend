use crate::accel::{accumulate_aabbs, AABB};
use crate::shape::{HitRecord, Hitable, HitableList};
use crate::Ray;

/// acceleration structure, bounding volume hierarchy
pub struct BVH<'a> {
    nodes: Vec<Box<dyn Hitable + 'a>>,
    bbox: AABB,
}

impl<'a> BVH<'a> {
    /// consturct BVH from hitable list
    pub fn from_list(hlist: HitableList<'a>, t0: f32, t1: f32) -> Self {
        BVH::new(hlist.into_vec(), t0, t1)
    }

    /// recursive consturct BVH
    fn new(mut list: Vec<Box<dyn Hitable + 'a>>, t0: f32, t1: f32) -> Self {
        use rand::prelude::*;

        // sort by random axis
        let axis = (3.0 * thread_rng().gen::<f32>()) as usize;
        list.sort_by(
            |a, b| match (a.bounding_box(t0, t1), b.bounding_box(t0, t1)) {
                (Some(a), Some(b)) => a.min[axis].partial_cmp(&b.min[axis]).unwrap(),
                _ => panic!("no bounding box on object"),
            },
        );

        let nodes = if list.len() <= 2 {
            list
        } else {
            let list2 = list.split_off(list.len() / 2);
            vec![
                Box::new(BVH::new(list, t0, t1)) as Box<dyn Hitable>,
                Box::new(BVH::new(list2, t0, t1)) as Box<dyn Hitable>,
            ]
        };

        match accumulate_aabbs(&nodes, t0, t1) {
            Some(bbox) => BVH { nodes, bbox },
            None => panic!("no bounding box on object"),
        }
    }
}

impl<'a> Hitable for BVH<'a> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if self.bbox.hit(ray, t_min, t_max) {
            self.nodes
                .iter()
                .filter_map(|h| h.hit(ray, t_min, t_max))
                .min_by(|h1, h2| h1.t.partial_cmp(&h2.t).unwrap())
        } else {
            None
        }
    }

    fn bounding_box(&self, _: f32, _: f32) -> Option<AABB> {
        Some(self.bbox.clone())
    }
}
