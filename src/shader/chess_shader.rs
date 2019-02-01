use crate::shader::Shader;
use crate::world::World;
use nalgebra::{Vector2, Vector3};

pub struct ChessShader<'a,'b> {
    pub shader1: &'a Shader,
    pub shader2: &'b Shader,
    pub size: f64, //size of squares
}

fn my_mod(a: f64, b: f64) -> f64 {
    let mut x = a;
    while x > b {
        x -= b;
    }
    while x < 0.0 {
        x += b;
    }
    x
}

impl<'a,'b> Shader for ChessShader<'a,'b> {
    fn get_appearance_for(
        &self,
        _intersection_pos: Vector3<f64>,
        _ray_dir: Vector3<f64>,
        _surface_normal: Vector3<f64>,
        _world: &World,
        _surface_pos: Vector2<f64>,
        _recursion_depth: u64,
    ) -> Vector3<f64> {
        let modulo = (_surface_pos / self.size).map(|x| if my_mod(x, 2.0) <= 1.0 { 0 } else { 1 });
        let chooser = modulo.dot(&modulo);
        return if chooser == 0 || chooser == 2 {
            self.shader1.get_appearance_for(
                _intersection_pos,
                _ray_dir,
                _surface_normal,
                _world,
                _surface_pos,
                _recursion_depth,
            )
        } else {
            self.shader2.get_appearance_for(
                _intersection_pos,
                _ray_dir,
                _surface_normal,
                _world,
                _surface_pos,
                _recursion_depth,
            )
        };
    }
}
