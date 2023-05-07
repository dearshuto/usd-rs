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

    pub fn get_index_count(&self) -> i32 {
        let count = unsafe { GeomMesh_GetIndexCount(self.pointer) };
        count
    }

    pub fn get_index(&self, index: i32) -> i32 {
        let mut result = 0;
        unsafe { GeomMesh_GetIndex(self.pointer, &mut result, index) };
        result
    }

    pub fn get_normal_count(&self) -> i32 {
        unsafe { GeomMesh_GetNormalCount(self.pointer) }
    }

    pub fn get_normal(&self, index: i32) -> (f32, f32, f32) {
        let mut x = 0.0f32;
        let mut y = 0.0f32;
        let mut z = 0.0f32;
        unsafe { GeomMesh_GetNormal(self.pointer, &mut x, &mut y, &mut z, index) };
        (x, y, z)
    }

    pub fn get_face_vertex_count(&self) -> &[i32] {
        let mut count = 0;
        let mut head: *mut i32 = std::ptr::null_mut();
        unsafe {
            GeomMesh_GetFaceVertexCounts(self.pointer, &mut head, &mut count);
        }

        let slice = unsafe { std::slice::from_raw_parts(head, count as usize) };
        slice
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

    fn GeomMesh_GetIndexCount(instance: *const c_void) -> i32;

    fn GeomMesh_GetIndex(instnace: *const c_void, out_x: *mut i32, index: i32);

    fn GeomMesh_GetNormalCount(instance: *const c_void) -> i32;

    fn GeomMesh_GetNormal(
        instnace: *const c_void,
        out_x: *mut f32,
        out_y: *mut f32,
        out_z: *mut f32,
        index: i32,
    );

    fn GeomMesh_GetFaceVertexCounts(
        instnace: *const c_void,
        out_head: *mut *mut i32,
        out_count: *mut i32,
    );
}
