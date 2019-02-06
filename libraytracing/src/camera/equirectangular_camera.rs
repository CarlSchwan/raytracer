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

use crate::camera::*;
use crate::helpers::polar2vector;
use crate::ray::Ray;
use crate::world::World;
use image::{DynamicImage, GenericImage};
use na::{Rotation3, Unit, Vector3};
use std::f64;

pub struct EquirectangularCamera {
    pub height: u32,
    pub roll: f64,
    pub pitch: f64,
    pub yaw: f64,
    pub pos: Vector3<f64>,
}

impl Camera for EquirectangularCamera {
    fn render(&self, world: &World, progress: bool) -> DynamicImage {
        let width = self.height * 2;
        let mut img = DynamicImage::new_rgb8(width, self.height);

        let rot_matrix = Rotation3::from_euler_angles(self.roll, self.pitch, self.yaw);
        for x in 0..width {
            for y in 0..self.height {
                let gamma = (y as f64) / (self.height as f64) * f64::consts::PI;
                let phi = (2.0 * (x as f64) / (width as f64) - 1.0) * f64::consts::PI;

                let dir = rot_matrix * polar2vector(gamma, phi).normalize();
                let ray = Ray {
                    dir: Unit::new_normalize(dir),
                    start: self.pos,
                };
                let rgb = world.color(ray, 10.0);

                img.put_pixel(x, self.height - y - 1, rgb);
            }
        }
        img
    }
}
