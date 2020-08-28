use crate::{objects::*, errors::*, JVMTIEnvFacade};
use jni::strings::JNIString;
use crate::sys::jint;

impl<'a> JVMTIEnvFacade<'a> {
    pub fn get_field_declaring_class_i<K, F, V>(&self, class: K, name: F, sig: V) -> Result<JObject>
        where
            K: Into<JNIString>,
            F: Into<JNIString>,
            V: Into<JNIString> {
        self.jvmti_rust().get_field_declaring_class_i(self.jni_rust(), class, name, sig)
    }

    pub fn get_field_modifiers_i<K, F, V>(&self, class: K, name: F, sig: V) -> Result<jint>
        where
            K: Into<JNIString>,
            F: Into<JNIString>,
            V: Into<JNIString> {
        self.jvmti_rust().get_field_modifiers_i(self.jni_rust(), class, name, sig)
    }

    pub fn is_field_synthetic_i<K, F, V>(&self, class: K, name: F, sig: V) -> Result<bool>
        where
            K: Into<JNIString>,
            F: Into<JNIString>,
            V: Into<JNIString> {
        self.jvmti_rust().is_field_synthetic_i(self.jni_rust(), class, name, sig)
    }
}
