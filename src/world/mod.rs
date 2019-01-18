use crate::intersection::Intersection;
use crate::ray::Ray;
use crate::world::light::Light;
use image::Rgba;
use na::Vector3;
use std::f64;

pub mod light;
pub mod plane;
pub mod sphere;
pub mod triangle;
pub mod camera;

pub trait Interceptable {
    fn intercept(&self, ray: &Ray) -> Option<(f64, Intersection)>;
}

pub struct World {
    pub elements: Vec<Box<Interceptable>>,
    pub lights: Vec<Light>,
}

impl World {
    pub fn new(
        elements: Vec<Box<Interceptable>>,
        lights: Vec<Light>,
    ) -> Self {
        World {
            elements,
            lights,
        }
    }

    pub fn color(&self, ray: Ray, recursion_depth: u64) -> Rgba<u8> {
        if let Some(intersection) = self.next_intersection(&ray) {
            // touch something
            intersection.get_color(ray.dir.into_inner(), self, recursion_depth)
        } else {
            // background color
            Rgba([0, 0, 0, 255])
        }
    }

    pub fn appearance(&self, ray: Ray, recursion_depth: u64) -> Vector3<f64> {
        if let Some(intersection) = self.next_intersection(&ray) {
            // touch something
            intersection.get_appearance(ray.dir.into_inner(), self, recursion_depth)
        } else {
            Vector3::new(0.0,0.0,0.0)
        }
    }

    pub fn next_intersection(&self, ray: &Ray) -> Option<Intersection> {
        let mut max_distance = f64::INFINITY;
        let mut interception = None;
        for element in &self.elements {
            if let Some((distance, intercept)) = element.intercept(ray) {
                if distance < max_distance {
                    max_distance = distance;
                    interception = Some(intercept);
                }
            }
        }
        interception
    }
}
