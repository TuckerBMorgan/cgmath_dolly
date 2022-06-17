use std::marker::PhantomData;

use cgmath::Vector3;

use crate::{
    driver::RigDriver, handedness::Handedness, rig::RigUpdateParams, transform::Transform
};

#[derive(Debug)]
pub struct Arm {
    pub offset: Vector3<f32>
}

impl Arm {
    pub fn new(offset: Vector3<f32>) -> Arm {
        Arm {
            offset
        }
    }
}

impl<H:Handedness> RigDriver<H> for Arm {
    fn update(&mut self, params: RigUpdateParams<H>) -> Transform<H> {
        Transform {
            rotation: params.parent.rotation,
            position: params.parent.position + params.parent.rotation * self.offset,
            phantom: PhantomData
        }
    }
}