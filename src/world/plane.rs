use crate::intersection::Intersection;
use crate::ray::Ray;
use crate::world::Interceptable;
use image::Rgba;
use na::{Vector3, Unit};

pub struct Plane {
    pub normal: Unit<Vector3<f64>>,
    pub d: f64,
    pub color: Rgba<u8>,
    pub opacity: f64,
    pub reflection: f64,
}

impl Interceptable for Plane {
    fn intercept(&self, ray:&Ray) -> Option<(f64, Intersection)> {
        unimplemented!();
    }
}
