extern crate image;
extern crate nalgebra as na;
extern crate wavefront_obj;

mod camera;
mod error;
mod helpers;
mod intersection;
mod obj;
mod ray;
mod shader;
mod storage;
mod world;

use crate::camera::equilinear_camera::*;
use crate::camera::Camera;
use crate::error::Error;
use crate::obj::FileParser;
use crate::shader::chess_shader::ChessShader;
use crate::shader::diffuse_shader::DiffuseShader;
use crate::shader::mirror_shader::MirrorShader;
use crate::shader::monochrome_shader::*;
use crate::shader::specular_shader::SpecularShader;
use crate::shader::*;
use crate::world::light::Light;
use crate::world::plane::*;
use crate::world::sphere::*;
use na::{Unit, Vector3};
use std::env;
use wavefront_obj::obj::*;

fn main() -> Result<(), Error> {
    // Parse file given as args
    let mut file_parser = FileParser::new();

    for argument in env::args().skip(1) {
        file_parser.parse(argument)?;
    }
    let mut elements = file_parser.elements;

    // add some spheres
    let green_shader = get_phong(Vector3::new(0.0, 1.0, 0.0));
    let green_shader = get_phong(Vector3::new(0.0, 1.0, 0.0));
    let red_shader = get_phong(Vector3::new(1.0, 0.0, 0.0));
    let blue_shader = get_phong(Vector3::new(0.0, 0.0, 1.0));

    elements.add(Box::new(Sphere {
        center: Vector3::new(0.0, 1.0, 6.0),
        radius: 1.0,
        shader: red_shader,
    }));
    elements.add(Box::new(Sphere {
        center: Vector3::new(1.0, -1.0, 5.0),
        radius: 1.0,
        shader: green_shader,
    }));
    elements.add(Box::new(Sphere {
        center: Vector3::new(2.0, 0.0, 9.0),
        radius: 1.0,
        shader: Box::new(MirrorShader {
            initial_step: 0.001,
        }),
    }));
    elements.add(Box::new(Plane {
        normal: Unit::new_normalize(Vector3::new(0.0, 1.0, 0.0)),
        d: 1.0,
        shader: blue_shader,
    }));

    // add light
    let mut lights = Vec::new();
    lights.push(Light::new(0.0, -10.0, 6.0, Vector3::new(1.0, 0.5, 1.0)));

    let cam = EquilinearCamera {
        width: 300,
        height: 150,
        roll: -0.2, // down-up
        pitch: 0.2, //right-left
        yaw: 0.2,   //rotation counterclockwise-clockwise
        pos: Vector3::new(0.0, 0.0, 0.0),
        vertical_viewangle: 40.0,
    };

    let w = world::World::new(elements.into_storage(), lights);
    let image = cam.render(&w);
    image.save("./output.png").expect("Could not save image!");
    Ok(())
}
