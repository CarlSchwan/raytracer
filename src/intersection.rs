use na::Vector3;
use image::Rgba;

pub struct Intersection {
    pub pos: Vector3<f64>,
    pub color: Rgba<u8>,
    pub opacity: f64,
    pub reflection: f64,
}
