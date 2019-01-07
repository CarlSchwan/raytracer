use na::Vector3;
use image::{Rgba, Pixel};

pub fn color2vector(c : &Rgba<f64>) -> Vector3<f64> {
    let (r,g,b,a) = c.channels4();
    return Vector3::new(r,g,b);
}
