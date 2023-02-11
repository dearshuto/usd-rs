use std::ffi::c_void;

use crate::value::TimeSamples;

pub enum OpType {
    // matrix
    Transform,

    // vector3
    Translate,
    Scale,

    // scalar
    RotateX,
    RotateY,
    RotateZ,

    // vector3
    RotateXYZ,
    RotateXZY,
    RotateYXZ,
    RotateYZX,
    RotateZXY,
    RotateZYX,

    // quaternion
    Orient,

    // Special token
    ResetXformStack, // !resetXformStack!
}

pub struct XformOp<'a> {
    instance: *const c_void,
    _marker: std::marker::PhantomData<&'a ()>,
}

impl<'a> XformOp<'a> {
    pub fn get_type(&self) -> OpType {
        let op_type = unsafe { XformOp_GetType(self.instance) };
        match op_type {
            0 => OpType::Transform,
            1 => OpType::Translate,
            2 => OpType::Scale,
            3 => OpType::RotateX,
            4 => OpType::RotateY,
            5 => OpType::RotateZ,
            6 => OpType::RotateXYZ,
            7 => OpType::RotateXZY,
            8 => OpType::RotateYXZ,
            9 => OpType::RotateYZX,
            10 => OpType::RotateZXY,
            11 => OpType::RotateZYX,
            12 => OpType::Orient,
            13 => OpType::ResetXformStack,
            _ => todo!(),
        }
    }

    pub fn is_time_samples(&self) -> bool {
        let is_time_samples = unsafe { XformOp_IsTimeSamples(self.instance) };
        is_time_samples
    }

    pub fn get_time_samples(&self) -> Option<TimeSamples> {
        if !self.is_time_samples() {
            return None;
        }

        let instance = unsafe { XformOp_GetTimeSamples(self.instance) };
        Some(TimeSamples::new(instance))
    }

    pub(crate) fn new(instance: *const c_void) -> Self {
        Self {
            instance,
            _marker: std::marker::PhantomData,
        }
    }
}

extern "C" {
    fn XformOp_GetType(instance: *const c_void) -> i32;
    fn XformOp_IsTimeSamples(instance: *const c_void) -> bool;
    fn XformOp_GetTimeSamples(instance: *const c_void) -> *const c_void;
}
