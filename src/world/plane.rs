use crate::intersection::Intersection;
use crate::ray::Ray;
use crate::world::Interceptable;
use crate::shader::Shader;
use na::{Unit, Vector3, Vector2};

pub struct Plane {
    pub a: Vector3<f64>,
	pub b: Vector3<f64>,
	pub c: Vector3<f64>,
    pub shader: Box<Shader>,
}

impl Interceptable for Plane {
    fn intercept(&self, ray: &Ray) -> Option<(f64, Intersection)> {
		let edge_ab = (self.a - self.b).normalize();
		let edge_ac = (self.a - self.c).normalize();
		let	normal = edge_ab.cross(&edge_ac);
        let convergence_rate = ray.dir.into_inner().dot(&normal);
        if convergence_rate == 0.0 {
            return None;
        }

        let intersection_distance = normal.dot(&(self.a -ray.start)) / convergence_rate;
        if intersection_distance < 0.0 {
            return None;
        }

        let intersection_pos = intersection_distance * ray.dir.into_inner() + ray.start;
        let normal = if convergence_rate > 0.0 {
                            - normal
                        } else {
                            normal
                        };
        let intersection = Intersection {
            pos: intersection_pos,
            normal_at_surface: normal,
            shader: &self.shader,
            pos_on_surface: Vector2::new((self.a -intersection_pos).dot(&edge_ab),(self.a - intersection_pos).dot(&edge_ac)),
        };
        return Some((intersection_distance, intersection));
    }
}
