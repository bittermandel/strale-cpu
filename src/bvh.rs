use std::{cmp::Ordering, sync::Arc};

use crate::{
    aabb::AABB,
    hittable::Hittable,
    ray::{HitRecord, Ray},
};

enum BvhNode {
    Branch { left: Box<Bvh>, right: Box<Bvh> },
    Leaf(Box<dyn Hittable>),
}

pub struct Bvh {
    tree: BvhNode,
    bbox: AABB,
}

impl Bvh {
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
                    let ac = a.minimum[axis] + a.maximum[axis];
                    let bc = b.minimum[axis] + b.maximum[axis];
                    ac.partial_cmp(&bc).unwrap()
                } else {
                    panic!("no bounding box in bvh node")
                }
            }
        }

        fn axis_range(objects: &[Box<dyn Hittable>], time0: f32, time1: f32, axis: usize) -> f32 {
            let (min, max) = objects
                .iter()
                .fold((f32::MAX, f32::MIN), |(bmin, bmax), hit| {
                    if let Some(aabb) = hit.bounding_box(time0, time1) {
                        (bmin.min(aabb.minimum[axis]), bmax.max(aabb.maximum[axis]))
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
                    Bvh {
                        tree: BvhNode::Leaf(leaf),
                        bbox,
                    }
                } else {
                    panic!("no bounding box in bvh node");
                }
            }
            _ => {
                let right = Bvh::new(objects.drain(len / 2..).collect(), time0, time1);
                let left = Bvh::new(objects, time0, time1);
                let bbox = AABB::surrounding_box(&left.bbox, &right.bbox);
                Bvh {
                    tree: BvhNode::Branch {
                        left: Box::new(left),
                        right: Box::new(right),
                    },
                    bbox,
                }
            }
        }
    }

    pub fn pretty_print(&self) {
        fn print_node(node: &Bvh, depth: usize) {
            match &node.tree {
                BvhNode::Branch { left, right, .. } => {
                    let padding: String = " ".repeat(depth as usize);
                    println!("{}child_l", padding);
                    print_node(&left, depth + 1);
                    println!("{}child_r", padding);
                    print_node(&right, depth + 1);
                }
                BvhNode::Leaf(_) => {
                    let padding: String = " ".repeat(depth as usize);
                    println!("{}shape\t", padding);
                }
            }
        }
        print_node(self, 0);
    }
}

impl Hittable for Bvh {
    fn hit(&self, r: &Ray, t_min: f32, mut t_max: f32) -> Option<HitRecord> {
        if !self.bbox.hit(r, t_min, t_max) {
            return None;
        }

        match &self.tree {
            BvhNode::Leaf(leaf) => leaf.hit(r, t_min, t_max),
            BvhNode::Branch { left, right } => {
                let left = left.hit(r, t_min, t_max);
                if let Some(hit) = &left {
                    t_max = hit.t;
                };
                let right = right.hit(r, t_min, t_max);
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
