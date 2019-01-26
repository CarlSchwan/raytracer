use crate::ray::Ray;
use crate::intersection::Intersection;
use crate::storage::PrimitiveStorage::PrimitiveStorage;
use nalgebra::Vector3;
use crate::storage::Bounded;
use std::f64;
use crate::world::Interceptable;
use crate::storage::*;

pub struct BVStorage {
    left: Box<Interceptable>,
    right: Box<Interceptable>,
    min: Vector3<f64>,
    max: Vector3<f64>,
}

impl BVStorage {
    pub fn new(elements: Vec<Box<Bounded>>) -> Self {
        let box_min = pointwise_min_list(elements.iter().map(|v| v.get_min()).collect());
        let box_max = pointwise_max_list(elements.iter().map(|v| v.get_max()).collect());

        let mins_max = pointwise_max_list(elements.iter().map(|v| v.get_min()).collect());


        let split_dimension = (mins_max - box_min).iamax();
        let split_point = (mins_max[split_dimension] + box_min[split_dimension]) / 2.0;

        
        let mut lower_elements = Vec::new();
        let mut upper_elements = Vec::new();
        for element in elements {
            if element.get_min()[split_dimension] < split_point {
                lower_elements.push(element);
            } else {
                upper_elements.push(element);
            }
        }
        
        let left: Box<Interceptable> = if lower_elements.len() < 4 {
                let mut eles: Vec<Box<Interceptable>> = Vec::new();
                for element in lower_elements {
                    eles.push(Box::from(element));
                }
                Box::new(PrimitiveStorage{ elements: eles})
            } else {
                Box::new(BVStorage::new(lower_elements))
            };
        let right: Box<Interceptable> = if upper_elements.len() < 4 {
                let mut eles: Vec<Box<Interceptable>> = Vec::new();
                for element in upper_elements {
                    eles.push(Box::from(element));
                }
                Box::new(PrimitiveStorage{ elements: eles})
            } else {
                Box::new(BVStorage::new(upper_elements))
            };
        
        BVStorage {
            min: box_min,
            max: box_max,
            left,
            right,
        }
    }
}

impl Interceptable for BVStorage {
    fn intercept(&self, ray: &Ray) -> Option<(f64, Intersection)> {
        let tm_choose = ray.dir.map(|x| (x > 0.0) as i64 as f64);
        let tM_choose = Vector3::new(1.0, 1.0, 1.0) - tm_choose;
        let mut tm = self.min.x * tm_choose + self.max.x * tM_choose;
        let mut tM = self.min.x * tM_choose + self.max.x * tm_choose;
        let speed : Vector3<f64> = ray.dir.map(|x| if x == 0.0 { f64::MIN_POSITIVE } else { x });
        tm = (tm - ray.start).component_div(&speed);
        tM = (tM - ray.start).component_div(&speed);

        if (tm.x.max(tm.y).max(tm.z) > tM.x.max(tM.y).max(tM.z)) {
            return None;
        }

        return match (self.left.intercept(ray), self.right.intercept(ray)) {
            (None, x) => x,
            (x, None) => x,
            (Some((dist1, int1)), Some((dist2, int2))) => if dist1 < dist2 {
                    Some((dist1, int1))
                } else {
                    Some((dist2, int2))
                }
        }
    }
}

fn pointwise_min_list(vectors: Vec<Vector3<f64>>) -> Vector3<f64> {
    let mut res = Vector3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
    for vector in vectors {
        res = pointwise_min(res, vector);
    }
    res
}

fn pointwise_max_list(vectors: Vec<Vector3<f64>>) -> Vector3<f64> {
    let mut res = Vector3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);
    for vector in vectors {
        res = pointwise_max(res, vector);
    }
    res
}

fn pointwise_min(v1: Vector3<f64>, v2:Vector3<f64>) -> Vector3<f64> {
    let x = v1.x.min(v2.x);
    let y = v1.y.min(v2.y);
    let z = v1.z.min(v2.z);
    Vector3::new(x, y, z)
}

fn pointwise_max(v1: Vector3<f64>, v2:Vector3<f64>) -> Vector3<f64> {
    let x = v1.x.max(v2.x);
    let y = v1.y.max(v2.y);
    let z = v1.z.max(v2.z);
    Vector3::new(x, y, z)
}