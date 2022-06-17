use std::marker::PhantomData;

use cgmath::Vector3;

use crate::{
    driver::RigDriver,
    handedness::Handedness,
    rig::RigUpdateParams,
    transform::Transform,
    util::{look_at, ExpSmoothed, ExpSmoothingParams},
};

#[derive(Debug)]
pub struct LookAt {
    pub smoothness: f32,
    pub target: Vector3<f32>,
    output_offset_scale: f32,
    smoothed_target: ExpSmoothed<Vector3<f32>>
}

impl LookAt {
    pub fn new(target: Vector3<f32>) -> Self {
        Self {
            smoothness: 0.0,
            output_offset_scale: 1.0,
            target,
            smoothed_target: Default::default()
        }
    }

    pub fn tracking_smoothness(mut self, smoothness: f32) -> Self {
        self.smoothness = smoothness;
        self
    }

    pub fn tracking_predictive(mut self, predictive: bool) -> Self {
        self.output_offset_scale = if predictive {-1.0} else {1.0};
        self
    }
}

impl<H: Handedness> RigDriver<H> for LookAt {
    fn update(&mut self, params: RigUpdateParams<H>) -> Transform<H> {
        let target = self.smoothed_target.exp_smooth_towards(
            &self.target,
            ExpSmoothingParams {
                smoothness: self.smoothness,
                output_offset_scale: self.output_offset_scale,
                delta_time_seconds: params.delta_time_seconds
            }
         );
         let rotation = look_at::<H>(target - params.parent.position);
         Transform {
            position: params.parent.position,
            rotation,
            phantom: PhantomData
         }
    }
}