use std::ffi::c_void;

use crate::{Path, PrimRef};

pub struct StageRef<'a> {
    instance: *mut c_void,
    _marker: std::marker::PhantomData<&'a ()>,
}

impl<'a> StageRef<'a> {
    pub fn new(instance: *mut c_void) -> Self {
        Self {
            instance,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn find_prim_at_path(&self, path: &Path) -> Option<PrimRef> {
        let mut pointer = 0 as *mut c_void;
        let is_found =
            unsafe { Stage_FindPrimitiveAtPath(self.instance, &mut pointer, path.raw_instance()) };

        if is_found {
            Some(PrimRef::new(pointer))
        } else {
            None
        }
    }
}

extern "C" {
    fn Stage_FindPrimitiveAtPath(
        instance: *const c_void,
        primitive: &mut *mut c_void,
        path: *const c_void,
    ) -> bool;
}
