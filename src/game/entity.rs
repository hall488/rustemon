use cgmath::Vector3;
use crate::renderer::instance::Instance;

pub trait Entity {
    fn position(&self) -> Vector3<f32>;
    fn instances(&self) -> &[Instance];
}
