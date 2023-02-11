use std::ffi::c_void;

use super::Value;

pub struct TimeSamples<'a> {
    instance: *const c_void,
    _marker: std::marker::PhantomData<&'a ()>,
}

impl<'a> TimeSamples<'a> {
    pub fn is_empty(&self) -> bool {
        unsafe { TimeSamples_Empty(self.instance) }
    }

    pub fn size(&self) -> i32 {
        unsafe { TimeSamples_Size(self.instance) }
    }

    pub fn get_time(&self, index: i32) -> f64 {
        unsafe { TimeSamples_GetTime(self.instance, index) }
    }

    pub fn get_value(&self, index: i32) -> Value {
        let value = Value::new();
        unsafe { TimeSamples_GetValue(value.buffer(), self.instance, index) };

        value
    }

    // instance は親 XformOp のポインタ
    pub(crate) fn new(instance: *const c_void) -> Self {
        Self {
            instance,
            _marker: std::marker::PhantomData,
        }
    }
}

extern "C" {
    fn TimeSamples_Empty(instance: *const c_void) -> bool;
    fn TimeSamples_Size(instance: *const c_void) -> i32;
    fn TimeSamples_GetTime(instnace: *const c_void, index: i32) -> f64;
    fn TimeSamples_GetValue(out: *mut c_void, instnace: *const c_void, index: i32);
}
