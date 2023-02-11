use std::ffi::{c_char, c_void, CString};

use crate::{AttributeRef, GeomMeshRef, XformRef};

pub struct PrimRef<'a> {
    pointer: *const c_void,
    _marker: std::marker::PhantomData<&'a ()>,
}

impl<'a> PrimRef<'a> {
    pub fn as_gemo_mesh(&self) -> Option<GeomMeshRef> {
        let pointer = unsafe { Prim_AsGeomMesh(self.pointer) };
        if pointer as i32 == 0 {
            None
        } else {
            Some(GeomMeshRef::new(pointer))
        }
    }

    pub fn as_x_form(&self) -> Option<XformRef> {
        let pointer = unsafe { Prim_AsXForm(self.pointer) };
        if pointer as i32 == 0 {
            None
        } else {
            Some(XformRef::new(pointer))
        }
    }

    pub fn get_attribute(&self, name: &str) -> Option<AttributeRef<'a>> {
        let name = CString::new(name).unwrap();
        let mut pointer = std::ptr::null_mut::<c_void>();
        if !unsafe { Prim_GetAttribute(&mut pointer, self.pointer, name.as_ptr()) } {
            return None;
        }

        return Some(AttributeRef::new(pointer));
    }

    pub fn get_child_count(&self) -> i32 {
        let count = unsafe { Prim_GetChildCount(self.pointer) };
        count
    }

    pub fn try_get_child(&self, index: i32) -> Option<PrimRef> {
        let pointer = unsafe { Prim_GetChild(self.pointer, index) };

        if pointer as i64 == 0 {
            None
        } else {
            Some(PrimRef::new(pointer))
        }
    }

    pub(crate) fn new(pointer: *const c_void) -> Self {
        Self {
            pointer,
            _marker: std::marker::PhantomData,
        }
    }
}

extern "C" {
    fn Prim_AsGeomMesh(instance: *const c_void) -> *mut c_void;
    fn Prim_AsXForm(instance: *const c_void) -> *mut c_void;
    fn Prim_GetChildCount(instance: *const c_void) -> i32;
    fn Prim_GetChild(instance: *const c_void, index: i32) -> *const c_void;
    fn Prim_GetAttribute(
        attribute: &mut *mut c_void,
        instance: *const c_void,
        name: *const c_char,
    ) -> bool;
}
