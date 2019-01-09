use crate::shader::Shader;
use nalgebra::{Vector3, Vector2};
use image::Rgba;
use crate::world::World;
use crate::helpers::*;

pub struct AdditiveShader {
    pub shader1: Box<Shader>,
    pub shader2: Box<Shader>,
}

impl Shader for AdditiveShader {
    fn get_appereance_for(&self, intersection_pos: Vector3<f64>, ray_dir: Vector3<f64>, surface_normal: Vector3<f64>, world: &World, surface_pos: Vector2<f64>) -> Rgba<f64> {
        let ap1 = self.shader1.get_appereance_for(intersection_pos, ray_dir, surface_normal, world, surface_pos);
        let ap2 = self.shader2.get_appereance_for(intersection_pos, ray_dir, surface_normal, world, surface_pos);
        let sum = color2vector(&ap1) + color2vector(&ap2);
        return vector2color(&sum.map(|x| if x > 1.0 {1.0} else {x}));
    }
}
