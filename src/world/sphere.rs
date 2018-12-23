use crate::intersection::Intersection;
use crate::ray::Ray;
use crate::world::Interceptable;
use image::Rgb;
use na::Point3;

pub struct Sphere {
    pub center: Point3<f64>,
    pub radius: f64,
    pub color: Rgb<u8>,
}

impl Interceptable for Sphere {
    fn intercept(&self, ray: &Ray) -> Option<(f64, Intersection)> {
        // TODO
        None
    }
}
