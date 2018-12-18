use crate::intersection::Intersection;
use crate::ray::Ray;
use crate::world::{light::Light, obj::Obj, plane::Plane, triangle::Triangle};
use image::{GenericImage, ImageBuffer};
use std::path::Path;

pub mod light;
pub mod obj;
pub mod plane;
pub mod triangle;

pub trait Interceptable {
    fn intercept(&self, ray: Ray) -> Option<(f64, Intersection)>;
}

pub struct World {
    width: usize,
    height: usize,
    elements: Vec<Element>,
}

pub enum Element {
    Light(Light),
    Triangle(Triangle),
    Plane(Plane),
    Obj(Obj),
}

impl World {
    pub fn new(width: usize, height: usize) -> Self {
        World { width, height, elements: Vec::new() }
    }

    pub fn add_sphere(&mut self, sphere: Sphere) -> Self {
        // TODO
        self
    }

    pub fn add_plane(&mut self, plane: Plane) -> Self {
        // TODO
        self
    }

    pub fn add_triangle(&mut self, triangle: Triangle) -> Self {
        // TODO
        self
    }

    pub fn add_obj(&mut self, obj_path: Path) -> Self {
        // TODO
        self
    }

    pub fn render(&self) /* -> ImgBuffer */ {

    }
}
