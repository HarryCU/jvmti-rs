pub use jni::JavaVM;
use std::ptr;

use crate::errors::*;
use crate::{sys, JVMTIEnv};

pub trait Jvmti {
    fn get_jvmti_env(&self) -> Result<JVMTIEnv>;
}

impl Jvmti for JavaVM {
    fn get_jvmti_env(&self) -> Result<JVMTIEnv> {
        let mut ptr = ptr::null_mut();
        unsafe {
            let res = java_vm_unchecked!(self.get_java_vm_pointer(), GetEnv, &mut ptr, sys::JVMTI_VERSION);
            jvmti_error_code_to_result(res as sys::jvmtiError)?;

            JVMTIEnv::from_raw(ptr as *mut sys::jvmtiEnv)
        }
    }
}