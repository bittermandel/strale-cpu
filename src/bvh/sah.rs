#![recursion_limit = "2048"]

use std::{cmp::Ordering, time::Instant};

use crate::{
    aabb::AABB,
    hittable::Hittable,
    ray::Ray,
    util::{concatenate_vectors, joint_aabb},
};

#[derive(Copy, Clone, Debug)]
struct Bucket {
    pub size: usize,
    pub aabb: AABB,
}

impl Bucket {
    pub fn empty() -> Bucket {
        Bucket {
            size: 0,
            aabb: AABB::empty(),
        }
    }

    pub fn add_aabb(&mut self, aabb: &AABB) {
        self.size += 1;
        self.aabb = self.aabb.join(aabb);
    }

    pub fn join_bucket(a: Bucket, b: &Bucket) -> Bucket {
        Bucket {
            size: a.size + b.size,
            aabb: a.aabb.join(&b.aabb),
        }
    }
}

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
        indices: &[usize],
        nodes: &mut Vec<BVHNode>,
        parent_index: usize,
        depth: u32,
    ) -> usize {
        let len = indices.len();

        if len == 0 {
            panic!("Indices empty!")
        }

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

        fn sum_aabbs(aabb_bounds: AABB, centroid_bounds: AABB, shape_aabb: &AABB) -> (AABB, AABB) {
            let center = &shape_aabb.center();
            (aabb_bounds.join(shape_aabb), centroid_bounds.grow(center))
        }

        let mut aabb_bounds = AABB::empty();
        let mut centroid_bounds = AABB::empty();

        for index in indices {
            (aabb_bounds, centroid_bounds) =
                sum_aabbs(aabb_bounds, centroid_bounds, &shapes[*index].bounding_box())
        }

        let node_index = nodes.len();
        nodes.push(BVHNode::Leaf {
            depth,
            parent_index: 0,
            shape_index: 0,
        });

        let split_axis = centroid_bounds.largest_axis();
        let split_axis_size =
            centroid_bounds.maximum[split_axis] - centroid_bounds.minimum[split_axis];

        let (child_l_index, child_l_aabb, child_r_index, child_r_aabb) = if split_axis_size
            < f32::EPSILON
        {
            let (child_l_indices, child_r_indices) = indices.split_at(indices.len() / 2);
            let child_l_aabb = joint_aabb(child_l_indices, shapes);
            let child_r_aabb = joint_aabb(child_r_indices, shapes);

            // Proceed recursively.
            let child_l_index =
                BVHNode::build(shapes, child_l_indices, nodes, node_index, depth + 1);
            let child_r_index =
                BVHNode::build(shapes, child_r_indices, nodes, node_index, depth + 1);
            (child_l_index, child_l_aabb, child_r_index, child_r_aabb)
        } else {
            const NUM_BUCKETS: usize = 6;

            let mut buckets = [Bucket::empty(); NUM_BUCKETS];
            let mut bucket_assignments: [Vec<usize>; NUM_BUCKETS] = Default::default();

            for idx in indices {
                let shape = &shapes[*idx];
                let shape_aabb = shape.bounding_box();
                let shape_center = shape_aabb.center();

                let relative_pos = (shape_center[split_axis] - centroid_bounds.minimum[split_axis])
                    / split_axis_size;

                let bucket_num = (relative_pos * (NUM_BUCKETS as f32 - 0.01)) as usize;

                buckets[bucket_num].add_aabb(&shape_aabb);
                bucket_assignments[bucket_num].push(*idx);
            }

            let mut min_bucket = 0;
            let mut min_cost = f32::INFINITY;
            let mut child_l_aabb = AABB::empty();
            let mut child_r_aabb = AABB::empty();

            for i in 0..(NUM_BUCKETS - 1) {
                let (l_buckets, r_buckets) = buckets.split_at(i + 1);

                let child_l = l_buckets.iter().fold(Bucket::empty(), Bucket::join_bucket);
                let child_r = r_buckets.iter().fold(Bucket::empty(), Bucket::join_bucket);

                let cost = (child_l.size as f32 * child_l.aabb.surface_area()
                    + child_r.size as f32 * child_r.aabb.surface_area())
                    / aabb_bounds.surface_area();

                if cost < min_cost {
                    min_bucket = i;
                    min_cost = cost;
                    child_l_aabb = child_l.aabb;
                    child_r_aabb = child_r.aabb;
                }
            }

            let (l_assignments, r_assignments) = bucket_assignments.split_at_mut(min_bucket + 1);
            let child_l_indices = concatenate_vectors(l_assignments);
            let child_r_indices = concatenate_vectors(r_assignments);

            let child_l_index =
                BVHNode::build(shapes, &child_l_indices, nodes, node_index, depth + 1);
            let child_r_index =
                BVHNode::build(shapes, &child_r_indices, nodes, node_index, depth + 1);
            (child_l_index, child_l_aabb, child_r_index, child_r_aabb)
        };

        nodes[node_index] = BVHNode::Node {
            depth,
            child_l_aabb,
            child_l_index,
            child_r_aabb,
            child_r_index,
            parent_index,
        };

        node_index
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
    pub fn build(shapes: &Vec<Box<dyn Hittable>>) -> Self {
        let now = Instant::now();

        let mut indices = (0..shapes.len()).collect::<Vec<usize>>();

        let mut nodes = Vec::with_capacity(shapes.len() * 2);

        //BVHNode::build(shapes, &mut indices, &mut nodes, 0, 0);
        BVHNode::build(shapes, &indices, &mut nodes, 0, 0);

        println!(
            "BVH built in {:?} using {} shapes",
            now.elapsed(),
            shapes.len()
        );

        BVH { nodes }
    }

    pub fn traverse(&'a self, r: &Ray, shapes: &'a [Box<dyn Hittable>]) -> Vec<&Box<dyn Hittable>> {
        let mut indices = Vec::new();

        BVHNode::traverse(r, &self.nodes, 0, &mut indices);

        indices
            .iter()
            .map(|index| &shapes[*index])
            .collect::<Vec<_>>()
    }

    pub fn total_surface_area(&self) -> f32 {
        let total = self.nodes.iter().fold(0.0, |total, node| match node {
            BVHNode::Node {
                child_l_aabb,
                child_r_aabb,
                ..
            } => {
                let aabb = AABB::join(child_l_aabb, child_r_aabb);
                total + aabb.surface_area()
            }
            _ => total,
        });
        total
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
        println!("total surface area: {}", self.total_surface_area());
    }
}
