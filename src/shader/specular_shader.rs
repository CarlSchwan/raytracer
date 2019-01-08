use crate::shader::Shader;
use nalgebra::{Vector3, Vector2};
use image::Rgba;
use crate::world::World;
use crate::helpers::*;

struct AmbientShader {
    reflection: f64,
    light: Rgba<f64>,
}

impl Shader for AmbientShader {
    fn get_appereance_for(&self, intersection_pos: Vector3<f64>, ray_dir: Vector3<f64>, surface_normal: Vector3<f64>, world: &World, surface_pos: Vector2<f64>) -> Rgba<f64> {
        return vector2color(&(self.reflection * color2vector(&self.light)));
    }
}
