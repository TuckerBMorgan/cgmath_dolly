use std::marker::PhantomData;

use cgmath::Quaternion;
use cgmath::Zero;

use crate::{
    driver::RigDriver, handedness::Handedness, rig::RigUpdateParams, transform::Transform,
};

/// Directly sets the rotation of the camera
#[derive(Debug)]
pub struct Rotation {
    pub rotation: Quaternion<f32>,
}


impl Rotation {
    pub fn new(rotation: Quaternion<f32>) -> Self {
        Self { rotation }
    }
}

impl<H: Handedness> RigDriver<H> for Rotation {
    fn update(&mut self, params: RigUpdateParams<H>) -> Transform<H> {
        Transform {
            position: params.parent.position,
            rotation: self.rotation,
            phantom: PhantomData,
        }
    }
}

impl Default for Rotation {
    fn default() -> Rotation {
        Rotation {
            rotation: Quaternion::zero()
        }
    }
}
