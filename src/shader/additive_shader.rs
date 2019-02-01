use crate::shader::Shader;
use crate::world::World;
use nalgebra::{Vector2, Vector3};
use std::ops::Add;

pub struct AdditiveShader<'a, 'b> {
    pub shader1: &'a Shader,
    pub shader2: &'b Shader,
}

impl<'a, 'b> Shader for AdditiveShader<'a, 'b> {
    fn get_appearance_for(
        &self,
        intersection_pos: Vector3<f64>,
        ray_dir: Vector3<f64>,
        surface_normal: Vector3<f64>,
        world: &World,
        surface_pos: Vector2<f64>,
        recursion_depth: u64,
    ) -> Vector3<f64> {
        let ap1 = self.shader1.get_appearance_for(
            intersection_pos,
            ray_dir,
            surface_normal,
            world,
            surface_pos,
            recursion_depth,
        );
        let ap2 = self.shader2.get_appearance_for(
            intersection_pos,
            ray_dir,
            surface_normal,
            world,
            surface_pos,
            recursion_depth,
        );
        ap1 + ap2
    }
}
impl<'a,'b> Add<&'b Shader> for &'a Shader {
    type Output = AdditiveShader<'a,'b>;

    fn add(self, other: &'b Shader) -> AdditiveShader<'a,'b> {
        AdditiveShader {
            shader1: self,
            shader2: other,
        }
    }
}

//// Add unboxed Shader to boxed shader (boxing is done here)
//impl<T: Shader + 'static> Add<T> for Box<Shader> {
//    type Output = Box<Shader>;
//
//    fn add(self, other: T) -> Box<Shader> {
//        Box::new(AdditiveShader {
//            shader1: self,
//            shader2: Box::new(other),
//        })
//    }
//}
