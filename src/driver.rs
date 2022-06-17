use crate::{rig::RigUpdateParams, transform::Transform, handedness::Handedness};

pub trait RigDriverTraits<H: Handedness> : RigDriver<H> + Sync + Send + std::any::Any + std::fmt::Debug {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

pub trait RigDriver<H: Handedness>: std::any::Any + std::fmt::Debug {
    fn update(&mut self, params: RigUpdateParams<H>) -> Transform<H>;
}

impl<H: Handedness, T> RigDriverTraits<H> for T
    where 
        T: RigDriver<H> + std::any::Any + Sync + Send + std::fmt::Debug
{
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}