use crate::{objects::*, errors::*, Transform, AdapterTransform, JVMTIFacadeEnv};
use jni::strings::JNIString;
use jni_sys::jfieldID;

impl<'a> JVMTIFacadeEnv<'a> {
    pub fn set_field_access_watch<K, F>(&self, class: K, field: F) -> Result<()>
        where
            K: Transform<'a, JClass<'a>>,
            F: AdapterTransform<jfieldID> {
        self.jvmti_rust().set_field_access_watch(self.jni_rust(), class, field)
    }

    pub fn clear_field_access_watch<K, F>(&self, class: K, field: F) -> Result<()>
        where
            K: Transform<'a, JClass<'a>>,
            F: AdapterTransform<jfieldID> {
        self.jvmti_rust().clear_field_access_watch(self.jni_rust(), class, field)
    }

    pub fn set_field_modification_watch<K, F>(&self, class: K, field: F) -> Result<()>
        where
            K: Transform<'a, JClass<'a>>,
            F: AdapterTransform<jfieldID> {
        self.jvmti_rust().set_field_modification_watch(self.jni_rust(), class, field)
    }

    pub fn clear_field_modification_watch<K, F>(&self, class: K, field: F) -> Result<()>
        where
            K: Transform<'a, JClass<'a>>,
            F: AdapterTransform<jfieldID> {
        self.jvmti_rust().clear_field_modification_watch(self.jni_rust(), class, field)
    }

    pub fn set_field_access_watch_s<K, F, V>(&self, class: K, name: F, sig: V) -> Result<()>
        where
            K: Into<JNIString>,
            F: Into<JNIString>,
            V: Into<JNIString> {
        self.jvmti_rust().set_field_access_watch_s(self.jni_rust(), class, name, sig)
    }

    pub fn clear_field_access_watch_s<K, F, V>(&self, class: K, name: F, sig: V) -> Result<()>
        where
            K: Into<JNIString>,
            F: Into<JNIString>,
            V: Into<JNIString> {
        self.jvmti_rust().clear_field_access_watch_s(self.jni_rust(), class, name, sig)
    }

    pub fn set_field_modification_watch_s<K, F, V>(&self, class: K, name: F, sig: V) -> Result<()>
        where
            K: Into<JNIString>,
            F: Into<JNIString>,
            V: Into<JNIString> {
        self.jvmti_rust().set_field_modification_watch_s(self.jni_rust(), class, name, sig)
    }

    pub fn clear_field_modification_watch_s<K, F, V>(&self, class: K, name: F, sig: V) -> Result<()>
        where
            K: Into<JNIString>,
            F: Into<JNIString>,
            V: Into<JNIString> {
        self.jvmti_rust().clear_field_modification_watch_s(self.jni_rust(), class, name, sig)
    }
}