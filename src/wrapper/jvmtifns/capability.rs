use crate::{
    objects::*,
    errors::*,
    JVMTIEnv,
};
use crate::sys::jvmtiCapabilities;

impl<'a> JVMTIEnv<'a> {
    pub fn get_capabilities(&self) -> Result<JCapabilities> {
        let mut capabilities: jvmtiCapabilities = jvmtiCapabilities {
            _bitfield_1: jvmtiCapabilities::newEmptyBitfield(),
        };
        let res = jvmti_call_result!(self.jvmti_raw(), GetCapabilities,
            &mut capabilities
        );
        jvmti_error_code_to_result(res)?;
        Ok(JCapabilities::merge(capabilities))
    }

    pub fn get_potential_capabilities(&self) -> Result<JCapabilities> {
        let mut capabilities: jvmtiCapabilities = jvmtiCapabilities {
            _bitfield_1: jvmtiCapabilities::newEmptyBitfield(),
        };
        let res = jvmti_call_result!(self.jvmti_raw(), GetPotentialCapabilities,
            &mut capabilities
        );
        jvmti_error_code_to_result(res)?;
        Ok(JCapabilities::merge(capabilities))
    }

    pub fn add_capabilities(&self, caps: &JCapabilities) -> Result<()> {
        let capabilities_ptr: jvmtiCapabilities = (*caps).into();
        jvmti_call!(self.jvmti_raw(), AddCapabilities,
            &capabilities_ptr
        )
    }

    pub fn relinquish_capabilities(&self, caps: &JCapabilities) -> Result<()> {
        let capabilities_ptr: jvmtiCapabilities = (*caps).into();
        jvmti_call!(self.jvmti_raw(), RelinquishCapabilities,
            &capabilities_ptr
        )
    }
}
