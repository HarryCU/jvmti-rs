use crate::{errors::*, objects::*, Transform, JVMTIEnvFacade};
use jni::strings::JNIString;
use crate::sys::jint;

impl<'a> JVMTIEnvFacade<'a> {
    pub fn get_method_declaring_class_i<K, M, V>(&self, class: K, name: M, sig: V) -> Result<JObject>
        where
            K: Transform<'a, JClass<'a>>,
            M: Into<JNIString>,
            V: Into<JNIString> {
        self.jvmti_rust().get_method_declaring_class_i(self.jni_rust(), class, name, sig)
    }

    pub fn get_method_modifiers_i<K, M, V>(&self, class: K, name: M, sig: V) -> Result<jint>
        where
            K: Transform<'a, JClass<'a>>,
            M: Into<JNIString>,
            V: Into<JNIString> {
        self.jvmti_rust().get_method_modifiers_i(self.jni_rust(), class, name, sig)
    }

    pub fn get_max_locals_i<K, M, V>(&self, class: K, name: M, sig: V) -> Result<jint>
        where
            K: Transform<'a, JClass<'a>>,
            M: Into<JNIString>,
            V: Into<JNIString> {
        self.jvmti_rust().get_max_locals_i(self.jni_rust(), class, name, sig)
    }

    pub fn get_arguments_size_i<K, M, V>(&self, class: K, name: M, sig: V) -> Result<jint>
        where
            K: Transform<'a, JClass<'a>>,
            M: Into<JNIString>,
            V: Into<JNIString> {
        self.jvmti_rust().get_arguments_size_i(self.jni_rust(), class, name, sig)
    }

    pub fn get_line_number_table_i<K, M, V>(&self, class: K, name: M, sig: V) -> Result<Vec<JLineNumberEntry>>
        where
            K: Transform<'a, JClass<'a>>,
            M: Into<JNIString>,
            V: Into<JNIString> {
        self.jvmti_rust().get_line_number_table_i(self.jni_rust(), class, name, sig)
    }

    pub fn get_method_location_i<K, M, V>(&self, class: K, name: M, sig: V) -> Result<JMethodLocation>
        where
            K: Transform<'a, JClass<'a>>,
            M: Into<JNIString>,
            V: Into<JNIString> {
        self.jvmti_rust().get_method_location_i(self.jni_rust(), class, name, sig)
    }

    pub fn get_local_variable_table_i<K, M, V>(&self, class: K, name: M, sig: V) -> Result<Vec<JLocalVariableEntry>>
        where
            K: Transform<'a, JClass<'a>>,
            M: Into<JNIString>,
            V: Into<JNIString> {
        self.jvmti_rust().get_local_variable_table_i(self.jni_rust(), class, name, sig)
    }

    pub fn get_bytecodes_i<K, M, V>(&self, class: K, name: M, sig: V) -> Result<JMemoryAllocate>
        where
            K: Transform<'a, JClass<'a>>,
            M: Into<JNIString>,
            V: Into<JNIString> {
        self.jvmti_rust().get_bytecodes_i(self.jni_rust(), class, name, sig)
    }

    pub fn is_method_native_i<K, M, V>(&self, class: K, name: M, sig: V) -> Result<bool>
        where
            K: Transform<'a, JClass<'a>>,
            M: Into<JNIString>,
            V: Into<JNIString> {
        self.jvmti_rust().is_method_native_i(self.jni_rust(), class, name, sig)
    }

    pub fn is_method_synthetic_i<K, M, V>(&self, class: K, name: M, sig: V) -> Result<bool>
        where
            K: Transform<'a, JClass<'a>>,
            M: Into<JNIString>,
            V: Into<JNIString> {
        self.jvmti_rust().is_method_synthetic_i(self.jni_rust(), class, name, sig)
    }

    pub fn is_method_obsolete_i<K, M, V>(&self, class: K, name: M, sig: V) -> Result<bool>
        where
            K: Transform<'a, JClass<'a>>,
            M: Into<JNIString>,
            V: Into<JNIString> {
        self.jvmti_rust().is_method_obsolete_i(self.jni_rust(), class, name, sig)
    }
}
