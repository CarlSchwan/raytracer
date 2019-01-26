use crate::intersection::Intersection;
use crate::ray::Ray;
use crate::shader::Shader;
use crate::world::Interceptable;
use na::{Unit, Vector2, Vector3};

pub struct Plane {
    pub normal: Unit<Vector3<f64>>, // Vektor, senkrecht zur Ebene
    pub d: f64,                     // Distanz der Ebene zum Ursprung
    pub shader: Box<Shader>,
}

impl Interceptable for Plane {
    fn intercept(&self, ray: &Ray) -> Option<(f64, Intersection)> {
        let convergence_rate = ray.dir.into_inner().dot(&self.normal.into_inner());
        if convergence_rate == 0.0 {
            return None;
        }

        let intersection_distance =
            (self.d - self.normal.into_inner().dot(&ray.start)) / convergence_rate;
        if intersection_distance < 0.0 {
            return None;
        }

        let intersection_pos = intersection_distance * ray.dir.into_inner() + ray.start;
        let normal = if convergence_rate > 0.0 {
            -self.normal.into_inner()
        } else {
            self.normal.into_inner()
        };
        let intersection = Intersection {
            pos: intersection_pos,
            normal_at_surface: normal,
            shader: &self.shader,
            pos_on_surface: Vector2::new(0.0, 0.0), //TODO
        };
        return Some((intersection_distance, intersection));
    }
}
