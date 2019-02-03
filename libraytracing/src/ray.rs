use na::Unit;
use na::Vector3;

pub struct Ray {
    pub dir: Unit<Vector3<f64>>,
    pub start: Vector3<f64>,
}
