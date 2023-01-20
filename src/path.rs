use std::ffi::{c_char, c_void, CString};

pub struct Path {
    instance: *mut c_void,
}

impl Path {
    pub fn new(absolute_primitive_path: &str, property_path: &str) -> Self {
        let ab_path = CString::new(absolute_primitive_path).unwrap();
        let p_path = CString::new(property_path).unwrap();
        let instance = unsafe { Path_New(ab_path.as_ptr(), p_path.as_ptr()) };
        Self { instance }
    }

    pub(crate) fn raw_instance(&self) -> *const c_void {
        self.instance
    }
}

impl Drop for Path {
    fn drop(&mut self) {
        let instance = self.instance;
        unsafe { Path_Delete(instance) }
    }
}

extern "C" {
    fn Path_New(
        absolute_primitive_path: *const c_char,
        property_path: *const c_char,
    ) -> *mut c_void;

    fn Path_Delete(instance: *mut c_void);
}
