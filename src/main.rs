extern crate nalgebra as na;

// imageBuffer
extern crate image;

// parse wavefront_obj
extern crate wavefront_obj;

use crate::world::sphere::*;
use crate::world::plane::*;
use crate::world::light::Light;
use na::{Vector3, Unit};
use crate::shader::monochrome_shader::*;
use crate::shader::diffuse_shader::DiffuseShader;
use crate::shader::specular_shader::SpecularShader;
use crate::shader::*;
use crate::shader::mirror_shader::MirrorShader;
use crate::shader::chess_shader::ChessShader;

mod helpers;
mod intersection;
mod ray;
mod world;
mod shader;

fn main() {

    let green_shader = get_phong(Vector3::new(0.0, 1.0, 0.0));
    let green_shader = get_phong(Vector3::new(0.0, 1.0, 0.0));
    let red_shader = get_phong(Vector3::new(1.0, 0.0, 0.0));
    let blue_shader = get_phong(Vector3::new(0.0, 0.0, 1.0));

    let mut elements: std::vec::Vec<std::boxed::Box<world::Interceptable>> = Vec::new();
    elements.push(Box::new(Sphere {
        center: Vector3::new(0.0, 1.0, -6.0),
        radius: 1.0,
        shader: red_shader,
    }));
    elements.push(Box::new(Sphere {
        center: Vector3::new(1.0, -1.0, -5.0),
        radius: 1.0,
        shader: green_shader,
    }));
    elements.push(Box::new(Sphere {
        center: Vector3::new(2.0, 0.0, -9.0),
        radius: 1.0,
        shader: Box::new(MirrorShader{
            initial_step: 0.001,
        }),
    }));
    elements.push(Box::new(Plane {
        normal: Unit::new_normalize(Vector3::new(0.0, 1.0, 0.0)),
        d: 1.0,
        shader: blue_shader,
    }));
    let mut lights = Vec::new();
    lights.push(Light::new(0.0, -10.0, 6.0, Vector3::new(1.0, 0.5, 1.0)));

    let w = world::World::new(300, 100, elements, lights);
    //w.render().save(io::stdout(), image::ImageFormat::PNG);
    let image = w.render();
    image.save("./output.png").expect("Could not save image!");
}
