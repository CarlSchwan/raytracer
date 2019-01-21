use na::Vector3;
use crate::world::Interceptable;

pub mod Collector;
mod BVStorage;
pub mod PrimitiveStorage;

pub trait Bounded : Interceptable {
	fn get_min(&self) -> Vector3<f64>;
	fn get_max(&self) -> Vector3<f64>;
}