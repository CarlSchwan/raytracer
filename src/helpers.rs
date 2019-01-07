use na::Vector3;
use image::{Rgba, Pixel};

pub fn color2vector(c : &Rgba<f64>) -> Vector3<f64> {
    let (r,g,b,a) = c.channels4();
    return Vector3::new(r,g,b);
}

pub fn vector2color(v : &Vector3<f64>) -> Rgba<f64> {
    *Rgba::from_slice(v.as_slice())
}
