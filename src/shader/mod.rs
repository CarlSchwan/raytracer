use image::Rgba;
use nalgebra::{Vector3, Vector2};
use crate::world::World;

pub trait Shader {
    fn get_appereance_for(&self, intersection_pos: Vector3<f64>, ray_dir: Vector3<f64>, surface_normal: Vector3<f64>,
                          world: &World, surface_pos: Vector2<f64>) -> Rgba<f64>;
}

pub mod monochrome_shader;
pub mod additive_shader;
pub mod ambient_shader;
pub mod diffuse_shader;
