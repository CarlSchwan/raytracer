use crate::intersection::Intersection;
use crate::ray::Ray;
use crate::world::Interceptable;
use std::f64;

pub struct PrimitiveStorage {
    pub elements: Vec<Box<Interceptable>>,
}

impl Interceptable for PrimitiveStorage {
    fn intercept(&self, ray: &Ray) -> Option<(f64, Intersection)> {
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
        return if let Some(int) = interception {
            Some((max_distance, int))
        } else {
            None
        };
    }
}
