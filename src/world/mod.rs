use crate::intersection::Intersection;
use crate::ray::Ray;
use crate::world::{light::Light, obj::Obj, plane::Plane, triangle::Triangle};
use image::{GenericImage, ImageBuffer};

pub mod light;
pub mod obj;
pub mod plane;
pub mod triangle;

pub trait Interceptable {
    fn intercept(&self, ray: Ray) -> Option<(f64, Intersection)>;
}

pub struct World {
    //image: ImageBuffer,
    elements: Vec<Element>,
}

pub enum Element {
    Light(Light),
    Triangle(Triangle),
    Plane(Plane),
    Obj(Obj),
}
