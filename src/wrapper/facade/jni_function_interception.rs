use jni::sys::JNIEnv;
use crate::{errors::*, JVMTIFacadeEnv};

impl<'a> JVMTIFacadeEnv<'a> {
    pub fn set_jni(&self, jni: &JNIEnv) -> Result<()> {
        self.jvmti_rust().set_jni(jni)
    }

    pub fn get_jni(&self) -> Result<jni::JNIEnv<'a>> {
        self.jvmti_rust().get_jni()
    }
}
