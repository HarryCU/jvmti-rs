use thiserror::Error;
use std::os::raw::{c_char, c_uchar};
use crate::wrapper::{JVMTIEnv, JDeallocate};
use std::str;
use std::ffi::CStr;
use std::string::FromUtf8Error;
use jni_sys::jlong;
use std::marker::PhantomData;
use std::borrow::Borrow;
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
}

impl<'a> Drop for JMemoryAllocate<'a> {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            self.env.deallocate(self).unwrap()
        }
    }
}

impl<'a> JDeallocate<'a> for JMemoryAllocate<'a> {
    fn as_ptr(&self) -> jmemory {
        self.ptr
    }
}