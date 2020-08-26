use crate::sys;
use std::marker::PhantomData;
use std::os::raw::c_char;

use crate::wrapper::*;
use crate::sys::{JClass, jlong, jmemory};

#[derive(Clone)]
#[repr(transparent)]
pub struct JVMTIEnv<'a> {
    internal: *mut sys::JVMTIEnv,
    lifetime: PhantomData<&'a ()>,
}

impl<'a> JVMTIEnv<'a> {
    pub unsafe fn from_raw(ptr: *mut sys::JVMTIEnv) -> Result<Self> {
        non_null!(ptr, "from_raw ptr argument");
        Ok(JVMTIEnv {
            internal: ptr,
            lifetime: PhantomData,
        })
    }

    pub fn jvmti_raw(&self) -> *mut sys::JVMTIEnv {
        self.internal
    }

    pub fn build_string(&self, value: *mut c_char) -> Result<JString> {
        Ok(JString::with_pointer(value, self))
    }

    pub fn define_class_definition(&self, klass: JClass<'a>, size: jlong, code_bytes: jmemory) -> Result<JClassDefinition> {
        let definition = JClassDefinition::new(klass, code_bytes, size, self);
        Ok(definition)
    }
}

