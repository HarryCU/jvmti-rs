use crate::wrapper::{
    errors::*,
    objects::*,
    JVMTIEnv,
};
use crate::sys::JMethodID;

impl<'a> JVMTIEnv<'a> {
    pub fn set_breakpoint(&self, method: &JMethodID, location: &JLocation) -> Result<()> {
        jvmti_call!(self.jvmti_raw(), SetBreakpoint,
            method.into_inner(),
            location.into()
        )
    }

    pub fn clear_breakpoint(&self, method: &JMethodID, location: &JLocation) -> Result<()> {
        jvmti_call!(self.jvmti_raw(), ClearBreakpoint,
            method.into_inner(),
            location.into()
        )
    }
}
