use na::Vector3;
use na::Unit;

pub struct Ray {
    pub dir: Unit<Vector3<f64>>,
    pub start: Vector3<f64>,
}
