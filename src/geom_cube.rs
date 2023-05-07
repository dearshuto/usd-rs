use std::ffi::c_void;

pub struct GeomCubeRef<'a> {
    pointer: *const c_void,
    _marker: std::marker::PhantomData<&'a ()>,
}

impl<'a> GeomCubeRef<'a> {
    pub(crate) fn new(pointer: *const c_void) -> Self {
        Self {
            pointer,
            _marker: std::marker::PhantomData,
        }
    }
}
