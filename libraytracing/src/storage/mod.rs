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
use na::Vector3;

pub mod bv_storage;
pub mod collector;
pub mod primitive_storage;

pub trait Bounded: Interceptable {
    fn get_min(&self) -> Vector3<f64>;
    fn get_max(&self) -> Vector3<f64>;
}

impl From<Box<Bounded>> for Box<Interceptable> {
    fn from(element: Box<Bounded>) -> Self {
        Box::new(InterceptFromBound { child: element })
    }
}

struct InterceptFromBound {
    child: Box<Bounded>,
}

impl Interceptable for InterceptFromBound {
    fn intercept(&self, ray: &Ray) -> Option<(f64, Intersection)> {
        self.child.intercept(ray)
    }
}
