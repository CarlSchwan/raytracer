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

use na::Vector3;

#[derive(Debug)]
pub struct Light {
    pub color: Vector3<f64>,
    pub pos: Vector3<f64>,
    pub intensity: f64,
}

impl Light {
    pub fn new(x: f64, y: f64, z: f64, color: Vector3<f64>) -> Self {
        let pos = Vector3::new(x, y, z);
        let intensity = 1.0;
        Light {
            color,
            pos,
            intensity,
        }
    }
}
