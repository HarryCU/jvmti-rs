use crate::{errors::*, objects::*, JSignature, JvmtiClassStatus, Transform, JVMTIFacadeEnv};
use jni::strings::JNIString;
use crate::sys::jint;

impl<'a> JVMTIFacadeEnv<'a> {
    pub fn get_class<S>(&self, name: S) -> Result<JClass>
        where
            S: Into<JNIString> {
        self.jvmti_rust().get_class(self.jni_rust(), name)
    }

    pub fn get_class_signature<K>(&self, class: K) -> Result<JSignature>
        where
            K: Transform<'a, JClass<'a>> {
        self.jvmti_rust().get_class_signature(self.jni_rust(), class)
    }

    pub fn get_class_status<K>(&self, class: K) -> Result<JvmtiClassStatus>
        where
            K: Transform<'a, JClass<'a>> {
        self.jvmti_rust().get_class_status(self.jni_rust(), class)
    }

    pub fn get_source_file_name<K>(&self, class: K) -> Result<JvmtiString>
        where
            K: Transform<'a, JClass<'a>> {
        self.jvmti_rust().get_source_file_name(self.jni_rust(), class)
    }

    pub fn get_class_modifiers<K>(&self, class: K) -> Result<jint>
        where
            K: Transform<'a, JClass<'a>> {
        self.jvmti_rust().get_class_modifiers(self.jni_rust(), class)
    }

    pub fn get_class_methods<K>(&self, class: K) -> Result<Vec<JMethodID>>
        where
            K: Transform<'a, JClass<'a>> {
        self.jvmti_rust().get_class_methods(self.jni_rust(), class)
    }

    pub fn get_class_fields<K>(&self, class: K) -> Result<Vec<JFieldID>>
        where
            K: Transform<'a, JClass<'a>> {
        self.jvmti_rust().get_class_fields(self.jni_rust(), class)
    }

    pub fn get_implemented_interfaces<K>(&self, class: K) -> Result<Vec<JClass>>
        where
            K: Transform<'a, JClass<'a>> {
        self.jvmti_rust().get_implemented_interfaces(self.jni_rust(), class)
    }

    pub fn is_interface<K>(&self, class: K) -> Result<bool>
        where
            K: Transform<'a, JClass<'a>> {
        self.jvmti_rust().is_interface(self.jni_rust(), class)
    }

    pub fn is_array_class<K>(&self, class: K) -> Result<bool>
        where
            K: Transform<'a, JClass<'a>> {
        self.jvmti_rust().is_array_class(self.jni_rust(), class)
    }

    pub fn is_modifiable_class<K>(&self, class: K) -> Result<bool>
        where
            K: Transform<'a, JClass<'a>> {
        self.jvmti_rust().is_modifiable_class(self.jni_rust(), class)
    }

    pub fn get_source_debug_extension<K>(&self, class: K) -> Result<JvmtiString>
        where
            K: Transform<'a, JClass<'a>> {
        self.jvmti_rust().get_source_debug_extension(self.jni_rust(), class)
    }

    pub fn get_class_loader<K>(&self, class: K) -> Result<JClassLoader>
        where
            K: Transform<'a, JClass<'a>> {
        self.jvmti_rust().get_class_loader(self.jni_rust(), class)
    }

    pub fn get_constant_pool<K>(&self, class: K) -> Result<JConstantPool>
        where
            K: Transform<'a, JClass<'a>> {
        self.jvmti_rust().get_constant_pool(self.jni_rust(), class)
    }

    pub fn get_class_version_numbers<K>(&self, class: K) -> Result<JClassVersionNumber>
        where
            K: Transform<'a, JClass<'a>> {
        self.jvmti_rust().get_class_version_numbers(self.jni_rust(), class)
    }
}
