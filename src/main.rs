#[macro_use]
extern crate approx; // For the macro relative_eq!
extern crate nalgebra as na;

// imageBuffer
extern crate image;

// parse wavefront_obj
extern crate wavefront_obj;
use na::{Vector3,normalize};

use image::{GenericImage, ImageBuffer, Rgb};

trait Interceptable {
    pub interceptable(&self, ray: Ray) -> bool
}

struct Scene {
    obj: Vec<Interceptable>,

}

struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub color: Rgb,
}

struct Triangle {
    point1: Point3,
    point2: Point3,
    point3: Point3,
}

struct Ray {
    start: Vector3,
    dir: Vector3,
}

fn main() {
    let img = ImageBuffer::new(512, 512);
    let (width, height) = img.dimensions();
    for y in 0..ing.height {
        for x in 0..ing.width {
            // convert coordinate in percent 
            let x_center = x as f64 / width as f64 * 2.0 - 1.0;
            let y_center = (y as f64/ height as f64 * 2.0 - 1.0) / (width as f64 / height as f64);
            let ray = Ray { start: Vector3::new(0.0, -40, -120), dir: normalize(Vector3::new(x_center, y_center, 1.0)) }; 

             
        }
    }
}
