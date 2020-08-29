use crate::{sys::*, errors::*, objects::*, JVMTIFacadeEnv};

impl<'a> JVMTIFacadeEnv<'a> {
    pub fn get_extension_events(&self) -> Result<Vec<JExtensionEventInfo>> {
        self.jvmti_rust().get_extension_events()
    }

    pub fn get_extension_functions(&self) -> Result<Vec<JExtensionFunctionInfo>> {
        self.jvmti_rust().get_extension_functions()
    }

    pub fn set_extension_event_callback(&self, extension_event_index: jint, callback: jvmtiExtensionEvent) -> Result<()> {
        self.jvmti_rust().set_extension_event_callback(extension_event_index, callback)
    }
}
