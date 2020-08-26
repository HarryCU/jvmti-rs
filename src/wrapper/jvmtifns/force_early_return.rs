use crate::wrapper::{
    errors::*,
    objects::*,
    JVMTIEnv,
};
use crate::sys::{jdouble, jfloat, jint, jlong, JObject};

impl<'a> JVMTIEnv<'a> {
    pub fn force_early_return_object(&self, thread: &JThreadID, value: &JObject) -> Result<()> {
        jvmti_call!(self.jvmti_raw(), ForceEarlyReturnObject,
            thread.into(),
            value.into_inner()
        )
    }

    pub fn force_early_return_int(&self, thread: &JThreadID, value: jint) -> Result<()> {
        jvmti_call!(self.jvmti_raw(), ForceEarlyReturnInt,
            thread.into(),
            value
        )
    }

    pub fn force_early_return_long(&self, thread: &JThreadID, value: jlong) -> Result<()> {
        jvmti_call!(self.jvmti_raw(), ForceEarlyReturnLong,
            thread.into(),
            value
        )
    }

    pub fn force_early_return_float(&self, thread: &JThreadID, value: jfloat) -> Result<()> {
        jvmti_call!(self.jvmti_raw(), ForceEarlyReturnFloat,
            thread.into(),
            value
        )
    }

    pub fn force_early_return_double(&self, thread: &JThreadID, value: jdouble) -> Result<()> {
        jvmti_call!(self.jvmti_raw(), ForceEarlyReturnDouble,
            thread.into(),
            value
        )
    }

    pub fn force_early_return_void(&self, thread: &JThreadID) -> Result<()> {
        jvmti_call!(self.jvmti_raw(), ForceEarlyReturnVoid,
            thread.into()
        )
    }
}
