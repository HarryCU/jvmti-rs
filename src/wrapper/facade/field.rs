use crate::{sys::*, objects::*, errors::*, JFieldName, Transform, JVMTIEnvFacade};
use jni::strings::JNIString;

impl<'a> JVMTIEnvFacade<'a> {
    pub fn get_field_id<K, F, V>(&self, class: K, name: F, sig: V) -> Result<(JClass, JFieldID)>
        where
            K: Into<JNIString>,
            F: Into<JNIString>,
            V: Into<JNIString> {
        self.jvmti_rust().get_field_id(self.jni_rust(), class, name, sig)
    }

    pub fn get_static_field_id<K, F, V>(&self, class: K, name: F, sig: V) -> Result<(JClass, JStaticFieldID)>
        where
            K: Into<JNIString>,
            F: Into<JNIString>,
            V: Into<JNIString> {
        self.jvmti_rust().get_static_field_id(self.jni_rust(), class, name, sig)
    }

    pub fn get_field_name<K, F>(&self, class: K, field: F) -> Result<JFieldName>
        where
            K: Transform<'a, JClass<'a>>,
            F: Transform<'a, jfieldID> {
        self.jvmti_rust().get_field_name(self.jni_rust(), class, field)
    }

    pub fn get_field_declaring_class<K, F>(&self, class: K, field: F) -> Result<JObject>
        where
            K: Transform<'a, JClass<'a>>,
            F: Transform<'a, jfieldID> {
        self.jvmti_rust().get_field_declaring_class(self.jni_rust(), class, field)
    }

    pub fn get_field_modifiers<K, F>(&self, class: K, field: F) -> Result<jint>
        where
            K: Transform<'a, JClass<'a>>,
            F: Transform<'a, jfieldID> {
        self.jvmti_rust().get_field_modifiers(self.jni_rust(), class, field)
    }

    pub fn is_field_synthetic<K, F>(&self, class: K, field: F) -> Result<bool>
        where
            K: Transform<'a, JClass<'a>>,
            F: Transform<'a, jfieldID> {
        self.jvmti_rust().is_field_synthetic(self.jni_rust(), class, field)
    }
}
