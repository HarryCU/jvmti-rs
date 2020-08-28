use std::ptr;

use crate::{sys::*, objects::*, errors::*, JVMTIEnv, to_bool};
use jni::strings::JNIString;

impl<'a> JVMTIEnv<'a> {
    pub fn get_field_declaring_class_i<K, F, V>(&self, jni: &jni::JNIEnv<'a>, class: K, name: F, sig: V) -> Result<JObject>
        where
            K: Into<JNIString>,
            F: Into<JNIString>,
            V: Into<JNIString> {
        let (klass, field) = self.get_field_id(jni, class, name, sig)?;

        let mut value_ptr: jclass = ptr::null_mut();
        let res = jvmti_call_result!(self.jvmti_raw(), GetFieldDeclaringClass,
            klass.into_inner(),
            field.into_inner(),
            &mut value_ptr
        );
        jvmti_error_code_to_result(res)?;
        Ok(value_ptr.into())
    }

    pub fn get_field_modifiers_i<K, F, V>(&self, jni: &jni::JNIEnv<'a>, class: K, name: F, sig: V) -> Result<jint>
        where
            K: Into<JNIString>,
            F: Into<JNIString>,
            V: Into<JNIString> {
        let (klass, field) = self.get_field_id(jni, class, name, sig)?;

        Ok(jvmti_call_number_result!(self.jvmti_raw(), jint,
            GetFieldModifiers,
            klass.into_inner(),
            field.into_inner()
        ))
    }

    pub fn is_field_synthetic_i<K, F, V>(&self, jni: &jni::JNIEnv<'a>, class: K, name: F, sig: V) -> Result<bool>
        where
            K: Into<JNIString>,
            F: Into<JNIString>,
            V: Into<JNIString> {
        let (klass, field) = self.get_field_id(jni, class, name, sig)?;

        let res = jvmti_call_number_result!(self.jvmti_raw(), jboolean,
            IsFieldSynthetic,
            klass.into_inner(),
            field.into_inner()
        );
        Ok(to_bool(res))
    }
}
