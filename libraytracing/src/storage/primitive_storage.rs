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
use crate::world::Interceptable;
use std::f64;

pub struct PrimitiveStorage {
    pub elements: Vec<Box<Interceptable>>,
}

impl Interceptable for PrimitiveStorage {
    fn intercept(&self, ray: &Ray) -> Option<(f64, Intersection)> {
        let mut max_distance = f64::INFINITY;
        let mut interception = None;
        for element in &self.elements {
            if let Some((distance, intercept)) = element.intercept(ray) {
                if distance < max_distance {
                    max_distance = distance;
                    interception = Some(intercept);
                }
            }
        }
        return if let Some(int) = interception {
            Some((max_distance, int))
        } else {
            None
        };
    }
}
