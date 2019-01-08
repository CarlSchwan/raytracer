use image::Rgba;
use crate::shader::*;

pub struct MonochromeShader {
    pub color: Rgba<f64>,
}

impl Shader for MonochromeShader {
    fn get_appereance_for(&self, intersection_pos: Vector3<f64>, ray_dir: Vector3<f64>, surface_normal: Vector3<f64>, world: &World, surface_pos: Vector2<f64>) -> Rgba<f64> {
        return self.color;
    }
}
