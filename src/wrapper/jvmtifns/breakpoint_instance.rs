use crate::{objects::*, errors::*, JVMTIEnv, Transform};
use crate::sys::jlocation;
use jni::strings::JNIString;

impl<'a> JVMTIEnv<'a> {
    pub fn set_breakpoint_i<K, M, V>(&self, jni: &jni::JNIEnv<'a>, class: K, name: M, sig: V, location: jlocation) -> Result<()>
        where
            K: Transform<'a, JClass<'a>>,
            M: Into<JNIString>,
            V: Into<JNIString> {
        let method = self.get_method_id(jni, class, name, sig)?;

        jvmti_call!(self.jvmti_raw(), SetBreakpoint,
            method.into_inner(),
            location
        )
    }

    pub fn clear_breakpoint_i<K, M, V>(&self, jni: &jni::JNIEnv<'a>, class: K, name: M, sig: V, location: jlocation) -> Result<()>
        where
            K: Transform<'a, JClass<'a>>,
            M: Into<JNIString>,
            V: Into<JNIString> {
        let method = self.get_method_id(jni, class, name, sig)?;

        jvmti_call!(self.jvmti_raw(), ClearBreakpoint,
            method.into_inner(),
            location
        )
    }
}
