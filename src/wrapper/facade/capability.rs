use crate::{objects::*, errors::*, JVMTIFacadeEnv};

impl<'a> JVMTIFacadeEnv<'a> {
    pub fn get_capabilities(&self) -> Result<JCapabilities> {
        self.jvmti_rust().get_capabilities()
    }

    pub fn get_potential_capabilities(&self) -> Result<JCapabilities> {
        self.jvmti_rust().get_potential_capabilities()
    }

    pub fn add_capabilities(&self, caps: &JCapabilities) -> Result<()> {
        self.jvmti_rust().add_capabilities(caps)
    }

    pub fn relinquish_capabilities(&self, caps: &JCapabilities) -> Result<()> {
        self.jvmti_rust().relinquish_capabilities(caps)
    }
}
