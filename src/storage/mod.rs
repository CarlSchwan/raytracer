use na::Vector3;
use crate::world::Interceptable;
use crate::intersection::Intersection;
use crate::ray::Ray;

pub mod Collector;
mod BVStorage;
pub mod PrimitiveStorage;

pub trait Bounded : Interceptable {
	fn get_min(&self) -> Vector3<f64>;
	fn get_max(&self) -> Vector3<f64>;
}

impl From<Box<Bounded>> for Box<Interceptable> {
	fn from(element: Box<Bounded>) -> Self {
		Box::new(InterceptFromBound{child: element})
	}
}

struct InterceptFromBound {
	child: Box<Bounded>,
}

impl Interceptable for InterceptFromBound {
    fn intercept(&self, ray: &Ray) -> Option<(f64, Intersection)> {
		self.child.intercept(ray)
	}
}