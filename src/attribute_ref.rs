use std::ffi::c_void;

pub struct AttributeRef<'a> {
    #[allow(dead_code)]
    pointer: *mut c_void,
    _marker: std::marker::PhantomData<&'a ()>,
}

impl<'a> AttributeRef<'a> {
    pub(crate) fn new(pointer: *mut c_void) -> Self {
        Self {
            pointer,
            _marker: std::marker::PhantomData,
        }
    }
}
