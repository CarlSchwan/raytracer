use crate::shader::Shader;
use crate::world::World;
use nalgebra::{Vector2, Vector3};

pub type AmbientShader = Vector3<f64>;

impl Shader for AmbientShader {
    fn get_appearance_for(
        &self,
        _intersection_pos: Vector3<f64>,
        _ray_dir: Vector3<f64>,
        _surface_normal: Vector3<f64>,
        _world: &World,
        _surface_pos: Vector2<f64>,
        _recursion_depth: u64,
    ) -> Vector3<f64> {
        *self
    }
}
