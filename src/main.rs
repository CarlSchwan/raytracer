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
use crate::shader::*;
use crate::shader::mirror_shader::MirrorShader;
use wavefront_obj::obj::*;
use std::env;
use std::fs::File;
use std::io::Read;

mod helpers;
mod intersection;
mod ray;
mod world;
mod shader;

#[derive(Debug)]
enum Error {
    ParseError(wavefront_obj::ParseError),
    Error(String),
}

fn parse_obj_file(path: String) -> Result<std::vec::Vec<std::boxed::Box<world::Interceptable>>, Error> {
    let mut elements: std::vec::Vec<std::boxed::Box<world::Interceptable>> = Vec::new();
    let mut file = File::open(path).expect("file don't exist");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("error reading file");

    let obj_set = match parse(contents) {
        Err(e) => Err(Error::ParseError(e)),
        Ok(a) => Ok(a),
    }?;
    for object in obj_set.objects {
        for geometry in object.geometry {
            for shape in geometry.shapes {
                let primitive = &shape.primitive;
                match primitive {
                    Primitive::Triangle(u, v, w) => {
                        let vertices_a = object.vertices[u.0];
                        let a = Vector3::new(vertices_a.x, vertices_a.y, vertices_a.z);

                        let vertices_b = object.vertices[v.0];
                        let b = Vector3::new(vertices_b.x, vertices_b.y, vertices_b.z);

                        let vertices_c = object.vertices[w.0];
                        let c = Vector3::new(vertices_c.x, vertices_c.y, vertices_c.z);

                        elements.push(Box::new(world::triangle::Triangle { a, b, c, shader: get_phong(Rgba::from_channels(0.0, 1.0, 0.0, 1.0)) }));
                    },
                    _ => (),
                };
            }
        }
    }

    Ok(elements)
}

fn main() {
    let mut elements: std::vec::Vec<std::boxed::Box<world::Interceptable>> = Vec::new();
    for argument in env::args().skip(1) {
        elements.append(&mut parse_obj_file(argument).expect("parse error"));
    }

    let green_shader = get_phong(Rgba::from_channels(0.0, 1.0, 0.0, 1.0));
    let red_shader = get_phong(Rgba::from_channels(1.0, 0.0, 0.0, 1.0));
    let blue_shader = get_phong(Rgba::from_channels(0.0, 0.0, 1.0, 1.0));

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
    lights.push(Light::new(0.0, -10.0, 6.0, Rgba::from_channels(1.0, 0.5, 1.0, 1.0)));

    let w = world::World::new(1200, 800, elements, lights);
    //w.render().save(io::stdout(), image::ImageFormat::PNG);
    let image = w.render();
    image.save("./output.png");
}
