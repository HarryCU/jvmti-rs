use crate::{errors::*, JVMTIEnv, Transform};
use crate::sys::{jlocation, jmethodID};

impl<'a> JVMTIEnv<'a> {
    pub fn set_breakpoint<M>(&self, method: M, location: jlocation) -> Result<()>
        where
            M: Transform<'a, jmethodID> {
        jvmti_call!(self.jvmti_raw(), SetBreakpoint,
            method.transform(self)?,
            location
        )
    }

    pub fn clear_breakpoint<M>(&self, method: M, location: jlocation) -> Result<()>
        where
            M: Transform<'a, jmethodID> {
        jvmti_call!(self.jvmti_raw(), ClearBreakpoint,
            method.transform(self)?,
            location
        )
    }
}
