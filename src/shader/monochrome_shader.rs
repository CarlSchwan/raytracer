use image::Rgba;
use crate::shader::*;

pub struct MonochromeShader {
    pub color: Rgba<f64>,
}

impl Shader for MonochromeShader {
    fn get_appereance_for(&self, _intersection_pos: Vector3<f64>, _ray_dir: Vector3<f64>, _surface_normal: Vector3<f64>, _world: &World, _surface_pos: Vector2<f64>, _recursion_depth: u64) -> Rgba<f64> {
        return self.color;
    }
}
