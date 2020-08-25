use std::ffi::c_void;
use std::marker::PhantomData;

pub struct JLocalStorage<'a> {
    data_ptr: *mut c_void,
    lifetime: PhantomData<&'a ()>,
}

impl<'a> JLocalStorage<'a> {
    pub fn new(data_ptr: *mut c_void) -> JLocalStorage<'a> {
        JLocalStorage {
            data_ptr,
            lifetime: PhantomData,
        }
    }

    pub fn as_ptr(&self) -> *const c_void {
        self.data_ptr as *const c_void
    }
}