use crate::intersection::Intersection;
use crate::ray::Ray;
use crate::world::Interceptable;
use image::Rgba;
use na::Vector3;

pub struct Sphere {
    pub center: Vector3<f64>,
    pub radius: f64,
    pub color: Rgba<f64>,
    pub opacity: f64,
    pub reflection: f64,
}

impl Interceptable for Sphere {
    fn intercept(&self, ray: &Ray) -> Option<(f64, Intersection)> {
        let h = ray.start - self.center; // vector, needs to be summed/normed before utilisation
        let a = 1.0; // scalar
        let b = 2.0 * h.dot(&ray.dir.unwrap()); // scalar
        let c = h.norm_squared() - self.radius.powi(2); // scalar

        let delta = b.powi(2) - 4.0 * a * c;

        let epsylon = 0.0001; //TODO: adjust precision (do we need more/less?)

        return if delta > epsylon {
            // 2 points : take the smallest positive lambda
            // here, lambda is the coefficient that you multiply with ray.dir and add to ray.start
            // to get the intersection point. So because our ray.dir is a unit vector, it is the
            // distance between ray start and intersection point

            let lambda_1 = -b + delta.sqrt();
            let lambda_1 = lambda_1 / (2.0 * a);

            let lambda_2 = -b - delta.sqrt();
            let lambda_2 = lambda_2 / (2.0 * a);

            let min_pos_lambda = if lambda_1 > epsylon && lambda_2 > epsylon {
                lambda_1.min(lambda_2)
            } else {
                lambda_1.max(lambda_2)
            };

            let pos = ray.start + ray.dir.unwrap() * min_pos_lambda;

            let pos_to_center = pos - self.center;

            //cosinus formula for angle between raydir and intersection-center vector
            //remove pi/4 to get the angle between tangeant at the intersection and ray
            // TODO: maybe we need to take the absolute value here ...
            let angle = (pos_to_center.dot(&-ray.dir) / (pos_to_center.norm() * ray.dir.norm()))
                .acos()
                - (std::f64::consts::PI / 4.0);

            let intersection = Intersection {
                pos: pos,
                color: self.color,
                opacity: self.opacity,
                reflection: self.reflection,
            };

            Some((min_pos_lambda, intersection))
        } else if delta < -epsylon {
            //sphere behind light
            None
        } else {
            //delta is approximatively 0
            let lambda = -b / (2.0 * a);

            if lambda > epsylon {
                let angle = std::f64::consts::PI / 4.0; //because there is only one intersection : the ray is tangeant to intersection
                let pos = ray.start + ray.dir.unwrap() * lambda;
                let intersection = Intersection {
                    pos: pos,
                    color: self.color,
                    opacity: self.opacity,
                    reflection: self.reflection,
                };
                Some((lambda, intersection))
            } else {
                None
            }
        };
    }
}
