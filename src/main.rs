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
use crate::shader::mirror_shader::MirrorShader;
use crate::shader::*;
use crate::world::light::Light;
use crate::world::plane::*;
use crate::world::sphere::*;
use na::Vector3;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::storage::primitive_storage::PrimitiveStorage;

fn main() -> Result<(), Error> {
    println!("Start parsing");
    // Parse file given as args
    let mut file_parser = FileParser::new();

    for argument in env::args().skip(1) {
        file_parser.parse(argument)?;
    }
    let mut elements = file_parser.elements;
    println!("End parsing");

    // add some spheres
//    let green_shader = get_phong(Vector3::new(0.0, 1.0, 0.0));
//    let red_shader = get_phong(Vector3::new(1.0, 0.0, 0.0));
//    let blue_shader = get_phong(Vector3::new(0.0, 0.0, 1.0));
//    let red_blue_shader = Box::new(chess_shader::ChessShader {
//        shader1: red_shader,
//        shader2: blue_shader,
//        size: 1.0,
//    });
//    let red_shader = get_phong(Vector3::new(1.0, 0.0, 0.0));
//    let blue_shader = get_phong(Vector3::new(0.0, 0.0, 1.0));
//    let red_blue_shader2 = Box::new(chess_shader::ChessShader {
//        shader1: red_shader,
//        shader2: blue_shader,
//        size: 1.0,
//    });
//    let red_shader = get_phong(Vector3::new(1.0, 0.0, 0.0));
//    let blue_shader = get_phong(Vector3::new(0.0, 0.0, 1.0));
//
//    elements.add(Box::new(Sphere {
//        center: Vector3::new(1.0, 1.0, 4.0),
//        radius: 1.0,
//        shader: red_blue_shader,
//        pitch: 0.0,
//        roll: 0.0,
//        yaw: 0.0,
//    }));
//    elements.add(Box::new(Sphere {
//        center: Vector3::new(0.0, 1.0, 6.0),
//        radius: 1.0,
//        shader: red_shader,
//        pitch: 0.0,
//        roll: 0.0,
//        yaw: 0.0,
//    }));
//    elements.add(Box::new(Sphere {
//        center: Vector3::new(1.0, -1.0, 5.0),
//        radius: 1.0,
//        shader: green_shader,
//        pitch: 0.0,
//        roll: 0.0,
//        yaw: 0.0,
//    }));
//    elements.add(Box::new(Sphere {
//        center: Vector3::new(2.0, 0.0, 9.0),
//        radius: 1.0,
//        shader: Box::new(MirrorShader {
//            initial_step: 0.001,
//        }),
//        pitch: 0.0,
//        roll: 0.0,
//        yaw: 0.0,
//    }));
//    elements.add(Box::new(Plane {
//        a: Vector3::new(0.0, 1.0, 0.0),
//        b: Vector3::new(1.0, 1.0, 0.0),
//        c: Vector3::new(0.0, 1.0, 1.0),
//        shader: red_blue_shader2,
//    }));

    // add light
    let mut lights = Vec::new();
    lights.push(Light::new(0.0, -10.0, 6.0, Vector3::new(1.0, 1.0, 1.0)));

    let mut cam = EquilinearCamera {
        width: 192,
        height: 108,
        roll: -0.3, // down-up
        pitch: 0.0, //right-left
        yaw: 0.0,   //rotation counterclockwise-clockwise
        pos: Vector3::new(0.0, 150.0, -600.0),
        vertical_viewangle: 40.0,
    };
    let w = world::World::new(Box::new(PrimitiveStorage { elements: elements.elements }), lights);
    let image = cam.render(&w);
    let name = format!("output{:?}.png", SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs());
    image.save(name).expect("Could not save image!");
    Ok(())
}
