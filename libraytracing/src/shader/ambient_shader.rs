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

use crate::helpers::polar2vector;
use crate::helpers::vector2polar;
use crate::ray::Ray;
use crate::shader::Shader;
use crate::world::World;
use nalgebra::{Rotation3, Unit, Vector2, Vector3};
use std::f64;

pub struct AmbientShader {
    pub color: Vector3<f64>,
}

impl AmbientShader {
    pub fn new(color: Vector3<f64>) -> Box<Shader> {
        Box::new(AmbientShader{color})
    }
    fn evaluate_ray(
        &self,
        world: &World,
        recursion_depth: f64,
        int_pos: Vector3<f64>,
        initial_step: f64,
        ray_dir: Vector3<f64>,
        surface_normal: Vector3<f64>,
    ) -> Vector3<f64> {
        let ray = Ray {
            start: int_pos + ray_dir * initial_step,
            dir: Unit::new_normalize(ray_dir),
        };
        if let Some((dist, int)) = world.intercept(&ray) {
            (int.get_appearance(ray.dir.into_inner(), world, recursion_depth / 5.0)
                * (1.0 / (1.0 + dist / 2.0))
                * (1.0 / (1.0 + dist / 2.0))
                * ray.dir.dot(&surface_normal.normalize()))
            .component_mul(&self.color)
        } else {
            Vector3::new(0.0, 0.0, 0.0)
        }
    }
}

impl Shader for AmbientShader {
    fn get_appearance_for(
        &self,
        intersection_pos: Vector3<f64>,
        _ray_dir: Vector3<f64>,
        surface_normal: Vector3<f64>,
        world: &World,
        _surface_pos: Vector2<f64>,
        recursion_depth: f64,
    ) -> Vector3<f64> {
        if recursion_depth < 1.0 {
            return Vector3::new(0.0, 0.0, 0.0);
        }
        let initial_step = 0.001;
        let (roll, pitch) = vector2polar(&surface_normal);
        let rot_matrix = Rotation3::from_euler_angles(roll, -pitch + f64::consts::FRAC_PI_2, 0.0);

        let ray_dir = rot_matrix * polar2vector(0.0, 0.0);
        let mut appereance = self.evaluate_ray(
            world,
            recursion_depth,
            intersection_pos,
            initial_step,
            ray_dir,
            surface_normal,
        );

        for i in 1..5 {
            let ray_dir = rot_matrix
                * polar2vector(
                    f64::consts::FRAC_PI_2 / 10.0,
                    f64::consts::FRAC_PI_2 * i as f64,
                );
            appereance += self.evaluate_ray(
                world,
                recursion_depth,
                intersection_pos,
                initial_step,
                ray_dir,
                surface_normal,
            );
        }
        for i in 1..7 {
            let ray_dir = rot_matrix
                * polar2vector(
                    f64::consts::FRAC_PI_2 / 5.0,
                    f64::consts::FRAC_PI_2 / 1.5 * i as f64,
                );
            appereance += self.evaluate_ray(
                world,
                recursion_depth,
                intersection_pos,
                initial_step,
                ray_dir,
                surface_normal,
            );
        }
        appereance / 8.0
    }
}
