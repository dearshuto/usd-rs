use std::ffi::{c_uchar, c_void};

pub struct StreamReader<'a> {
    instance: *mut c_void,

    #[allow(dead_code)]
    data: &'a [u8],
}

impl<'a> StreamReader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        let data_ptr = data.as_ptr();
        let data_size = data.len() as u64;
        let instance = unsafe {
            StreamReader_New(data_ptr, data_size, false /*is_swap_endian*/)
        };

        Self { instance, data }
    }

    pub(crate) fn pointer_mut(&mut self) -> *mut c_void {
        self.instance
    }
}

impl<'a> Drop for StreamReader<'a> {
    fn drop(&mut self) {
        unsafe { StreamReader_Delete(self.instance) }
    }
}

extern "C" {
    fn StreamReader_New(binary: *const c_uchar, length: u64, is_swap_endian: bool) -> *mut c_void;

    fn StreamReader_Delete(instance: *mut c_void);
}
