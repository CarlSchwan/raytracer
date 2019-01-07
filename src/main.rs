extern crate nalgebra as na;

// imageBuffer
extern crate image;

// parse wavefront_obj
extern crate wavefront_obj;

use crate::world::sphere::Sphere;
use image::{Pixel, Rgba};
use na::Vector3;

mod helpers;
mod intersection;
mod ray;
mod world;

fn main() {
    /*let img = ImageBuffer::new(512, 512);
    let (width, height) = img.dimensions();
    for y in 0..height {
        for x in 0..width {
        }
    }*/

    let mut elements: std::vec::Vec<std::boxed::Box<world::Interceptable>> = Vec::new();
    elements.push(Box::new(Sphere {
        center: Vector3::new(0.0, -2.0, 5.0),
        radius: 1.0,
        color: Rgba::from_channels(1.0, 0.0, 0.0, 1.0),
        opacity: 1.0,
        reflection: 0.0,
    }));
    let mut lights = Vec::new();

    let w = world::World::new(100, 100, elements, lights);
    //w.render().save(io::stdout(), image::ImageFormat::PNG);
    let image = w.render();
    image.save("./output.png");
}
