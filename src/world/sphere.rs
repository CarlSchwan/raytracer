use na::Point3;
use image::Rgb;

struct Sphere {
    pub center: Point3<f64>,
    pub radius: f64,
    pub color: Rgb,
}

impl Interceptable for Sphere {
    unimplement!();
}
