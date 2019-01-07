use crate::intersection::Intersection;
use crate::ray::Ray;
use crate::world::{light::Light, obj::Obj, plane::Plane, sphere::Sphere, triangle::Triangle};
use image::{DynamicImage, GenericImage, ImageBuffer, Rgba};
use na::{normalize, Unit, Vector3};
use std::f64;
use std::path::Path;
use std::rc::Rc;

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
        // algorythm for direction taken from https://www.scratchapixel.com/code.php?id=3&origin=/lessons/3d-basic-rendering/introduction-to-ray-tracing
        let mut img = DynamicImage::new_rgb8(self.width, self.height);
        let inv_width = 1.0 / self.width as f64;
        let inv_height = 1.0 / self.height as f64;
        let fov = 30.0;
        let aspectratio = self.width as f64 / self.height as f64;
        let angle = (f64::consts::FRAC_PI_2 * fov / 180.0).tan();
        for x in 0..self.width {
            for y in 0..self.height {
                let xx = (2.0 * ((x as f64 + 0.5) * inv_width) - 1.0) * angle * aspectratio;
                let yy = (1.0 * 2.0 * ((y as f64 + 0.5) * inv_height)) * angle;
                let dir = Vector3::new(xx, yy, -1.0);
                let starting_point = Vector3::new(0.0, 0.0, 0.0); //TODO: choose a starting point
                normalize(&dir);
                let ray = Ray {
                    dir: Unit::new_normalize(dir),
                    start: starting_point,
                };
                let rgb = self.color(ray);
                img.put_pixel(x, y, rgb);
            }
        }
        img
    }

    fn color(&self, ray: Ray) -> Rgba<u8> {
        if let Some(intersection) = self.next_intersection(ray) {
            // touch something
            let color = intersection.color;
            color
        } else {
            // background color
            Rgba([0, 0, 0, 0])
        }
    }

    fn next_intersection(&self, ray: Ray) -> Option<Intersection> {
        let mut max_distance = f64::INFINITY;
        let mut interception = None;
        for element in &self.elements {
            if let Some((distance, intercept)) = element.intercept(&ray) {
                if distance < max_distance {
                    max_distance = distance;
                    interception = Some(intercept);
                }
            }
        }
        interception
    }

    fn color_at_intersection(&self, ray: Ray: intersection: Intersection) -> Rgba<u8> {
        let mut color = Rgba([0, 0, 0, 0]);
        for light in self.lights {
            let shade_ray = Ray { dir: Unit::new_normalize(light.pos - intersection.pos), start: intersection.pos};
            let shade_intersection = next_intersection(shade_ray)?;
            if shade_intersection.pos = light.pos {
                color = color + light.color / 3; 
            }
        }
    }
}
