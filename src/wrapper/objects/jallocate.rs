use crate::{objects::*, JVMTIEnv};
use jni_sys::jlong;
use crate::sys::jmemory;

#[derive(Clone)]
pub struct JMemoryAllocate<'a> {
    pub ptr: jmemory,
    pub size: usize,
    env: &'a JVMTIEnv<'a>,
}

impl<'a> JMemoryAllocate<'a> {
    pub fn new<'b: 'a>(ptr: jmemory,
                       size: jlong,
                       env: &'b JVMTIEnv<'a>) -> JMemoryAllocate<'a> {
        JMemoryAllocate {
            ptr,
            size: size as usize,
            env,
        }
    }

    pub fn deallocate(&self) {
        if !self.ptr.is_null() {
            self.env.deallocate(self).unwrap()
        }
    }
}

impl<'a> Drop for JMemoryAllocate<'a> {
    fn drop(&mut self) {
        self.deallocate()
    }
}

impl<'a> JDeallocate<'a> for JMemoryAllocate<'a> {
    fn as_deallocate_ptr(&self) -> jmemory {
        self.ptr
    }
}