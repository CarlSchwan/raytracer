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

use crate::shader::Shader;
use crate::world::World;
use na::{Vector2, Vector3};

pub struct MonochromeShader {
    pub color: Vector3<f64>,
}

impl Shader for MonochromeShader {
    fn get_appearance_for(
        &self,
        _intersection_pos: Vector3<f64>,
        _ray_dir: Vector3<f64>,
        _surface_normal: Vector3<f64>,
        _world: &World,
        _surface_pos: Vector2<f64>,
        _recursion_depth: f64,
    ) -> Vector3<f64> {
        self.color
    }
}
