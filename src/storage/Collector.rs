use crate::storage::PrimitiveStorage::PrimitiveStorage;
use crate::world::Interceptable;
use crate::storage::Bounded;

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
	
	pub fn into_storage(self) -> Box<Interceptable> {
		//self.elements.append(&mut self.bounded_elements); //TODO
		return Box::new(PrimitiveStorage{ elements: self.elements})
	}
}