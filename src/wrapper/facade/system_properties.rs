use jni::strings::JNIString;

use crate::{errors::*, JVMTIFacadeEnv};

impl<'a> JVMTIFacadeEnv<'a> {
    pub fn get_system_properties(&self) -> Result<Vec<String>> {
        self.jvmti_rust().get_system_properties()
    }

    pub fn get_system_property<S>(&self, property: S) -> Result<String>
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
