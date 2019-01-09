use crate::intersection::Intersection;
use crate::ray::Ray;
use crate::world::light::Light;
use image::{DynamicImage, GenericImage, Rgba};
use na::{normalize, Unit, Vector3};
use std::f64;
use image::Pixel;
use crate::helpers::{color2vector, vector2color};
use crate::shader::*;

pub mod light;
pub mod obj;
pub mod plane;
pub mod sphere;
pub mod triangle;

pub trait Interceptable {
    fn intercept(&self, ray: &Ray) -> Option<(f64, Intersection)>;
}

pub struct World {
    pub width: u32,
    pub height: u32,
    pub elements: Vec<Box<Interceptable>>,
    pub lights: Vec<Light>,
}

impl World {
    pub fn new(
        width: u32,
        height: u32,
        elements: Vec<Box<Interceptable>>,
        lights: Vec<Light>,
    ) -> Self {
        World {
            width,
            height,
            elements,
            lights,
        }
    }
    pub fn render(&self) -> DynamicImage {
        // algorithm for direction taken from https://www.scratchapixel.com/code.php?id=3&origin=/lessons/3d-basic-rendering/introduction-to-ray-tracing
        let mut img = DynamicImage::new_rgb8(self.width, self.height);
        let inv_width = 1.0 / self.width as f64;
        let inv_height = 1.0 / self.height as f64;
        let vertical_view_angle = 40.0;
        let aspectratio = self.width as f64 / self.height as f64;
        let vertical_half_canvas_size = (f64::consts::FRAC_PI_2 * vertical_view_angle / 180.0).tan();
        for x in 0..self.width {
            for y in 0..self.height {
                let xx = (2.0 * ((x as f64 + 0.5) * inv_width) - 1.0) * vertical_half_canvas_size * aspectratio;
                let yy = (2.0 * ((y as f64 + 0.5) * inv_height) -1.) * vertical_half_canvas_size;
                let dir = Vector3::new(xx, -yy, 1.0);
                let starting_point = Vector3::new(0.0, 0.0, 0.0); //TODO: choose a starting point and dir
                normalize(&dir);
                let ray = Ray {
                    dir: Unit::new_normalize(dir),
                    start: starting_point,
                };
                let rgb = self.color(ray, 10);

                let rgb = Rgba::from_channels((rgb.channels4().0 * 255.0).floor() as u8, (rgb.channels4().1 * 255.0).floor() as u8, (rgb.channels4().2 * 255.0).floor() as u8, (rgb.channels4().3 * 255.0).floor() as u8);

                img.put_pixel(x, y, rgb);
            }
        }
        img
    }

    pub fn color(&self, ray: Ray, recursion_depth: u64) -> Rgba<f64> {
        if let Some(intersection) = self.next_intersection(&ray) {
            // touch something
            intersection.get_color(ray.dir.into_inner(), self, recursion_depth)
        } else {
            // background color
            Rgba([0.0, 0.0, 0.0, 1.0])
        }
    }

    pub fn next_intersection(&self, ray: &Ray) -> Option<Intersection> {
        let mut max_distance = f64::INFINITY;
        let mut interception = None;
        for element in &self.elements {
            if let Some((distance, intercept)) = element.intercept(ray) {
                if distance < max_distance {
                    max_distance = distance;
                    interception = Some(intercept);
                }
            }
        }
        interception
    }
}
