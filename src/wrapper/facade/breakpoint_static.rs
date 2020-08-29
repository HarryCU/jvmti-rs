use crate::{objects::*, errors::*, Transform, JVMTIFacadeEnv};
use crate::sys::jlocation;
use jni::strings::JNIString;

impl<'a> JVMTIFacadeEnv<'a> {
    pub fn set_breakpoint_s<K, M, V>(&self, class: K, name: M, sig: V, location: jlocation) -> Result<()>
        where
            K: Transform<'a, JClass<'a>>,
            M: Into<JNIString>,
            V: Into<JNIString> {
        self.jvmti_rust().set_breakpoint_s(self.jni_rust(), class, name, sig, location)
    }

    pub fn clear_breakpoint_s<K, M, V>(&self, class: K, name: M, sig: V, location: jlocation) -> Result<()>
        where
            K: Transform<'a, JClass<'a>>,
            M: Into<JNIString>,
            V: Into<JNIString> {
        self.jvmti_rust().clear_breakpoint_s(self.jni_rust(), class, name, sig, location)
    }
}
