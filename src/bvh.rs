use std::cmp::Ordering;

use crate::{
    aabb::AABB,
    hittable::Hittable,
    ray::{HitRecord, Ray},
};

enum BVHNode {
    Branch { left: Box<BVH>, right: Box<BVH> },
    Leaf(Box<dyn Hittable>),
}

pub struct BVH {
    tree: BVHNode,
    bbox: AABB,
}

impl BVH {
    pub fn new(mut objects: Vec<Box<dyn Hittable>>, time0: f32, time1: f32) -> Self {
        fn box_compare(
            time0: f32,
            time1: f32,
            axis: usize,
        ) -> impl FnMut(&Box<dyn Hittable>, &Box<dyn Hittable>) -> Ordering {
            move |a, b| {
                let a_bbox = a.bounding_box(time0, time1);
                let b_bbox = b.bounding_box(time0, time1);
                if let (Some(a), Some(b)) = (a_bbox, b_bbox) {
                    let ac = a.minimum.e[axis] + a.maximum.e[axis];
                    let bc = b.minimum.e[axis] + b.maximum.e[axis];
                    ac.partial_cmp(&bc).unwrap()
                } else {
                    panic!("no bounding box in bvh node")
                }
            }
        }

        fn axis_range(
            objects: &Vec<Box<dyn Hittable>>,
            time0: f32,
            time1: f32,
            axis: usize,
        ) -> f32 {
            let (min, max) = objects
                .iter()
                .fold((f32::MAX, f32::MIN), |(bmin, bmax), hit| {
                    if let Some(aabb) = hit.bounding_box(time0, time1) {
                        (
                            bmin.min(aabb.minimum.e[axis]),
                            bmax.max(aabb.maximum.e[axis]),
                        )
                    } else {
                        (bmin, bmax)
                    }
                });
            max - min
        }

        let mut axis_ranges: Vec<(usize, f32)> = (0..3)
            .map(|a| (a, axis_range(&objects, time0, time1, a)))
            .collect();

        axis_ranges.sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        let axis = axis_ranges[0].0;

        objects.sort_unstable_by(box_compare(time0, time1, axis));
        let len = objects.len();

        match len {
            0 => panic!["no elements in scene"],
            1 => {
                let leaf = objects.pop().unwrap();
                if let Some(bbox) = leaf.bounding_box(time0, time1) {
                    BVH {
                        tree: BVHNode::Leaf(leaf),
                        bbox,
                    }
                } else {
                    panic!("no bounding box in bvh node");
                }
            }
            _ => {
                let right = BVH::new(objects.drain(len / 2..).collect(), time0, time1);
                let left = BVH::new(objects, time0, time1);
                let bbox = AABB::surrounding_box(&left.bbox, &right.bbox);
                BVH {
                    tree: BVHNode::Branch {
                        left: Box::new(left),
                        right: Box::new(right),
                    },
                    bbox,
                }
            }
        }
    }
}

impl Hittable for BVH {
    fn hit(&self, r: &Ray, t_min: f32, mut t_max: f32) -> Option<HitRecord> {
        if !self.bbox.hit(r, t_min, t_max) {
            return None;
        }

        match &self.tree {
            BVHNode::Leaf(leaf) => leaf.hit(&r, t_min, t_max),
            BVHNode::Branch { left, right } => {
                let left = left.hit(&r, t_min, t_max);
                if let Some(hit) = &left {
                    t_max = hit.t;
                };
                let right = right.hit(&r, t_min, t_max);
                if right.is_some() {
                    right
                } else {
                    left
                }
            }
        }
    }

    fn bounding_box(&self, _time0: f32, _time1: f32) -> Option<AABB> {
        Some(self.bbox)
    }
}
