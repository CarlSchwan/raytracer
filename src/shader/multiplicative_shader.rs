use crate::shader::Shader;
use crate::world::World;
use nalgebra::{Vector2, Vector3};
use std::ops::Mul;

pub struct MultiplicativeShader<T> {
    pub alpha: T,
    pub shader: Box<Shader>,
}

// scalar * shader
impl Shader for MultiplicativeShader<f64> {
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
impl Shader for MultiplicativeShader<Box<Shader>> {
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
// shader * shader
impl<T: Shader + 'static> Shader for MultiplicativeShader<Box<T>> {
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
impl Mul<f64> for Box<Shader> {
    type Output = Box<Shader>;

    fn mul(self, other: f64) -> Box<Shader> {
        Box::new(MultiplicativeShader {
            shader: self,
            alpha: other,
        })
    }
}
// shader * scalar
impl Mul<Box<Shader>> for f64 {
    type Output = Box<Shader>;

    fn mul(self, other: Box<Shader>) -> Box<Shader> {
        Box::new(MultiplicativeShader {
            alpha: self,
            shader: other,
        })
    }
}
// shader * shader (dynamic dispatch)
impl Mul<Box<Shader>> for Box<Shader> {
    type Output = Box<Shader>;

    fn mul(self, other: Box<Shader>) -> Box<Shader> {
        Box::new(MultiplicativeShader {
            alpha: self,
            shader: other,
        })
    }
}

// shader * shader
impl<T: Shader + 'static> Mul<T> for Box<Shader> {
    type Output = Box<Shader>;

    fn mul(self, other: T) -> Box<Shader> {
        Box::new(MultiplicativeShader {
            shader: self,
            alpha: Box::new(other),
        })
    }
}
