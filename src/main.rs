extern crate image;
extern crate nalgebra as na;

use libraytracing::camera::equilinear_camera::*;
use libraytracing::camera::Camera;
use libraytracing::error::Error;
use libraytracing::obj::FileParser;
use libraytracing::shader::mirror_shader::MirrorShader;
use libraytracing::shader::ambient_shader::AmbientShader;
use libraytracing::shader::*;
use libraytracing::world::World;
use libraytracing::world::light::Light;
use libraytracing::world::plane::*;
use libraytracing::world::sphere::*;
use libraytracing::storage::primitive_storage::PrimitiveStorage;
use na::Vector3;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

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
    let green_shader: Box<Shader> = Box::new(AmbientShader {
        color: Vector3::new(0.0, 1.0, 0.0),
    });
    let red_shader = get_phong(Vector3::new(1.0, 0.0, 0.0));
    let blue_shader = get_phong(Vector3::new(0.0, 0.0, 1.0));
    let red_blue_shader = Box::new(chess_shader::ChessShader {
        shader1: red_shader,
        shader2: blue_shader,
        size: 1.0,
    });
    let red_shader = get_phong(Vector3::new(1.0, 0.0, 0.0));
    let blue_shader = get_phong(Vector3::new(0.0, 0.0, 1.0));
    let red_blue_shader2 = Box::new(chess_shader::ChessShader {
        shader1: red_shader,
        shader2: blue_shader,
        size: 1.0,
    });
    let red_shader = get_phong(Vector3::new(1.0, 0.0, 0.0));
    let blue_shader = get_phong(Vector3::new(0.0, 0.0, 1.0));

    elements.add(Box::new(Sphere {
        center: Vector3::new(1.0, 1.0, 4.0),
        radius: 1.0,
        shader: red_blue_shader,
        pitch: 0.0,
        roll: 0.0,
        yaw: 0.0,
    }));
    elements.add(Box::new(Sphere {
        center: Vector3::new(0.0, 1.0, 6.0),
        radius: 1.0,
        shader: red_shader,
        pitch: 0.0,
        roll: 0.0,
        yaw: 0.0,
    }));
    elements.add(Box::new(Sphere {
        center: Vector3::new(2.0, -1.0, 5.0),
        radius: 1.0,
        shader: get_phong(Vector3::new(0.0, 1.0, 0.0)),
        pitch: 0.0,
        roll: 0.0,
        yaw: 0.0,
    }));
    let mirror: Box<Shader> = Box::new(MirrorShader {
        initial_step: 0.001,
    });
    elements.add(Box::new(Sphere {
        center: Vector3::new(1.0, 0.0, 7.0),
        radius: 1.0,
        shader: mirror + 0.2 * get_phong(Vector3::new(0.0, 0.0, 1.0)),
        pitch: 0.0,
        roll: 0.0,
        yaw: 0.0,
    }));
    elements.add(Box::new(Plane {
        a: Vector3::new(0.0, 1.0, 0.0),
        b: Vector3::new(1.0, 1.0, 0.0),
        c: Vector3::new(0.0, 1.0, 1.0),
        shader: red_blue_shader2,
    }));

    // add light
    let mut lights = Vec::new();
    lights.push(Light::new(0.0, -10.0, 6.0, Vector3::new(1.0, 0.5, 1.0)));
    lights.push(Light::new(6.0, -10.0, 6.0, Vector3::new(0.5, 1.0, 1.0)));

    let mut cam = EquilinearCamera {
        width: 1200,
        height: 800,
        roll: 0.0, // down-up
        pitch: 3.7, //right-left
        yaw: 0.0,   //rotation counterclockwise-clockwise
        pos: Vector3::new(200.0, 0.0, 300.0),
        vertical_viewangle: 40.0,
    };
    let w = World::new(Box::new(PrimitiveStorage { elements: elements.elements }), lights);
    let image = cam.render(&w, true);
    let name = format!("output{:?}.png", SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs());
    image.save(name).expect("Could not save image!");
    Ok(())
}
