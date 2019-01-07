use crate::intersection::Intersection;
use crate::ray::Ray;
use crate::world::Interceptable;
use image::Rgba;
use na::Vector3;

pub struct Triangle {
    pub a: Vector3<f64>,
    pub b: Vector3<f64>,
    pub c: Vector3<f64>,
    pub color: Rgba<f64>,
    pub opacity: f64,
    pub reflection: f64,
}

impl Interceptable for Triangle {
    // Shamelessly stolen from https://en.wikipedia.org/wiki/M%C3%B6ller%E2%80%93Trumbore_intersection_algorithm
    fn intercept(&self, ray: &Ray) -> Option<(f64, Intersection)> {
        let epsylon = 0.0001;
        let edge1 = self.b - self.a;
        let edge2 = self.c - self.a;
        let h = ray.dir.cross(&edge2);
        let a = edge1.dot(&h);
        if a > -epsylon && a < epsylon {
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
        return if t > epsylon
        // ray intersection
        {
            let pos = ray.start + ray.dir.unwrap() * t;
            let h = edge1.cross(&edge2);
            //TODO: do we need to take this angle modulo to get a value in [0, pi/4] ?
            let angle = (h.dot(&ray.dir) / (h.norm() * ray.dir.norm())).acos();

            let normal = if angle.abs() > std::f64::consts::FRAC_PI_4 { h } else { -h };

            let intersection = Intersection {
                pos: pos,
                normal_at_surface: normal,
                color: self.color,
                opacity: self.opacity,
                reflection: self.reflection,
            };

            Some((t, intersection))
        } else {
            None
        };
    }
}
