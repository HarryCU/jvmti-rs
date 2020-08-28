use crate::{objects::*, errors::*, JVMTIEnv, Transform, AdapterTransform};
use jni::strings::JNIString;
use jni_sys::jfieldID;

impl<'a> JVMTIEnv<'a> {
    pub fn set_field_access_watch<K, F>(&self, jni: &jni::JNIEnv<'a>, class: K, field: F) -> Result<()>
        where
            K: Transform<'a, JClass<'a>>,
            F: AdapterTransform<jfieldID> {
        let klass: JClass = class.transform(jni)?;

        jvmti_call!(self.jvmti_raw(), SetFieldAccessWatch,
            klass.into_inner(),
            field.transform()
        )
    }

    pub fn clear_field_access_watch<K, F>(&self, jni: &jni::JNIEnv<'a>, class: K, field: F) -> Result<()>
        where
            K: Transform<'a, JClass<'a>>,
            F: AdapterTransform<jfieldID> {
        let klass: JClass = class.transform(jni)?;

        jvmti_call!(self.jvmti_raw(), ClearFieldAccessWatch,
            klass.into_inner(),
            field.transform()
        )
    }

    pub fn set_field_modification_watch<K, F>(&self, jni: &jni::JNIEnv<'a>, class: K, field: F) -> Result<()>
        where
            K: Transform<'a, JClass<'a>>,
            F: AdapterTransform<jfieldID> {
        let klass: JClass = class.transform(jni)?;

        jvmti_call!(self.jvmti_raw(), SetFieldModificationWatch,
            klass.into_inner(),
            field.transform()
        )
    }

    pub fn clear_field_modification_watch<K, F>(&self, jni: &jni::JNIEnv<'a>, class: K, field: F) -> Result<()>
        where
            K: Transform<'a, JClass<'a>>,
            F: AdapterTransform<jfieldID> {
        let klass: JClass = class.transform(jni)?;

        jvmti_call!(self.jvmti_raw(), ClearFieldModificationWatch,
            klass.into_inner(),
            field.transform()
        )
    }

    pub fn set_field_access_watch_s<K, F, V>(&self, jni: &jni::JNIEnv<'a>, class: K, name: F, sig: V) -> Result<()>
        where
            K: Into<JNIString>,
            F: Into<JNIString>,
            V: Into<JNIString> {
        let (klass, field) = self.get_field_id(jni, class, name, sig)?;

        jvmti_call!(self.jvmti_raw(), SetFieldAccessWatch,
            klass.into_inner(),
            field.into_inner()
        )
    }

    pub fn clear_field_access_watch_s<K, F, V>(&self, jni: &jni::JNIEnv<'a>, class: K, name: F, sig: V) -> Result<()>
        where
            K: Into<JNIString>,
            F: Into<JNIString>,
            V: Into<JNIString> {
        let (klass, field) = self.get_field_id(jni, class, name, sig)?;

        jvmti_call!(self.jvmti_raw(), ClearFieldAccessWatch,
            klass.into_inner(),
            field.into_inner()
        )
    }

    pub fn set_field_modification_watch_s<K, F, V>(&self, jni: &jni::JNIEnv<'a>, class: K, name: F, sig: V) -> Result<()>
        where
            K: Into<JNIString>,
            F: Into<JNIString>,
            V: Into<JNIString> {
        let (klass, field) = self.get_field_id(jni, class, name, sig)?;

        jvmti_call!(self.jvmti_raw(), SetFieldModificationWatch,
            klass.into_inner(),
            field.into_inner()
        )
    }

    pub fn clear_field_modification_watch_s<K, F, V>(&self, jni: &jni::JNIEnv<'a>, class: K, name: F, sig: V) -> Result<()>
        where
            K: Into<JNIString>,
            F: Into<JNIString>,
            V: Into<JNIString> {
        let (klass, field) = self.get_field_id(jni, class, name, sig)?;

        jvmti_call!(self.jvmti_raw(), ClearFieldModificationWatch,
            klass.into_inner(),
            field.into_inner()
        )
    }
}
