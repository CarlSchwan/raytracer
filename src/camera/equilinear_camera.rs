use na::{Vector3, Unit, Rotation3};
use crate::world::World;
use crate::ray::Ray;
use crate::camera::*;
use image::{DynamicImage, GenericImage};
use std::f64;
use crate::helpers::*;


pub struct EquilinearCamera {
	pub height: u32,
	pub width: u32,
	pub roll: f64,
	pub pitch: f64,
	pub yaw: f64,
	pub pos: Vector3<f64>,
	pub vertical_viewangle: f64,
}

impl EquilinearCamera {
	pub fn point_at(&mut self, object: Vector3<f64>) -> () {
		self.yaw = 0.0;
		let (gamma, phi) = vector2polar(&(object - self.pos));
		self.roll = gamma;
		self.pitch = phi;
	}
}

impl Camera for EquilinearCamera {
	fn render(&self, world: World) -> DynamicImage {
        // algorithm for direction taken from https://www.scratchapixel.com/code.php?id=3&origin=/lessons/3d-basic-rendering/introduction-to-ray-tracing
        let mut img = DynamicImage::new_rgb8(self.width, self.height);
        let inv_width = 1.0 / self.width as f64;
        let inv_height = 1.0 / self.height as f64;
        let aspectratio = self.width as f64 / self.height as f64;
        let vertical_half_canvas_size = (f64::consts::FRAC_PI_2 * self.vertical_viewangle / 180.0).tan();
		let rot_matrix = Rotation3::from_euler_angles(self.roll, self.pitch, self.yaw);
        for x in 0..self.width {
            for y in 0..self.height {
                let xx = (2.0 * ((x as f64 + 0.5) * inv_width) - 1.0) * vertical_half_canvas_size * aspectratio;
                let yy = (2.0 * ((y as f64 + 0.5) * inv_height) -1.) * vertical_half_canvas_size;
                let dir = rot_matrix * Vector3::new(xx, yy, 1.0).normalize();
                let ray = Ray {
                    dir: Unit::new_normalize(dir),
                    start: self.pos,
                };
                let rgb = world.color(ray, 10);

                img.put_pixel(x, self.height - y -1, rgb);
            }
        }
        img
    }
}