use std::ptr;

use crate::JVMTIEnv;
use crate::sys::jint;

pub trait Builder<T> {
    fn build(&self) -> Vec<T>;
}

pub trait WithJvmtiEnvBuilder<'a, T> {
    fn build(&self, _: &'a JVMTIEnv<'a>) -> Vec<T>;
}

pub trait AutoDeallocateBuilder<'a, T> {
    fn build(&self, _: &'a JVMTIEnv<'a>) -> Vec<T>;
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

pub struct MutAutoDeallocateObjectArrayBuilder<T> {
    pub count: jint,
    pub items: *mut T,
}

impl<T> MutAutoDeallocateObjectArrayBuilder<T> {
    pub fn new() -> MutAutoDeallocateObjectArrayBuilder<T> {
        MutAutoDeallocateObjectArrayBuilder {
            count: 0 as jint,
            items: ptr::null_mut(),
        }
    }

    pub fn with_size(size: jint) -> MutAutoDeallocateObjectArrayBuilder<T> {
        MutAutoDeallocateObjectArrayBuilder {
            count: size,
            items: ptr::null_mut(),
        }
    }

    pub fn create(count: jint, items_ptr: *mut T) -> MutAutoDeallocateObjectArrayBuilder<T> {
        MutAutoDeallocateObjectArrayBuilder {
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