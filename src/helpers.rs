use na::Vector3;
use image::{Rgba, Rgb, Pixel};

// It's easier to do everything with Vectors, and then convert them to colors
//pub fn color2vector(c : &Rgba<f64>) -> Vector3<f64> {
//    let (r,g,b,_a) = c.channels4();
//    return Vector3::new(r,g,b);
//}

pub fn vector2color<T:na::Scalar + image::Primitive>(v : &Vector3<T>) -> Rgba<T> {
    Rgb::from_slice(v.as_slice()).to_rgba()
}
