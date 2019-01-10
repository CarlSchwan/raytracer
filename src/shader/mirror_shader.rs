use crate::shader::Shader;
use nalgebra::{Vector3, Vector2, Unit};
use image::{Rgba, Pixel};
use crate::world::World;
use crate::ray::Ray;

pub struct MirrorShader { 
    pub initial_step: f64,
} 

impl Shader for MirrorShader {
    fn get_appereance_for(&self, intersection_pos: Vector3<f64>, ray_dir: Vector3<f64>, surface_normal: Vector3<f64>, world: &World, _surface_pos: Vector2<f64>, recursion_depth: u64) -> Rgba<f64> {
        if recursion_depth==0 {
            return Rgba::from_channels(0.0, 0.0, 0.0, 1.0);
        }
        let unit_normal = surface_normal.normalize();
        let othogonal_part = unit_normal.dot(&-ray_dir) * unit_normal;
        let mirror_ray_dir = (2.0 * othogonal_part - ray_dir).normalize();
        let mirror_ray = Ray { start: intersection_pos + mirror_ray_dir * self.initial_step, dir: Unit::new_normalize(mirror_ray_dir)};
        return world.color(mirror_ray, recursion_depth - 1);
    }
}
