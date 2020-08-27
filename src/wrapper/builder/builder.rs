use std::ptr;
use crate::sys::jint;
use crate::JVMTIEnv;

pub trait Builder<T> {
    fn build<'a>(&self) -> Vec<T>;
}

pub trait WithJvmtiEvnBuilder<T> {
    fn build<'a>(&self, _: &JVMTIEnv<'a>) -> Vec<T>;
}

pub struct MutObjectArrayBuilder<T> {
    pub count: jint,
    pub items: *mut T,
}

impl<T> MutObjectArrayBuilder<T> {
    pub fn new() -> MutObjectArrayBuilder<T> {
        MutObjectArrayBuilder {
            count: 0 as jint,
            items: ptr::null_mut(),
        }
    }

    pub fn with_size(size: jint) -> MutObjectArrayBuilder<T> {
        MutObjectArrayBuilder {
            count: size,
            items: ptr::null_mut(),
        }
    }

    pub fn create(count: jint, items_ptr: *mut T) -> MutObjectArrayBuilder<T> {
        MutObjectArrayBuilder {
            count,
            items: items_ptr,
        }
    }
}

pub struct ObjectArrayBuilder<T> {
    pub count: jint,
    pub items: *const T,
}

impl<T> ObjectArrayBuilder<T> {
    pub fn new(count: jint, items_ptr: *const T) -> ObjectArrayBuilder<T> {
        ObjectArrayBuilder {
            count,
            items: items_ptr,
        }
    }
}