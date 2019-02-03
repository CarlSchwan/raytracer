pub mod equilinear_camera;
pub mod equirectangular_camera;
use crate::world::World;
use image::DynamicImage;

pub trait Camera {
    fn render(&self, world: &World, progess: bool) -> DynamicImage;
}
