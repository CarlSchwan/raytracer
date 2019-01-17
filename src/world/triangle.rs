use crate::intersection::Intersection;
use crate::ray::Ray;
use crate::world::Interceptable;
use crate::shader::Shader;
use na::{Vector3, Vector2};

pub struct Triangle {
    pub a: Vector3<f64>,
    pub b: Vector3<f64>,
    pub c: Vector3<f64>,
    pub shader: Box<Shader>,
}

impl Interceptable for Triangle {
    // Shamelessly stolen from https://en.wikipedia.org/wiki/M%C3%B6ller%E2%80%93Trumbore_intersection_algorithm
    fn intercept(&self, ray: &Ray) -> Option<(f64, Intersection)> {
        let epsilon = 0.0001;
        let edge1 = self.b - self.a;
        let edge2 = self.c - self.a;
        let h = ray.dir.cross(&edge2);
        let a = edge1.dot(&h);
        if a.abs() < epsilon {
            // This ray is parallel to this triangle.
            return None;
        }
        let f = 1.0 / a;
        let s = ray.start - self.a;
        let u = f * s.dot(&h);
        if u < 0.0 || u > 1.0 {
            return None;
        }
        let q = s.cross(&edge1);
        let v = f * ray.dir.dot(&q);
        if v < 0.0 || u + v > 1.0 {
            return None;
        }
        // At this stage we can compute t to find out where the intersection point is on the line.
        let t = f * edge2.dot(&q);
        return if t > epsilon
        // ray intersection
        {
            let pos = ray.start + ray.dir.into_inner() * t;
            let h = edge1.cross(&edge2);

            let normal = if h.dot(&ray.dir) < 0.0 { h } else { -h };

            let intersection = Intersection {
                pos: pos,
                normal_at_surface: normal,
                shader: &self.shader,
                pos_on_surface: Vector2::new(0.0,0.0),
            };

            Some((t, intersection))
        } else {
            None
        };
    }
}
