use image::Rgba;
use na::Vector3;

pub struct Intersection {
    pub pos: Vector3<f64>,
    pub normal_at_surface: Vector3<f64>,
    pub color: Rgba<f64>,
    pub opacity: f64,
    pub reflection: f64,
}
