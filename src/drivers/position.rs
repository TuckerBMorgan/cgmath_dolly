use std::marker::PhantomData;

use cgmath::Vector3;


use crate::{
    driver::RigDriver, handedness::Handedness, rig::RigUpdateParams, transform::Transform
};

#[derive(Debug)]
pub struct Position {
    pub position: Vector3<f32>,
}

impl Position {
    pub fn new(position: Vector3<f32>) -> Self {
        Self {
            position
        }
    }

    pub fn translate(&mut self, move_vec: Vector3<f32>) {
        self.position += move_vec;
    }
}

impl<H: Handedness> RigDriver<H> for Position {
    fn update(&mut self, params: RigUpdateParams<H>) -> Transform<H> {
        Transform {
            position: self.position,
            rotation: params.parent.rotation,
            phantom: PhantomData
        }
    }
}

impl Default for Position {
    fn default() -> Position {
        Position {
            position: Vector3::new(0.0, 0.0, 0.0)
        }
    }
}