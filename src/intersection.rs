use image::Rgba;
use na::{Vector3, Vector2};
use crate::shader::Shader;
use crate::world::World;

pub struct Intersection {
    pub pos: Vector3<f64>,
    pub normal_at_surface: Vector3<f64>,
    pub pos_on_surface: Vector2<f64>,
    pub shader: &Box<Shader>,
}

impl Intersection {
    pub fn get_color(&self, ray_dir: Vector3<f64>, world: &World) -> Rgba<f64> {
        self.shader.get_appereance_for(self.pos, ray_dir, self.normal_at_surface, world, self.pos_on_surface)
    }
}

