use std::marker::PhantomData;
use std::os::raw::c_char;

use crate::sys;
use crate::{errors::*, objects::*};

#[derive(Clone)]
#[repr(transparent)]
pub struct JVMTIEnv<'a> {
    internal: *mut sys::jvmtiEnv,
    lifetime: PhantomData<&'a ()>,
}

impl<'a> JVMTIEnv<'a> {
    pub unsafe fn from_raw(ptr: *mut sys::jvmtiEnv) -> Result<Self> {
        non_null!(ptr, "from_raw ptr argument");
        Ok(JVMTIEnv {
            internal: ptr,
            lifetime: PhantomData,
        })
    }

    pub fn jvmti_raw(&self) -> *mut sys::jvmtiEnv {
        self.internal
    }

    pub fn build_string(&self, value: *mut c_char) -> Result<JvmtiString> {
        Ok(JvmtiString::with_pointer(value, self))
    }

    pub fn define_class_definition(&self, klass: JClass<'a>, size: sys::jlong, code_bytes: sys::jmemory) -> Result<JClassDefinition> {
        let definition = JClassDefinition::new(klass, code_bytes, size, self);
        Ok(definition)
    }
}

