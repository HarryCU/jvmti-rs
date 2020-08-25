use crate::wrapper::{JMemoryAllocate, JVMTIEnv};
use std::marker::PhantomData;
use std::os::raw::c_uchar;
use crate::sys::{jint, jmemory};
use jni_sys::jlong;
use std::borrow::Borrow;

pub struct JConstantPool<'a> {
    lifetime: PhantomData<&'a ()>,

    pub count: u32,
    pub bytes: JMemoryAllocate<'a>,
}

impl<'a> JConstantPool<'a> {
    pub fn new<'b: 'a>(constant_pool_count: jint,
                       constant_pool_byte_count: jint,
                       constant_pool_bytes: jmemory,
                       env: &'b JVMTIEnv<'a>) -> JConstantPool<'a> {
        let bytes = JMemoryAllocate::new(constant_pool_bytes, constant_pool_byte_count as jlong, env);

        JConstantPool {
            lifetime: PhantomData,
            count: constant_pool_count as u32,
            bytes,
        }
    }
}

impl<'a> Drop for JConstantPool<'a> {
    fn drop(&mut self) {
        std::mem::drop((&mut self.bytes))
    }
}