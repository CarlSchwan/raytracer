/**
 * Copyright © 2019
 * Sami Shalayel <sami.shalayel@tutamail.com>,
 * Carl Schwan <carl@carlschwan.eu>,
 * Daniel Freiermuth <d_freiermu14@cs.uni-kl.de>
 *
 * This work is free. You can redistribute it and/or modify it under the
 * terms of the Do What The Fuck You Want To Public License, Version 2,
 * as published by Sam Hocevar. See the LICENSE file for more details.
 * 
 * This program is free software. It comes without any warranty, to
 * the extent permitted by applicable law. You can redistribute it
 * and/or modify it under the terms of the Do What The Fuck You Want
 * To Public License, Version 2, as published by Sam Hocevar. See the LICENSE
 * file for more details. **/

#[macro_use]
extern crate criterion;
extern crate image;
extern crate nalgebra as na;
extern crate libraytracing;

use criterion::Criterion;
use libraytracing::camera::equilinear_camera::*;
use libraytracing::camera::Camera;
use libraytracing::error::Error;
use libraytracing::obj::FileParser;
use libraytracing::shader::mirror_shader::MirrorShader;
use libraytracing::shader::*;
use libraytracing::world::World;
use libraytracing::world::light::Light;
use libraytracing::world::plane::*;
use libraytracing::world::sphere::*;
use na::Vector3;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
use libraytracing::storage::primitive_storage::PrimitiveStorage;

fn parsing_ducky(c: &mut Criterion) {
    c.bench_function("parse ducky.obj",
        |b| b.iter(
            || {
                let mut file_parser = FileParser::new();
                file_parser.parse("ducky.obj".to_string()).expect("file ducky.obj to benchmark")
            }
        )
    );
}

fn ducky_primitive_storage(c: &mut Criterion) {
    c.bench_function("ducky_primitive_storage",
        |b| b.iter(
            || {
                let mut file_parser = FileParser::new();
                file_parser.parse("ducky.obj".to_string()).expect("file ducky.obj to benchmark");
                let mut elements = file_parser.elements;
                
                let mut lights = Vec::new();
                lights.push(Light::new(700.0, 50.0, 600.0, Vector3::new(1.0, 1.0, 1.0)));

                let mut cam = EquilinearCamera {
                    width: 5,
                    height: 5,
                    roll: 0.0, // down-up
                    pitch: 3.7, //right-left
                    yaw: 0.0,   //rotation counterclockwise-clockwise
                    pos: Vector3::new(100.0, 50.0, 200.0),
                    vertical_viewangle: 40.0,
                };
                let w = World::new(Box::new(PrimitiveStorage { elements: elements.elements }), lights);
                let _image = cam.render(&w, false);
            }
        )
    );
}

fn ducky_bv_storage(c: &mut Criterion) {
    c.bench_function("ducky_bv_storage",
        |b| b.iter(
            || {
                let mut file_parser = FileParser::new();
                file_parser.parse("ducky.obj".to_string()).expect("file ducky.obj to benchmark");
                let mut elements = file_parser.elements;
                
                let mut lights = Vec::new();
                lights.push(Light::new(700.0, 50.0, 600.0, Vector3::new(1.0, 1.0, 1.0)));

                let mut cam = EquilinearCamera {
                    width: 5,
                    height: 5,
                    roll: 0.0, // down-up
                    pitch: 3.7, //right-left
                    yaw: 0.0,   //rotation counterclockwise-clockwise
                    pos: Vector3::new(100.0, 50.0, 200.0),
                    vertical_viewangle: 40.0,
                };
                let w = World::new(elements.into_storage(), lights);
                let _image = cam.render(&w, false);
            }
        )
    );
}

criterion_group!(benches, parsing_ducky, ducky_bv_storage, ducky_primitive_storage);
criterion_main!(benches);

