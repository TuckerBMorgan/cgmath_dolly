use cgmath::{Vector3, Quaternion};
use cgmath::{Zero, One};
use std::marker::PhantomData;

use crate::handedness::Handedness;

#[derive(Clone, Copy, Debug)]
pub struct Transform<H: Handedness> {
    pub position: Vector3<f32>,
    pub rotation: Quaternion<f32>,
    pub phantom: PhantomData<H>
}


impl<H: Handedness> Transform<H> {
    pub fn from_position_rotation(position: Vector3<f32>, rotation: Quaternion<f32>) -> Self  {
        Self  {
            position,
            rotation,
            phantom: PhantomData
        }
    }

    pub fn into_position_rotation(self) -> (Vector3<f32>, Quaternion<f32>) {
        (self.position, self.rotation)
    }

    pub fn right(&self) -> Vector3<f32> {
        self.rotation * Vector3::unit_x()
    }

    pub fn up(&self) -> Vector3<f32> {
        self.rotation * Vector3::unit_y()
    }

    pub fn forward(&self) -> Vector3<f32> {
        self.rotation * H::forward()
    }

    pub fn IDENTITY() -> Transform<H> {
        Transform {
            position: Vector3::zero(),
            rotation: Quaternion::one(),
            phantom: PhantomData,
        }
    }
}