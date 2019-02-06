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

use crate::ray::Ray;
use crate::shader::Shader;
use crate::world::World;
use nalgebra::{Unit, Vector2, Vector3};

pub struct MirrorShader {
    pub initial_step: f64,
}

impl Shader for MirrorShader {
    fn get_appearance_for(
        &self,
        intersection_pos: Vector3<f64>,
        ray_dir: Vector3<f64>,
        surface_normal: Vector3<f64>,
        world: &World,
        _surface_pos: Vector2<f64>,
        recursion_depth: f64,
    ) -> Vector3<f64> {
        if recursion_depth < 1.0 {
            return Vector3::new(0.0, 0.0, 0.0);
        }
        let unit_normal = surface_normal.normalize();
        let othogonal_part = unit_normal.dot(&-ray_dir) * unit_normal;
        let mirror_ray_dir = (2.0 * othogonal_part + ray_dir).normalize();
        let mirror_ray = Ray {
            start: intersection_pos + mirror_ray_dir * self.initial_step,
            dir: Unit::new_normalize(mirror_ray_dir),
        };
        world.appearance(mirror_ray, recursion_depth - 1.0)
    }
}
