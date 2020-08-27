use crate::wrapper::{
    errors::*,
    JVMTIEnv,
};
use crate::sys::{JMethodID, jlocation};

impl<'a> JVMTIEnv<'a> {
    pub fn set_breakpoint(&self, method: &JMethodID, location: jlocation) -> Result<()> {
        jvmti_call!(self.jvmti_raw(), SetBreakpoint,
            method.into_inner(),
            location
        )
    }

    pub fn clear_breakpoint(&self, method: &JMethodID, location: jlocation) -> Result<()> {
        jvmti_call!(self.jvmti_raw(), ClearBreakpoint,
            method.into_inner(),
            location
        )
    }
}
