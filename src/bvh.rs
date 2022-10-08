use std::cmp::Ordering;

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
    pub fn new(mut objects: Vec<Box<dyn Hittable>>) -> Self {
        fn box_compare(
            axis: usize,
        ) -> impl FnMut(&Box<dyn Hittable>, &Box<dyn Hittable>) -> Ordering {
            move |a, b| {
                let a_bbox = a.bounding_box();
                let b_bbox = b.bounding_box();
                let ac = a_bbox.minimum[axis] + a_bbox.maximum[axis];
                let bc = b_bbox.minimum[axis] + b_bbox.maximum[axis];
                ac.partial_cmp(&bc).unwrap()
            }
        }

        fn axis_range(objects: &[Box<dyn Hittable>], axis: usize) -> f32 {
            let (min, max) = objects
                .iter()
                .fold((f32::MAX, f32::MIN), |(bmin, bmax), hit| {
                    (
                        bmin.min(hit.bounding_box().minimum[axis]),
                        bmax.max(hit.bounding_box().maximum[axis]),
                    )
                });
            max - min
        }

        let mut axis_ranges: Vec<(usize, f32)> =
            (0..3).map(|a| (a, axis_range(&objects, a))).collect();

        axis_ranges.sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        let axis = axis_ranges[0].0;

        objects.sort_unstable_by(box_compare(axis));
        let len = objects.len();

        match len {
            0 => panic!["no elements in scene"],
            1 => {
                let leaf = objects.pop().unwrap();
                Bvh {
                    bbox: leaf.bounding_box(),
                    tree: BvhNode::Leaf(leaf),
                }
            }
            _ => {
                let right = Bvh::new(objects.drain(len / 2..).collect());
                let left = Bvh::new(objects);
                let bbox = AABB::join(&left.bbox, &right.bbox);
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
                    print_node(left, depth + 1);
                    println!("{}child_r", padding);
                    print_node(right, depth + 1);
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

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
