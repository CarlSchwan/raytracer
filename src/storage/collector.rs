use crate::storage::bv_storage::*;
use crate::storage::primitive_storage::PrimitiveStorage;
use crate::storage::Bounded;
use crate::world::Interceptable;

pub struct Collector {
    bounded_elements: Vec<Box<Bounded>>,
    elements: Vec<Box<Interceptable + 'static>>,
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

fn bounded2interceptable(bounded: Vec<Box<Bounded>>) -> Vec<Box<Interceptable>> {
    let mut result = Vec::new();
    for e in bounded {
        result.push(Box::from(e));
    }
    result
}
