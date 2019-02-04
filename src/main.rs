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
use crate::shader::ambient_shader::AmbientShader;
use crate::shader::mirror_shader::MirrorShader;
use crate::shader::*;
use crate::world::light::Light;
use crate::world::plane::*;
use crate::world::sphere::*;
use na::{Vector3, Rotation3};
use std::env;
use std::f64;
use std::rc::Rc;
use wavefront_obj::obj::*;
use std::io;           
use std::io::prelude::*;                                                                                                                       

fn main() -> Result<(), Error> {
    // Parse file given as args
    let mut file_parser = FileParser::new();

    for argument in env::args().skip(1) {
        file_parser.parse(argument)?;
    }
    let mut elements = file_parser.elements;
	print!("Parsen abgeschlossen\n");

    // add some spheres
//    let gr_shader: Box<Shader> = Box::new(AmbientShader {
//        color: Vector3::new(0.0, 1.0, 0.0),
//    });
    let black_shader = get_phong(Vector3::new(0.0, 0.0, 0.0));
    let white_shader = get_phong(Vector3::new(1.0, 1.0, 1.0));
    let bw_shader = Box::new(chess_shader::ChessShader {
        shader1: black_shader,
        shader2: white_shader,
        size: 1.0,
    });
    let white_shader = get_phong(Vector3::new(1.0, 1.0, 1.0));
//    let blue_shader = get_phong(Vecto_r3::new(0.0, 0.0, 1.0));
//    let red_blue_shader2 = Box::new(chess_shader::ChessShader {
//        shader1: red_shader,
//        shader2: blue_shader,
//        size: 1.0,
//    });
    let red_shader = get_phong(Vector3::new(1.0, 0.0, 0.0));
    let blue_shader = get_phong(Vector3::new(0.0, 0.0, 1.0));

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
	let sp = Box::new(Sphere {
        center: Vector3::new(-6.0, 11.0, -7.0),
        radius: 3.0,
        shader: white_shader,
        pitch: 0.0,
        roll: 0.0,
        yaw: 0.0,
    });
    elements.add(sp);
    let mirror: Box<Shader> = Box::new(MirrorShader {
        initial_step: 0.001,
    });
    elements.add(Box::new(Sphere {
        center: Vector3::new(9.0, 0.0, 3.0),
        radius: 7.0,
        shader: mirror + 0.1 * get_phong(Vector3::new(0.0, 0.0, 1.0)),
        pitch: 0.0,
        roll: 0.0,
        yaw: 0.0,
    }));
	let mirror: Box<Shader> = Box::new(MirrorShader {
        initial_step: 0.001,
    });
    elements.add(Box::new(Plane {
        a: Vector3::new(11.0, 0.0, -16.0),
        b: Vector3::new(0.0, 0.0, -16.0),
        c: Vector3::new(0.0, 10.0, -16.0),
        shader: mirror,
    }));
	let mirror: Box<Shader> = Box::new(MirrorShader {
        initial_step: 0.001,
    });
    elements.add(Box::new(Plane {
        a: Vector3::new(11.0, 0.0, 16.0),
        b: Vector3::new(0.0, 0.0, 16.0),
        c: Vector3::new(0.0, 10.0, 16.0),
        shader: mirror,
    }));
    elements.add(Box::new(Plane {
        a: Vector3::new(11.0, 0.0, 11.0),
        b: Vector3::new(0.0, 0.0, 11.0),
        c: Vector3::new(11.0, 0.0, 0.0),
        shader: bw_shader,
    }));

    // add light
    let mut lights = Vec::new();
    lights.push(Light::new(-7.0, 20.0, 5.0, Vector3::new(1.0, 0.5, 1.0)));
    //lights.push(Light::new(6.0, -10.0, 6.0, Vector3::new(0.5, 1.0, 1.0)));

    let mut cam = EquilinearCamera {
        width: 500,
        height: 500,
        roll: 0.0, // down-up
        pitch: f64::consts::FRAC_PI_2*2.0, //right-left
        yaw: 0.0,   //rotation counterclockwise-clockwise
        pos: Vector3::new(0.0, 10.0, 10.0),
        vertical_viewangle: 130.0,
    };
    
    let w = world::World::new(elements.into_storage(), lights);
	print!("Storage vorbereitet\n");
	let n = 200;
	let rot_c = Rotation3::from_euler_angles(0.0, f64::consts::PI * 2.0 / n as f64, 0.0);
	let base_c = Vector3::new(0.0, 15.0, 0.0);
	let mut diff_c = Vector3::new(0.0, 0.0, 15.0);
	
	print!("image 0/{}", n);
	for i in 0..n {
        cam.pos = base_c + diff_c;
		diff_c = rot_c * diff_c;
		
		cam.point_at(Vector3::new(0.0, 10.0, 0.0));
        let image = cam.render(&w);
        image.save(format!("./output{:09}.png", i))?;
		print!("\rimage {}/{}", i, n);
	    io::stdout().flush().ok().expect("Could not flush stdout");
    }
	print!("\n");
    Ok(())
}
