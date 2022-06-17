use crate::{
    transform::Transform,
    driver::{RigDriver, RigDriverTraits},
    handedness::{Handedness, RightHanded},
};

use std::marker::PhantomData;

//#[derive(Debug)]
pub struct CameraRig<H: Handedness = RightHanded> {
    pub drivers: Vec<Box<dyn RigDriverTraits<H>>>,
    pub final_transform: Transform<H>,
    phantom: PhantomData<H>
}

struct RigUpdateToken;

pub struct RigUpdateParams<'a, H: Handedness> {
    ///
    pub parent: &'a Transform<H>,
    ///
    pub delta_time_seconds: f32,

    phantom: PhantomData<H>,
    _token: RigUpdateToken,
}


impl<H: Handedness> CameraRig<H> {
    
    pub fn driver_mut<T: RigDriver<H>>(&mut self) -> &mut T {
        self.try_driver_mut::<T>().unwrap_or_else(||{
            panic!("No {} driver found in the CameraRig", std::any::type_name::<T>());
        })
    }    

    pub fn try_driver_mut<T: RigDriver<H>>(&mut self) -> Option<&mut T> {
        self.drivers.iter_mut().find_map(|driver| driver.as_mut().as_any_mut().downcast_mut::<T>())
    }

    pub fn update(&mut self, delta_time_seconds: f32) -> Transform<H> {
        let mut parent_transform = Transform::IDENTITY();
        
        for driver in self.drivers.iter_mut() {
            let transform = driver.update(
                RigUpdateParams {
                    parent: &parent_transform,
                    delta_time_seconds,
                    phantom: PhantomData,
                    _token: RigUpdateToken
                }
            );
            parent_transform = transform;
        }
        self.final_transform = parent_transform;
        self.final_transform
    }

        /// Use this to make a new rig
    pub fn builder() -> CameraRigBuilder<H> {
        CameraRigBuilder {
            drivers: Default::default(),
            phantom: PhantomData,
        }
    }
}

pub struct CameraRigBuilder<H: Handedness> {
    drivers: Vec<Box<dyn RigDriverTraits<H>>>,
    phantom: PhantomData<H>,
}

impl<H: Handedness> CameraRigBuilder<H> {
    pub fn with(mut self, driver: impl RigDriverTraits<H>) -> Self {
        self.drivers.push(Box::new(driver));
        self
    }

    pub fn build(self) -> CameraRig<H> {
        let mut rig = CameraRig {
            drivers: self.drivers,
            final_transform: Transform::IDENTITY(),
            phantom: PhantomData,
        };

        rig.update(0.0);
        rig
    }
}