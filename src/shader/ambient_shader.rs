use crate::shader::Shader;
use nalgebra::{Vector3, Vector2};
use image::Rgba;
use crate::world::World;
use crate::helpers::*;

pub struct AmbientShader {
    pub reflection: f64,
    pub light: Rgba<f64>,
}

impl Shader for AmbientShader {
    fn get_appereance_for(&self, _intersection_pos: Vector3<f64>, _ray_dir: Vector3<f64>, _surface_normal: Vector3<f64>, _world: &World, _surface_pos: Vector2<f64>, _recursion_depth: u64) -> Rgba<f64> {
        return vector2color(&(self.reflection * color2vector(&self.light)));
    }
}
