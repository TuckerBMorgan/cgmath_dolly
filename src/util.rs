use cgmath::{Matrix3, Quaternion, Vector3};
use cgmath::VectorSpace;
use cgmath::InnerSpace;
use crate::handedness::Handedness;

pub(crate) trait Interpolate {
    fn interpolate(self, other: Self, t: f32) -> Self;
}

impl Interpolate for Vector3<f32> {
    fn interpolate(self, other: Vector3<f32>, t: f32) -> Vector3<f32> {
        other.lerp(other, t)
    }
}

impl Interpolate for Quaternion<f32> {
    fn interpolate(self, other: Self, t: f32) -> Self {
        self.normalize().nlerp(other.normalize(), t).normalize()
    }
}

pub(crate) struct ExpSmoothingParams {
    pub smoothness: f32,
    pub output_offset_scale: f32,
    pub delta_time_seconds: f32
}

#[derive(Debug)]
pub (crate) struct ExpSmoothed<T: Interpolate + Copy + std::fmt::Debug>(Option<T>);


impl Default for ExpSmoothed<Vector3<f32>> {
    fn default() -> ExpSmoothed<Vector3<f32>> {
        ExpSmoothed(None)
    }
}

impl Default for ExpSmoothed<Quaternion<f32>> {
    fn default() -> ExpSmoothed<Quaternion<f32>> {
        ExpSmoothed(None)
    }
}

impl<T: Interpolate + Copy + std::fmt::Debug> ExpSmoothed<T> {
    pub(crate) fn exp_smooth_towards(&mut self, other: &T, params: ExpSmoothingParams) -> T {
        // An ad-hoc multiplier to make default smoothness parameters
        // produce good-looking results.
        const SMOOTHNESS_MULT: f32 = 8.0;

        // Calculate the exponential blending based on frame time
        let interp_t = 1.0
            - (-SMOOTHNESS_MULT * params.delta_time_seconds / params.smoothness.max(1e-5)).exp();

        let prev = self.0.unwrap_or(*other);
        let smooth = prev.interpolate(*other, interp_t);

        self.0 = Some(smooth);

        #[allow(clippy::float_cmp)]
        if params.output_offset_scale != 1.0 {
            Interpolate::interpolate(*other, smooth, params.output_offset_scale)
        } else {
            smooth
        }
    }
}

pub fn look_at<H: Handedness>(forward: Vector3<f32>) -> Quaternion<f32> {
    let forward = forward.normalize();
    let right = H::right_from_up_and_forward(Vector3::unit_y(), forward).normalize();
    let up = H::up_from_right_and_forward(right, forward);
    let mat3 = Matrix3::from_cols(right, up, forward * H::FORWARD_Z_SIGN);
    let quat : Quaternion<f32> = mat3.into();
    return quat;
}