/**
 * Copyright © 2019
 * Sami Shalayel <sami.shalayel@tutamail.com>,
 * Carl Schwan <carl@carlschwan.eu>,
 * Daniel Freiermuth <d_freiermu14@cs.uni-kl.de>
 *
 * This work is free. You can redistribute it and/or modify it under the
 * terms of the Do What The Fuck You Want To Public License, Version 2,
 * as published by Sam Hocevar. See the LICENSE file for more details.
 * 
 * This program is free software. It comes without any warranty, to
 * the extent permitted by applicable law. You can redistribute it
 * and/or modify it under the terms of the Do What The Fuck You Want
 * To Public License, Version 2, as published by Sam Hocevar. See the LICENSE
 * file for more details. **/

use crate::helpers::vector2polar;
use crate::intersection::Intersection;
use crate::ray::Ray;
use crate::shader::Shader;
use crate::storage::Bounded;
use crate::world::Interceptable;
use na::{Rotation3, Vector2, Vector3};
use std::f64;

pub struct Sphere {
    pub center: Vector3<f64>,
    pub radius: f64,
    pub shader: Box<Shader>,
    pub roll: f64,
    pub pitch: f64,
    pub yaw: f64,
}

impl Interceptable for Sphere {
    fn intercept(&self, ray: &Ray) -> Option<(f64, Intersection)> {
        let h = ray.start - self.center; // vector, needs to be summed/normed before utilisation

        let b = 2.0 * ray.dir.into_inner().dot(&h); // scalar
        let c = h.dot(&h) - self.radius.powi(2); // scalar

        let delta = b.powi(2) - 4.0 * c;

        if delta >= 0.0 {
            let mut lambdas: Vec<f64> = Vec::new();
            lambdas.push((-b + delta.sqrt()) / 2.0);
            lambdas.push((-b - delta.sqrt()) / 2.0);

            let pos_lambdas: Vec<&f64> = lambdas.iter().filter(|x| *x >= &0.0).collect();

            match min(pos_lambdas) {
                None => {
                    return None;
                }
                Some(lambda) => {
                    let pos = ray.start + ray.dir.into_inner() * lambda;
                    let pos_to_center = pos - self.center;
                    let vector_on_surface =
                        Rotation3::from_euler_angles(self.roll, self.pitch, self.yaw)
                            * pos_to_center;
                    let (vertical_angle, horizontal_angle) = vector2polar(&vector_on_surface);
                    return Some((
                        lambda,
                        Intersection {
                            pos: pos,
                            normal_at_surface: pos_to_center,
                            shader: &self.shader,
                            pos_on_surface: Vector2::new(
                                vertical_angle * self.radius,
                                horizontal_angle * self.radius,
                            ),
                        },
                    ));
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

impl Bounded for Sphere {
    fn get_min(&self) -> Vector3<f64> {
        Vector3::new(
            self.center.x - self.radius,
            self.center.y - self.radius,
            self.center.z - self.radius,
        )
    }
    fn get_max(&self) -> Vector3<f64> {
        Vector3::new(
            self.center.x + self.radius,
            self.center.y + self.radius,
            self.center.z + self.radius,
        )
    }
}
