use crate::{errors::*, objects::*, Transform, JVMTIFacadeEnv};
use jni::strings::JNIString;

impl<'a> JVMTIFacadeEnv<'a> {
    pub fn get_method_id<K, M, V>(&self, class: K, name: M, sig: V) -> Result<JMethodID>
        where
            K: Transform<'a, JClass<'a>>,
            M: Into<JNIString>,
            V: Into<JNIString> {
        self.jvmti_rust().get_method_id(self.jni_rust(), class, name, sig)
    }

    pub fn get_static_method_id<K, M, V>(&self, class: K, name: M, sig: V) -> Result<JStaticMethodID>
        where
            K: Transform<'a, JClass<'a>>,
            M: Into<JNIString>,
            V: Into<JNIString> {
        self.jvmti_rust().get_static_method_id(self.jni_rust(), class, name, sig)
    }
}
