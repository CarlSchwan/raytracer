use crate::intersection::Intersection;
use crate::ray::Ray;
use crate::world::Interceptable;
use image::Rgba;
use na::Vector3;
use std::f64;

#[derive(Serialize, Deserialize, Debug)]
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
        let b = 2.0 * ray.dir.unwrap().dot(&h); // scalar
        let c = h.dot(&h) - self.radius.powi(2); // scalar

        let delta = b.powi(2) - 4.0 * c;

        if delta >= 0.0 {

            let mut lambdas : Vec<f64> = Vec::new();
            lambdas.push((-b + delta.sqrt()) / 2.0);
            lambdas.push((-b - delta.sqrt()) / 2.0);

            let pos_lambdas : Vec<&f64> = lambdas.iter().filter(|x| *x >= &0.0).collect();


            match min(pos_lambdas) {
                None => { return None; }
                Some(lambda) => {
                    let pos = ray.start + ray.dir.unwrap() * lambda;
                    let pos_to_center = pos - self.center;
                    return Some((lambda,
                                 Intersection {
                                    pos: pos,
                                    normal_at_surface: -pos_to_center,
                                    color: self.color,
                                    reflection: self.reflection,
                                    opacity: 1.0

                        }));
                }
            }
        }
        return None;
    }

}

fn min(v: Vec<&f64>) -> Option<f64> {
    let mut min = f64::INFINITY;
    let mut ret = None;
    for element in v {
        if element < &min {
            min = *element;
            ret = Some(min);
        }
    }
    return ret;
}

