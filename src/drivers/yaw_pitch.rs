use std::marker::PhantomData;

use cgmath::{Euler, Quaternion};

use crate::{
    driver::RigDriver, handedness::Handedness, rig::RigUpdateParams, transform::Transform,
};

/// Calculate camera rotation based on yaw and pitch angles.
///
/// The angles follow the [`right-hand rule`] for curve orientation, and assume
/// an `OpenGL`-style coordinate system, meaning that for a camera to rotate right,
/// a negative value of yaw should be provided, and for it to rotate up,
/// a positive value of pitch.
///
/// [`right-hand rule`]: https://en.wikipedia.org/wiki/Right-hand_rule#Curve_orientation_and_normal_vectors
#[derive(Debug)]
pub struct YawPitch {
    /// [0..720)
    ///
    /// Note: Quaternions can encode 720 degrees of rotation, causing a slerp from 350 to 0 degrees
    /// to happen counter-intuitively in the negative direction; the positive direction would go through 720,
    /// thus being farther. By encoding rotation here in the 0..720 range, we reduce the risk of this happening.
    pub yaw_degrees: cgmath::Deg<f32>,

    /// [-90..90]
    pub pitch_degrees: cgmath::Deg<f32>,
}

impl Default for YawPitch {
    fn default() -> Self {
        Self::new()
    }
}

impl YawPitch {
    /// Creates camera looking forward along Z axis (negative or positive depends on system handedness)
    pub fn new() -> Self {
        Self {
            yaw_degrees: cgmath::Deg(0.0),
            pitch_degrees: cgmath::Deg(0.0),
        }
    }

    /// Initialize the yaw and pitch angles from a quaternion.
    /// Any roll rotation will be ignored.
    pub fn rotation_quat(mut self, rotation: Quaternion<f32>) -> Self {
        self.set_rotation_quat(rotation);
        self
    }

    /// Set the yaw angle in degrees.
    pub fn yaw_degrees(mut self, yaw_degrees: f32) -> Self {
        self.yaw_degrees = cgmath::Deg(yaw_degrees);
        self
    }

    /// Set the pitch angle in degrees.
    pub fn pitch_degrees(mut self, pitch_degrees: f32) -> Self {
        self.pitch_degrees = cgmath::Deg(pitch_degrees);
        self
    }

    /// Additively rotate by the specified angles.
    pub fn rotate_yaw_pitch(&mut self, yaw_degrees: f32, pitch_degrees: f32) {
        self.yaw_degrees = cgmath::Deg((self.yaw_degrees.0 + yaw_degrees) % 720_f32);
        self.pitch_degrees = cgmath::Deg((self.pitch_degrees.0 + pitch_degrees).clamp(-90.0, 90.0));
    }

    /// Set the yaw and pitch angles from a quaternion.
    /// Any roll rotation will be ignored.
    pub fn set_rotation_quat(&mut self, rotation: Quaternion<f32>) {
        let euler : Euler::<cgmath::Rad<f32>> = Euler::from(rotation);
        self.yaw_degrees = euler.y.into();
        self.pitch_degrees = euler.x.into();
    }
}

impl<H: Handedness> RigDriver<H> for YawPitch {
    fn update(&mut self, params: RigUpdateParams<H>) -> Transform<H> {
        let euler : Euler::<cgmath::Deg<f32>> = Euler {
            y: self.yaw_degrees,
            x: self.pitch_degrees,
            z: cgmath::Deg(0.0),
        };

        Transform {
            position: params.parent.position,
            rotation: Quaternion::from(
                euler
            ),
            phantom: PhantomData,
        }
    }
}
