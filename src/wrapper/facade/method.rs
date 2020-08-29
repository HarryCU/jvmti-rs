use crate::{sys::*, errors::*, objects::*, JMethodName, AdapterTransform, JVMTIFacadeEnv};
use jni::strings::JNIString;

impl<'a> JVMTIFacadeEnv<'a> {
    pub fn set_native_method_prefix<S>(&self, prefix: S) -> Result<()>
        where
            S: Into<JNIString> {
        self.jvmti_rust().set_native_method_prefix(prefix)
    }

    pub fn set_native_method_prefixes<S>(&self, prefixes: &Vec<S>) -> Result<()>
        where
            S: Into<JNIString> + AsRef<str> {
        self.jvmti_rust().set_native_method_prefixes(prefixes)
    }

    pub fn get_method_name<M>(&self, method: M) -> Result<JMethodName>
        where
            M: AdapterTransform<jmethodID> {
        self.jvmti_rust().get_method_name(method)
    }

    pub fn get_method_declaring_class<M>(&self, method: M) -> Result<JObject>
        where
            M: AdapterTransform<jmethodID> {
        self.jvmti_rust().get_method_declaring_class(method)
    }

    pub fn get_method_modifiers<M>(&self, method: M) -> Result<jint>
        where
            M: AdapterTransform<jmethodID> {
        self.jvmti_rust().get_method_modifiers(method)
    }

    pub fn get_max_locals<M>(&self, method: M) -> Result<jint>
        where
            M: AdapterTransform<jmethodID> {
        self.jvmti_rust().get_max_locals(method)
    }

    pub fn get_arguments_size<M>(&self, method: M) -> Result<jint>
        where
            M: AdapterTransform<jmethodID> {
        self.jvmti_rust().get_arguments_size(method)
    }

    pub fn get_line_number_table<M>(&self, method: M) -> Result<Vec<JLineNumberEntry>>
        where
            M: AdapterTransform<jmethodID> {
        self.jvmti_rust().get_line_number_table(method)
    }

    pub fn get_method_location<M>(&self, method: M) -> Result<JMethodLocation>
        where
            M: AdapterTransform<jmethodID> {
        self.jvmti_rust().get_method_location(method)
    }

    pub fn get_local_variable_table<M>(&self, method: M) -> Result<Vec<JLocalVariableEntry>>
        where
            M: AdapterTransform<jmethodID> {
        self.jvmti_rust().get_local_variable_table(method)
    }

    pub fn get_bytecodes<M>(&self, method: M) -> Result<JMemoryAllocate>
        where
            M: AdapterTransform<jmethodID> {
        self.jvmti_rust().get_bytecodes(method)
    }

    pub fn is_method_native<M>(&self, method: M) -> Result<bool>
        where
            M: AdapterTransform<jmethodID> {
        self.jvmti_rust().is_method_native(method)
    }

    pub fn is_method_synthetic<M>(&self, method: M) -> Result<bool>
        where
            M: AdapterTransform<jmethodID> {
        self.jvmti_rust().is_method_synthetic(method)
    }

    pub fn is_method_obsolete<M>(&self, method: M) -> Result<bool>
        where
            M: AdapterTransform<jmethodID> {
        self.jvmti_rust().is_method_obsolete(method)
    }
}
