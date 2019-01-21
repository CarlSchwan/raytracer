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

		let split_dimension = max_abs_dimension(mins_max - box_min);
		let split_point = (get_vector_dim(mins_max, split_dimension) - get_vector_dim(box_min, split_dimension)) / 2.0;
		
		let mut lower_elements = Vec::new();
		let mut upper_elements = Vec::new();
		for element in elements {
			if get_vector_dim(element.get_min(), split_dimension) < split_point {
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
		let xspeed = if ray.dir.x == 0.0 {
			f64::MIN_POSITIVE } else {ray.dir.x};
		let yspeed = if ray.dir.y == 0.0 {
			f64::MIN_POSITIVE } else {ray.dir.y};
		let zspeed = if ray.dir.z == 0.0 {
			f64::MIN_POSITIVE } else {ray.dir.z};
		let txm = if ray.dir.x > 0.0 {
				(self.min.x - ray.start.x) / xspeed
			} else {
				(self.max.x - ray.start.x) / xspeed
			};
		let txM = if ray.dir.x > 0.0 {
				(self.max.x - ray.start.x) / xspeed
			} else {
				(self.min.x - ray.start.x) / xspeed
			};
			
		let tym = if ray.dir.y > 0.0 {
				(self.min.y - ray.start.y) / yspeed
			} else {
				(self.max.y - ray.start.y) / yspeed
			};
		let tyM = if ray.dir.y > 0.0 {
				(self.max.y - ray.start.y) / yspeed
			} else {
				(self.min.y - ray.start.y) / yspeed
			};
			
		let tzm = if ray.dir.z > 0.0 {
				(self.min.z - ray.start.z) / zspeed
			} else {
				(self.max.z - ray.start.z) / zspeed
			};
		let tzM = if ray.dir.z > 0.0 {
				(self.max.z - ray.start.z) / zspeed
			} else {
				(self.min.z - ray.start.z) / zspeed
			};
			
		if txm.max(tym).max(tzm) > txM.min(tyM).min(tzM) {
			return None
		}
		return match (self.left.intercept(ray), self.right.intercept(ray)) {
			(None, None) => None,
			(Some((dist, int)), None) => Some((dist,int)),
			(None, Some((dist, int))) => Some((dist, int)),
			(Some((dist1, int1)), Some((dist2, int2))) => if dist1 < dist2 {
					Some((dist1, int1))
				} else {
					Some((dist2, int2))
				}
		}
	}
}

fn get_vector_dim(v: Vector3<f64>, dim: u8) -> f64 {
	if dim==0 {
		return v.x
	}
	if dim==1 {
		return v.y
	}
	v.z
}

fn max_abs_dimension(v: Vector3<f64>) -> u8 {
	let mut max = v.x.abs();
	let mut dim = 0;
	
	if v.y.abs() > max {
		max = v.y.abs();
		dim = 1;
	}
	if v.z.abs() > max {
		dim = 2;
	}
	dim
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