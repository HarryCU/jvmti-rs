use crate::{sys::*, errors::*, objects::*, JVMTIFacadeEnv};

impl<'a> JVMTIFacadeEnv<'a> {
    pub fn force_early_return_object(&self, thread: &JThreadID, value: &JObject) -> Result<()> {
        self.jvmti_rust().force_early_return_object(thread, value)
    }

    pub fn force_early_return_int(&self, thread: &JThreadID, value: jint) -> Result<()> {
        self.jvmti_rust().force_early_return_int(thread, value)
    }

    pub fn force_early_return_long(&self, thread: &JThreadID, value: jlong) -> Result<()> {
        self.jvmti_rust().force_early_return_long(thread, value)
    }

    pub fn force_early_return_float(&self, thread: &JThreadID, value: jfloat) -> Result<()> {
        self.jvmti_rust().force_early_return_float(thread, value)
    }

    pub fn force_early_return_double(&self, thread: &JThreadID, value: jdouble) -> Result<()> {
        self.jvmti_rust().force_early_return_double(thread, value)
    }

    pub fn force_early_return_void(&self, thread: &JThreadID) -> Result<()> {
        self.jvmti_rust().force_early_return_void(thread)
    }
}
