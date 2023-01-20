use std::ffi::c_void;

use crate::util::Float4x4;

pub struct XformRef<'a> {
    pointer: *mut c_void,
    _marker: std::marker::PhantomData<&'a ()>,
}

impl<'a> XformRef<'a> {
    pub fn get_transform(&self, matrix_data: &mut Float4x4) {
        unsafe {
            Xform_GetLocalMatrix(
                self.pointer,
                matrix_data.data.as_mut_ptr(),
                matrix_data.data.len() as i32,
            )
        }
    }

    pub(crate) fn new(pointer: *mut c_void) -> Self {
        Self {
            pointer,
            _marker: std::marker::PhantomData,
        }
    }
}

extern "C" {
    fn Xform_GetLocalMatrix(instance: *const c_void, array: *mut f32, count: i32);
}
