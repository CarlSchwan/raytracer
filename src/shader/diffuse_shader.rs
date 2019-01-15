use crate::shader::Shader;
use nalgebra::{Unit, Vector3, Vector2};
use image::Rgba;
use crate::world::World;
use crate::helpers::*;
use crate::ray::Ray;

pub struct DiffuseShader {
        pub reflection : f64,
        pub color: Rgba<f64>,
}

impl Shader for DiffuseShader {
    fn get_appearance_for(&self, intersection_pos: Vector3<f64>, _ray_dir: Vector3<f64>, _surface_normal: Vector3<f64>, world: &World, _surface_pos: Vector2<f64>, _recursion_depth: u64) -> Rgba<f64> {
        let mut i_diffuse = Vector3::new(0.0, 0.0, 0.0);

        for light in &world.lights {
            let shade_ray = Ray { dir: Unit::new_normalize(intersection_pos -light.pos), start: light.pos};

            if let Some(shade_intersection) = world.next_intersection(&shade_ray) {
                if (shade_intersection.pos - intersection_pos).norm() < 0.1 {
                    let l_m = shade_ray.dir.normalize();
                    let n_hat = shade_intersection.normal_at_surface.normalize();
                    i_diffuse += 2.0 * (-l_m.dot(&n_hat) * self.reflection * color2vector(&self.color)).component_mul(&color2vector(&light.color));
                }
            }
        }
        return vector2color(&i_diffuse);
    }
}
