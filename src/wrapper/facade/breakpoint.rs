use crate::{errors::*, Transform, JVMTIFacadeEnv};
use crate::sys::{jlocation, jmethodID};

impl<'a> JVMTIFacadeEnv<'a> {
    pub fn set_breakpoint<M>(&self, method: M, location: jlocation) -> Result<()>
        where
            M: Transform<'a, jmethodID> {
        self.jvmti_rust().set_breakpoint(self.jni_rust(), method, location)
    }

    pub fn clear_breakpoint<M>(&self, method: M, location: jlocation) -> Result<()>
        where
            M: Transform<'a, jmethodID> {
        self.jvmti_rust().clear_breakpoint(self.jni_rust(), method, location)
    }
}
