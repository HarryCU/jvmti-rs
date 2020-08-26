use crate::wrapper::{
    errors::*,
    JVMTIEnv,
};
use crate::sys::JNIString;

impl<'a> JVMTIEnv<'a> {
    pub fn add_to_bootstrap_class_loader_search<S>(&self, segment: S) -> Result<()>
        where
            S: Into<JNIString> {
        let seg = segment.into();
        jvmti_call!(self.jvmti_raw(), AddToBootstrapClassLoaderSearch,
            seg.as_ptr()
        )
    }

    pub fn add_to_system_class_loader_search<S>(&self, segment: S) -> Result<()>
        where
            S: Into<JNIString> {
        let seg = segment.into();
        jvmti_call!(self.jvmti_raw(), AddToSystemClassLoaderSearch,
            seg.as_ptr()
        )
    }
}
