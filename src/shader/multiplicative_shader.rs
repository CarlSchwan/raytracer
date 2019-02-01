use crate::shader::Shader;
use crate::world::World;
use nalgebra::{Vector2, Vector3};
use std::ops::Mul;

pub struct MultiplicativeShader<'a> {
    pub alpha: f64,
    pub shader: & 'a Shader,
}

pub struct MultiplicativeShaders<'a, 'b> {
    pub alpha: &'a Shader,
    pub shader: &'b Shader,
}

// scalar * shader
impl<'a> Shader for MultiplicativeShader<'a> {
    fn get_appearance_for(
        &self,
        intersection_pos: Vector3<f64>,
        ray_dir: Vector3<f64>,
        surface_normal: Vector3<f64>,
        world: &World,
        surface_pos: Vector2<f64>,
        recursion_depth: u64,
    ) -> Vector3<f64> {
        self.alpha
            * self.shader.get_appearance_for(
                intersection_pos,
                ray_dir,
                surface_normal,
                world,
                surface_pos,
                recursion_depth,
            )
    }
}
// shader * shader (dynamic dispatch)
impl<'a,'b> Shader for MultiplicativeShaders<'a,'b> {
    fn get_appearance_for(
        &self,
        intersection_pos: Vector3<f64>,
        ray_dir: Vector3<f64>,
        surface_normal: Vector3<f64>,
        world: &World,
        surface_pos: Vector2<f64>,
        recursion_depth: u64,
    ) -> Vector3<f64> {
        let l = self.alpha.get_appearance_for(
            intersection_pos,
            ray_dir,
            surface_normal,
            world,
            surface_pos,
            recursion_depth,
        );
        let r = self.shader.get_appearance_for(
            intersection_pos,
            ray_dir,
            surface_normal,
            world,
            surface_pos,
            recursion_depth,
        );
        l.component_mul(&r)
    }
}

// scalar * shader
impl<'a> Mul<f64> for &'a Shader {
    type Output = MultiplicativeShader<'a>;

    fn mul(self, other: f64) -> MultiplicativeShader<'a> {
        MultiplicativeShader {
            shader: self,
            alpha: other,
        }
    }
}
// shader * scalar
impl<'a> Mul<&'a Shader> for f64 {
    type Output = MultiplicativeShader<'a>;

    fn mul(self, other: &'a Shader) -> MultiplicativeShader<'a> {
        other.mul(self)
    }
}
// shader * shader (dynamic dispatch)
impl<'a, 'b:'a> Mul<&'b Shader> for &'a Shader {
    type Output = MultiplicativeShaders<'a, 'b>;

    fn mul(self, other: &'b Shader) ->  MultiplicativeShaders<'a,'b>{
        MultiplicativeShaders {
            alpha: self,
            shader: other,
        }
    }
}
