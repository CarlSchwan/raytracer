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

use crate::intersection::Intersection;
use crate::ray::Ray;
use crate::world::light::Light;
use image::Rgba;
use na::Vector3;
use std::f64;

pub mod light;
pub mod plane;
pub mod sphere;
pub mod triangle;

pub trait Interceptable {
    fn intercept(&self, ray: &Ray) -> Option<(f64, Intersection)>;
}

pub struct World {
    pub elements: Box<Interceptable>,
    pub lights: Vec<Light>,
}

impl World {
    pub fn new(elements: Box<Interceptable>, lights: Vec<Light>) -> Self {
        World { elements, lights }
    }

    pub fn color(&self, ray: Ray, recursion_depth: f64) -> Rgba<u8> {
        if let Some(intersection) = self.next_intersection(&ray) {
            // touch something
            intersection.get_color(ray.dir.into_inner(), self, recursion_depth)
        } else {
            // background color
            Rgba([0, 0, 0, 255])
        }
    }

    pub fn appearance(&self, ray: Ray, recursion_depth: f64) -> Vector3<f64> {
        if let Some(intersection) = self.next_intersection(&ray) {
            // touch something
            intersection.get_appearance(ray.dir.into_inner(), self, recursion_depth)
        } else {
            Vector3::new(0.0, 0.0, 0.0)
        }
    }

    pub fn next_intersection(&self, ray: &Ray) -> Option<Intersection> {
        return if let Some((_dist, int)) = self.intercept(ray) {
            Some(int)
        } else {
            None
        };
    }

    pub fn intercept(&self, ray: &Ray) -> Option<(f64, Intersection)> {
        self.elements.intercept(ray)
    }
}
