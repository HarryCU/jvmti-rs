use std::{
    os::raw::c_char,
    ptr,
};

use jni::strings::JNIString;

use crate::{errors::*, JVMTIEnv, slice_raw, sys::*};

impl<'a> JVMTIEnv<'a> {
    pub fn get_system_properties(&self) -> Result<Vec<String>> {
        let mut count: jint = 0 as jint;
        let mut properties: *mut *mut c_char = ptr::null_mut();

        let res = jvmti_call_result!(self.jvmti_raw(), GetSystemProperties,
            &mut count,
            &mut properties
        );
        jvmti_error_code_to_result(res)?;

        if count == 0 {
            return Ok(vec![]);
        }

        let items = slice_raw(properties, count);
        let mut strings: Vec<String> = Vec::with_capacity(count as usize);
        for &item in items.iter() {
            strings.push(self.build_string(item)?.into())
        }
        self.deallocate_raw(properties as jmemory)?;
        Ok(strings)
    }

    pub fn get_system_property<S>(&self, property: S) -> Result<String>
        where
            S: Into<JNIString> {
        let ffi_name = property.into();

        let mut value = ptr::null_mut();
        let res = jvmti_call_result!(self.jvmti_raw(), GetSystemProperty,
            ffi_name.as_ptr(),
            &mut value
        );
        jvmti_error_code_to_result(res)?;
        Ok(self.build_string(value)?.into())
    }

    pub fn set_system_property<S>(&self, property: S, value: S) -> Result<()>
        where
            S: Into<JNIString> {
        let ffi_name = property.into();
        let ffi_value = value.into();
        jvmti_call!(self.jvmti_raw(), SetSystemProperty,
            ffi_name.as_ptr(),
            ffi_value.as_ptr()
        );
    }
}
