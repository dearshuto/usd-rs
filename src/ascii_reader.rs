use crate::{LoadState, StageRef, StreamReader};
use std::ffi::c_void;

pub struct AsciiReader<'a> {
    instance: *mut c_void,
    _marker: std::marker::PhantomData<&'a ()>,
}

impl<'a> AsciiReader<'a> {
    pub fn new(stream_reader: &mut StreamReader) -> Self {
        let instance = unsafe { USDAReader_New(stream_reader.pointer_mut()) };
        Self {
            instance,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn read(&mut self, state: LoadState) -> bool {
        let state = state as i32;
        let is_success = unsafe { USDAReader_Read(self.instance, state) };
        is_success
    }

    pub fn reconstruct_stage(&mut self) -> bool {
        let instance = self.instance;
        let is_sucess = unsafe { USDAReader_ReconstructStage(instance) };
        is_sucess
    }

    pub fn try_get_stage(&self) -> Option<StageRef> {
        let mut pointer = 0 as *mut c_void;
        unsafe { USDAReader_GetStage(self.instance, &mut pointer) }

        if pointer as i64 == 0 {
            None
        } else {
            Some(StageRef::new(pointer))
        }
    }
}

impl<'a> Drop for AsciiReader<'a> {
    fn drop(&mut self) {
        unsafe { USDAReader_Delete(self.instance) }
    }
}

extern "C" {
    fn USDAReader_New(stream_reader: *mut c_void) -> *mut c_void;

    fn USDAReader_Delete(instance: *mut c_void);

    fn USDAReader_Read(instnace: *mut c_void, state: i32) -> bool;

    fn USDAReader_ReconstructStage(instance: *mut c_void) -> bool;

    fn USDAReader_GetStage(instance: *mut c_void, pointer: &mut *mut c_void);
}
