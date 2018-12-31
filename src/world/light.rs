use image::{Rgba, Pixel};
use na::Vector3;

pub struct Light {
    pub color : Rgba<u8>,
    pub position : Vector3<f64>,
    pub intensity : f64
}

impl Light {
    pub fn new(x: f64,y: f64,z: f64) -> Self {
        let color = Rgba::from_channels(255,255,255,255);
        let position = Vector3::new(x,y,z);
        let intensity = 1.0;
        Light {
            color,
            position,
            intensity,
        }
    }
}
