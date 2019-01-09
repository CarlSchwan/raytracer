extern crate nalgebra as na;

// imageBuffer
extern crate image;

// parse wavefront_obj
extern crate wavefront_obj;

use crate::world::sphere::*;
use crate::world::plane::*;
use crate::world::light::Light;
use image::{Pixel, Rgba};
use na::{Vector3, Unit};
use crate::shader::monochrome_shader::*;
use crate::shader::diffuse_shader::DiffuseShader;

mod helpers;
mod intersection;
mod ray;
mod world;
mod shader;

fn main() {

    let green_shader = MonochromeShader {color: Rgba::from_channels(0.0, 1.0, 0.0, 1.0)};
    let red_shader = MonochromeShader {color: Rgba::from_channels(1.0, 0.0, 0.0, 1.0)};
    let blue_shader = MonochromeShader {color: Rgba::from_channels(0.0, 0.0, 1.0, 1.0)};

    let red_diffuse_shader = DiffuseShader { color: Rgba::from_channels(1.0, 0.0, 0.0, 1.0), reflection: 0.5};
    let red_diffuse_shader2 = DiffuseShader { color: Rgba::from_channels(1.0, 0.0, 0.0, 1.0), reflection: 0.5};
    let red_diffuse_shader3 = DiffuseShader { color: Rgba::from_channels(1.0, 0.0, 0.0, 1.0), reflection: 0.5};

    let mut elements: std::vec::Vec<std::boxed::Box<world::Interceptable>> = Vec::new();
    elements.push(Box::new(Sphere {
        center: Vector3::new(0.0, 1.0, -6.0),
        radius: 1.0,
        shader: Box::new(red_diffuse_shader),
    }));
    elements.push(Box::new(Sphere {
        center: Vector3::new(1.0, -1.0, -5.0),
        radius: 1.0,
        shader: Box::new(red_diffuse_shader2),
    }));
    elements.push(Box::new(Plane {
        normal: Unit::new_normalize(Vector3::new(0.0, 1.0, 0.0)),
        d: 1.0,
        shader: Box::new(red_diffuse_shader3),
    }));
    let mut lights = Vec::new();
    lights.push(Light::new(0.0, -10.0, 6.0, Rgba::from_channels(1.0, 0.5, 1.0, 1.0)));

    let w = world::World::new(400, 400, elements, lights);
    //w.render().save(io::stdout(), image::ImageFormat::PNG);
    let image = w.render();
    image.save("./output.png");
}
