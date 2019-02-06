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

use crate::storage::bv_storage::*;
use crate::storage::primitive_storage::PrimitiveStorage;
use crate::storage::Bounded;
use crate::world::Interceptable;

pub struct Collector {
    pub bounded_elements: Vec<Box<Bounded>>,
    pub elements: Vec<Box<Interceptable + 'static>>,
}

impl<'a> Collector {
    pub fn new() -> Self {
        Collector {
            bounded_elements: Vec::new(),
            elements: Vec::new(),
        }
    }
    pub fn add(&mut self, element: Box<Interceptable>) -> () {
        self.elements.push(element);
    }

    pub fn add_bounded(&mut self, element: Box<Bounded>) -> () {
        self.bounded_elements.push(element);
    }

    pub fn into_storage(mut self) -> Box<Interceptable> {
        let bounded_elements = Box::new(BVStorage::new(self.bounded_elements));
        self.elements.push(bounded_elements);
        //self.elements.append(&mut bounded2interceptable(self.bounded_elements));
        Box::new(PrimitiveStorage {
            elements: self.elements,
        })
    }
}
