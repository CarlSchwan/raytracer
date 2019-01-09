use image::{Pixel, Rgba};
use na::Vector3;

pub struct Light {
    pub color: Rgba<f64>,
    pub pos: Vector3<f64>,
    pub intensity: f64,
}

impl Light {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        let color = Rgba::from_channels(1.0, 0.0, 1.0, 1.0);
        let pos = Vector3::new(x, y, z);
        let intensity = 1.0;
        Light {
            color,
            pos,
            intensity,
        }
    }
}
