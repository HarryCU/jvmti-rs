use std::   ptr;
use jni_sys::JNIEnv;
use crate::{
    errors::*,
    JVMTIEnv,
};

impl<'a> JVMTIEnv<'a> {
    pub fn set_jni(&self, jni: &JNIEnv) -> Result<()> {
        jvmti_call!(self.jvmti_raw(), SetJNIFunctionTable,
          *jni
        )
    }

    pub fn get_jni(&self) -> Result<jni::JNIEnv<'a>> {
        let mut jni_env = ptr::null_mut();
        let res = jvmti_call_result!(self.jvmti_raw(), GetJNIFunctionTable,
            &mut jni_env
        );
        jvmti_error_code_to_result(res)?;
        unsafe {
            let mut jni = jni_env as JNIEnv;
            Ok(jni!(&mut jni as *mut JNIEnv))
        }
    }
}
