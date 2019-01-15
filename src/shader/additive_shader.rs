use crate::shader::Shader;
use nalgebra::{Vector3, Vector2};
use crate::world::World;

pub struct AdditiveShader {
    pub shader1: Box<Shader>,
    pub shader2: Box<Shader>,
    pub alpha1: f64,
    pub alpha2: f64,
}

impl Shader for AdditiveShader {
    fn get_appearance_for(&self, intersection_pos: Vector3<f64>, ray_dir: Vector3<f64>, surface_normal: Vector3<f64>, world: &World, surface_pos: Vector2<f64>, recursion_depth: u64) -> Vector3<f64> {
        let ap1 = self.shader1.get_appearance_for(intersection_pos, ray_dir, surface_normal, world, surface_pos, recursion_depth);
        let ap2 = self.shader2.get_appearance_for(intersection_pos, ray_dir, surface_normal, world, surface_pos, recursion_depth);
        self.alpha1 * ap1 + self.alpha2 * ap2
    }
}
