use std::ffi::c_void;

pub struct GeomMeshRef<'a> {
    pointer: *const c_void,
    _marker: std::marker::PhantomData<&'a ()>,
}

impl<'a> GeomMeshRef<'a> {
    pub fn get_point_count(&self) -> i32 {
        let count = unsafe { GeomMesh_GetPointCount(self.pointer) };
        count
    }

    pub fn get_point(&self, index: i32) -> (f32, f32, f32) {
        let mut x = 0.0f32;
        let mut y = 0.0f32;
        let mut z = 0.0f32;
        unsafe { GeomMesh_GetPoint(self.pointer, &mut x, &mut y, &mut z, index) };
        (x, y, z)
    }

    pub(crate) fn new(pointer: *const c_void) -> Self {
        Self {
            pointer,
            _marker: std::marker::PhantomData,
        }
    }
}

extern "C" {
    fn GeomMesh_GetPointCount(instance: *const c_void) -> i32;

    fn GeomMesh_GetPoint(
        instnace: *const c_void,
        out_x: *mut f32,
        out_y: *mut f32,
        out_z: *mut f32,
        index: i32,
    );
}
