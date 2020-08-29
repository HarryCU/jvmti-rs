use crate::{errors::*, JVMTIFacadeEnv};
use jni::strings::JNIString;

impl<'a> JVMTIFacadeEnv<'a> {
    pub fn add_to_bootstrap_class_loader_search<S>(&self, segment: S) -> Result<()>
        where
            S: Into<JNIString> {
        self.jvmti_rust().add_to_bootstrap_class_loader_search(segment)
    }

    pub fn add_to_system_class_loader_search<S>(&self, segment: S) -> Result<()>
        where
            S: Into<JNIString> {
        self.jvmti_rust().add_to_system_class_loader_search(segment)
    }
}
