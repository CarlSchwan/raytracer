use crate::helpers::*;
use crate::shader::chess_shader::*;
use crate::shader::diffuse_shader::*;
use crate::shader::monochrome_shader::*;
use crate::shader::specular_shader::*;
use crate::world::World;
use image::Rgba;
use nalgebra::{Vector2, Vector3};

pub trait Shader {
    fn get_appearance_for(
        &self,
        intersection_pos: Vector3<f64>,
        ray_dir: Vector3<f64>,
        surface_normal: Vector3<f64>,
        world: &World,
        surface_pos: Vector2<f64>,
        recursion_depth: u64,
    ) -> Vector3<f64>;
    //default implementation to get a rgb<u8> (instead of a vector<f64>)
    fn get_color_for(
        &self,
        intersection_pos: Vector3<f64>,
        ray_dir: Vector3<f64>,
        surface_normal: Vector3<f64>,
        world: &World,
        surface_pos: Vector2<f64>,
        recursion_depth: u64,
    ) -> Rgba<u8> {
        let val = &self.get_appearance_for(
            intersection_pos,
            ray_dir,
            surface_normal,
            world,
            surface_pos,
            recursion_depth,
        );
        let u8val = val.map(|x| (x * 255.0).min(255.0).max(0.0) as u8);
        vector2color(&u8val)
    }
}

pub fn get_phong(color: Vector3<f64>) -> AdditiveShader {
    let diffuse_shader: Box<Shader> = Box::new(DiffuseShader { color: color });
    let specular_shader = SpecularShader { alpha: 10.0 };
    let ambient_shader: Box<Shader> = Box::new(MonochromeShader {
        color: Vector3::new(0.1, 0.1, 0.1),
    });
    return 0.5 * diffuse_shader + specular_shader + 0.5 * ambient_shader;
}

pub fn get_bw_chess<'a, 'b>() -> ChessShader<'a, 'b> {
    let black_shader = MonochromeShader {
        color: Vector3::new(0.0, 0.0, 0.0),
    };
    let white_shader = MonochromeShader {
        color: Vector3::new(1.0, 1.0, 1.0),
    };
    ChessShader {
        shader1: & black_shader,
        shader2: & white_shader,
        size: 1.0,
    }
}

pub mod additive_shader;
pub mod ambient_shader;
pub mod chess_shader;
pub mod diffuse_shader;
pub mod mirror_shader;
pub mod monochrome_shader;
pub mod multiplicative_shader;
pub mod specular_shader;
