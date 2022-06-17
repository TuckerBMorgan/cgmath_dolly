use std::fmt::Debug;

use cgmath::{Vector3};

pub trait Handedness : Clone + Copy + Debug + 'static {
    const FORWARD_Z_SIGN: f32;
    fn forward() -> Vector3<f32>;
    fn right_from_up_and_forward(up: Vector3<f32>, forward: Vector3<f32>) -> Vector3<f32>;
    fn up_from_right_and_forward(right: Vector3<f32>, forward: Vector3<f32>) -> Vector3<f32>;
}

#[derive(Clone, Copy, Debug)]
pub struct LeftHanded;

impl Handedness for LeftHanded {
    const FORWARD_Z_SIGN: f32 = 1.0;
    fn forward() -> Vector3<f32> {
        return Vector3::new(0.0, 0.0, Self::FORWARD_Z_SIGN);
    }

    fn right_from_up_and_forward(up: Vector3<f32>, forward: Vector3<f32>) -> Vector3<f32> {
        up.cross(forward)
    }

    fn up_from_right_and_forward(right: Vector3<f32>, forward: Vector3<f32>) -> Vector3<f32> {
        forward.cross(right)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct RightHanded;

impl Handedness for RightHanded {
    const FORWARD_Z_SIGN: f32 = -1.0;

    fn forward() -> Vector3<f32> {
        return Vector3::new(0.0, 0.0, Self::FORWARD_Z_SIGN);
    }

    fn right_from_up_and_forward(up: Vector3<f32>, forward: Vector3<f32>) -> Vector3<f32> {
        forward.cross(up)
    }

    fn up_from_right_and_forward(right: Vector3<f32>, forward: Vector3<f32>) -> Vector3<f32> {
        right.cross(forward)
    }
}