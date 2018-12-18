use Ray;
use Intersection;

pub mod triangle;
pub mod plane;
pub mod light;
pub mod obj;

pub trait Interceptable {
    pub fn intercept(&self, ray: Ray) -> Option<(f64, Intersection)>
}

pub struct World {
    image: ImageBuffer,
    elements: Vec<Element>,
}

pub enum Element {
    Light(Light),
    Triangle(Triangle),
    Plane(Plane),
    Obj(Obj),
}
