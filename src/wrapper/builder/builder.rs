use crate::sys;
use std::ptr;
use crate::wrapper::JVMTIEnv;

pub trait Builder<T> {
    fn build<'a>(&self) -> Vec<T>;
}

pub trait WithJvmtiEvnBuilder<T> {
    fn build<'a>(&self, _: &JVMTIEnv<'a>) -> Vec<T>;
}

pub struct MutObjectArrayBuilder<T> {
    pub count: sys::jint,
    pub items: *mut T,
}

impl<T> MutObjectArrayBuilder<T> {
    pub fn new() -> MutObjectArrayBuilder<T> {
        MutObjectArrayBuilder {
            count: 0 as sys::jint,
            items: ptr::null_mut(),
        }
    }

    pub fn with_size(size: sys::jint) -> MutObjectArrayBuilder<T> {
        MutObjectArrayBuilder {
            count: size,
            items: ptr::null_mut(),
        }
    }

    pub fn create(count: sys::jint, items_ptr: *mut T) -> MutObjectArrayBuilder<T> {
        MutObjectArrayBuilder {
            count,
            items: items_ptr,
        }
    }
}

pub struct ObjectArrayBuilder<T> {
    pub count: sys::jint,
    pub items: *const T,
}

impl<T> ObjectArrayBuilder<T> {
    pub fn new(count: sys::jint, items_ptr: *const T) -> ObjectArrayBuilder<T> {
        ObjectArrayBuilder {
            count,
            items: items_ptr,
        }
    }
}