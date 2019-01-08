use crate::intersection::Intersection;
use crate::ray::Ray;
use crate::world::Interceptable;
use crate::shader::Shader;
use image::Rgba;
use na::{Unit, Vector3, Vector2};

pub struct Plane {
    pub normal: Unit<Vector3<f64>>, // Vektor, senkrecht zur Ebene
    pub d: f64,                 // Distanz der Ebene zum Ursprung
    pub shader: Box<Shader>,
}

impl Interceptable for Plane {
    fn intercept(&self, ray: &Ray) -> Option<(f64, Intersection)> {
        let convergence_rate = ray.dir.unwrap().dot(&self.normal.unwrap());
        if convergence_rate == 0.0 {
            return None;
        }

        let intersection_distance = (self.d - self.normal.unwrap().dot(&ray.start)) / convergence_rate;
        if intersection_distance < 0.0 {
            return None;
        }

        let intersection_pos = intersection_distance * ray.dir.unwrap() + ray.start;
        let normal = if convergence_rate > 0.0 {
                            - self.normal.unwrap()
                        } else {
                            self.normal.unwrap()
                        };
        let intersection = Intersection {
            pos: intersection_pos,
            normal_at_surface: normal,
            shader: &self.shader,
            pos_on_surface: Vector2::new(0.0,0.0),//TODO
        };
        return Some((intersection_distance, intersection));
    }
}
