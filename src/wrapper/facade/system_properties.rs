use crate::{errors::*, objects::*, JVMTIFacadeEnv};
use jni::strings::JNIString;

impl<'a> JVMTIFacadeEnv<'a> {
    pub fn get_system_properties(&self) -> Result<Vec<JvmtiString>> {
        self.jvmti_rust().get_system_properties()
    }

    pub fn get_system_property<S>(&self, property: S) -> Result<JvmtiString>
        where
            S: Into<JNIString> {
        self.jvmti_rust().get_system_property(property)
    }

    pub fn set_system_property<S>(&self, property: S, value: S) -> Result<()>
        where
            S: Into<JNIString> {
        self.jvmti_rust().set_system_property(property, value)
    }
}
