use std::{cmp::Ordering, time::Instant};

use crate::{
    aabb::AABB,
    hittable::Hittable,
    ray::{HitRecord, Ray},
    util::{joint_aabb, joint_aabb_from_shapes},
};

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug)]
pub enum BVHNode {
    Leaf {
        depth: u32,
        parent_index: usize,
        shape_index: usize,
    },
    Node {
        depth: u32,
        parent_index: usize,
        child_l_index: usize,
        child_l_aabb: AABB,
        child_r_index: usize,
        child_r_aabb: AABB,
    },
}

impl BVHNode {
    pub fn build(
        shapes: &Vec<Box<dyn Hittable>>,
        indices: &mut [usize],
        nodes: &mut Vec<BVHNode>,
        parent_index: usize,
        depth: u32,
    ) -> usize {
        fn box_compare(
            shapes: &Vec<Box<dyn Hittable>>,
            axis: usize,
        ) -> impl FnMut(&usize, &usize) -> Ordering + '_ {
            move |a, b| {
                let a_bbox = shapes[*a].bounding_box();
                let b_bbox = shapes[*b].bounding_box();
                if let (Some(a), Some(b)) = (a_bbox, b_bbox) {
                    let ac = a.minimum[axis] + a.maximum[axis];
                    let bc = b.minimum[axis] + b.maximum[axis];
                    ac.partial_cmp(&bc).unwrap()
                } else {
                    panic!("no bounding box in bvh node")
                }
            }
        }

        fn axis_range(shapes: &Vec<Box<dyn Hittable>>, indices: &[usize], axis: usize) -> f32 {
            let (min, max) = indices
                .iter()
                .fold((f32::MAX, f32::MIN), |(bmin, bmax), hit| {
                    if let Some(aabb) = shapes[*hit].bounding_box() {
                        (bmin.min(aabb.minimum[axis]), bmax.max(aabb.maximum[axis]))
                    } else {
                        (bmin, bmax)
                    }
                });
            max - min
        }

        let mut axis_ranges: Vec<(usize, f32)> = (0..3)
            .map(|a| (a, axis_range(shapes, indices, a)))
            .collect();

        axis_ranges.sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        let axis = axis_ranges[0].0;

        let len = indices.len();

        indices.sort_unstable_by(box_compare(shapes, axis));

        if len == 1 {
            let shape_index = indices[0];
            let node_index = nodes.len();

            nodes.push(BVHNode::Leaf {
                depth,
                shape_index,
                parent_index,
            });

            return node_index;
        }

        let node_index = nodes.len();
        nodes.push(BVHNode::Leaf {
            depth,
            parent_index: 0,
            shape_index: 0,
        });

        let (left_indices, right_indices) = indices.split_at_mut(indices.len() / 2);

        let left = BVHNode::build(shapes, left_indices, nodes, node_index, depth + 1);
        let right = BVHNode::build(shapes, right_indices, nodes, node_index, depth + 1);

        nodes[node_index] = BVHNode::Node {
            depth,
            parent_index,
            child_l_index: left,
            child_l_aabb: joint_aabb(left_indices, shapes),
            child_r_index: right,
            child_r_aabb: joint_aabb(right_indices, shapes),
        };

        return node_index;
    }

    pub fn build_sah(
        shapes: &Vec<Box<dyn Hittable>>,
        indices: &mut [usize],
        nodes: &mut Vec<BVHNode>,
        parent_index: usize,
        depth: u32,
        time0: f32,
        time1: f32,
    ) -> usize {
        let len = indices.len();

        if len == 1 {
            let shape_index = indices[0];
            let node_index = nodes.len();

            nodes.push(BVHNode::Leaf {
                depth,
                shape_index,
                parent_index,
            });

            return node_index;
        }

        let node_index = nodes.len();
        nodes.push(BVHNode::Leaf {
            depth,
            parent_index: 0,
            shape_index: 0,
        });

        let (left_indices, right_indices) = indices.split_at_mut(indices.len() / 2);

        let left = BVHNode::build(shapes, left_indices, nodes, node_index, depth + 1);
        let right = BVHNode::build(shapes, right_indices, nodes, node_index, depth + 1);

        nodes[node_index] = BVHNode::Node {
            depth,
            parent_index,
            child_l_index: left,
            child_l_aabb: joint_aabb(left_indices, shapes),
            child_r_index: right,
            child_r_aabb: joint_aabb(right_indices, shapes),
        };

        return node_index;
    }

    fn traverse(r: &Ray, nodes: &[BVHNode], node_index: usize, indices: &mut Vec<usize>) {
        match nodes[node_index] {
            BVHNode::Node {
                child_l_index,
                child_l_aabb,
                child_r_index,
                child_r_aabb,
                ..
            } => {
                if r.aabb_intersect(child_l_aabb) {
                    BVHNode::traverse(r, nodes, child_l_index, indices);
                }
                if r.aabb_intersect(child_r_aabb) {
                    BVHNode::traverse(r, nodes, child_r_index, indices);
                }
            }
            BVHNode::Leaf { shape_index, .. } => indices.push(shape_index),
        }
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug)]
pub struct BVH {
    pub nodes: Vec<BVHNode>,
}

impl<'a> BVH {
    pub fn build(shapes: &mut Vec<Box<dyn Hittable>>) -> Self {
        let mut indices = (0..shapes.len()).collect::<Vec<usize>>();

        let mut nodes = Vec::new();

        BVHNode::build(shapes, &mut indices, &mut nodes, 0, 0);

        BVH { nodes }
    }

    pub fn traverse(
        &'a self,
        r: &Ray,
        shapes: &'a Vec<Box<dyn Hittable>>,
    ) -> Vec<&Box<dyn Hittable>> {
        let mut indices = Vec::new();

        BVHNode::traverse(r, &self.nodes, 0, &mut indices);

        indices
            .iter()
            .map(|index| &shapes[*index])
            .collect::<Vec<_>>()
    }

    pub fn pretty_print(&self) {
        let nodes = &self.nodes;
        fn print_node(nodes: &[BVHNode], node_index: usize) {
            match nodes[node_index] {
                BVHNode::Node {
                    child_l_index,
                    child_r_index,
                    depth,
                    child_l_aabb,
                    child_r_aabb,
                    ..
                } => {
                    let padding: String = " ".repeat(depth as usize);
                    println!("{}child_l {:?}", padding, child_l_aabb);
                    print_node(nodes, child_l_index);
                    println!("{}child_r {:?}", padding, child_r_aabb);
                    print_node(nodes, child_r_index);
                }
                BVHNode::Leaf {
                    shape_index, depth, ..
                } => {
                    let padding: String = " ".repeat(depth as usize);
                    println!("{}shape\t{:?}", padding, shape_index);
                }
            }
        }
        print_node(nodes, 0);
    }
}
