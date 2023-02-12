use std::ffi::c_void;

use crate::{util::Float4x4, xform_op_ref::XformOp};

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

    pub fn get_op_count(&self) -> i32 {
        let count = unsafe { Xform_GetOpCount(self.pointer) };
        count
    }

    pub fn get_op(&self, index: i32) -> XformOp {
        let pointer = unsafe { Xform_GetOp(self.pointer, index) };
        XformOp::new(pointer)
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
    fn Xform_GetOp(instance: *const c_void, index: i32) -> *const c_void;
    fn Xform_GetOpCount(instance: *const c_void) -> i32;
}
