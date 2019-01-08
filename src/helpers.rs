use na::Vector3;
use image::{Rgba, Rgb, Pixel};

pub fn color2vector(c : &Rgba<f64>) -> Vector3<f64> {
    let (r,g,b,a) = c.channels4();
    return Vector3::new(r,g,b);
}

pub fn vector2color(v : &Vector3<f64>) -> Rgba<f64> {
    Rgb::from_slice(v.as_slice()).to_rgba()
}
