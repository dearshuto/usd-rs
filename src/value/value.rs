use std::ffi::c_void;

pub struct Value<'a> {
    instance: *mut c_void,
    _marker: std::marker::PhantomData<&'a ()>,
}

impl<'a> Value<'a> {
    pub fn as_float(&self) -> f32 {
        let value = unsafe { Value_AsFloat(self.instance) };
        value
    }

    pub(crate) fn new() -> Self {
        Self {
            instance: unsafe { Value_New() },
            _marker: std::marker::PhantomData,
        }
    }

    pub(crate) fn buffer(&self) -> *mut c_void {
        self.instance
    }
}

impl<'a> Drop for Value<'a> {
    fn drop(&mut self) {
        unsafe { Value_Delete(self.instance) }
    }
}

extern "C" {
    fn Value_New() -> *mut c_void;
    fn Value_Delete(instance: *mut c_void);
    fn Value_AsFloat(instance: *const c_void) -> f32;
}
