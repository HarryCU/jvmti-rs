use crate::{sys::*, errors::*, builder::*, objects::*, JVMTIEnv};

impl<'a> JVMTIEnv<'a> {
    pub fn get_extension_events(&self) -> Result<Vec<JExtensionEventInfo>> {
        let mut builder: MutObjectArrayBuilder<jvmtiExtensionEventInfo> = MutObjectArrayBuilder::new();
        let res = jvmti_call_result!( self.jvmti_raw(), GetExtensionEvents,
            &mut builder.count,
            &mut builder.items
        );
        jvmti_error_code_to_result(res)?;
        Ok(builder.build())
    }

    pub fn get_extension_functions(&self) -> Result<Vec<JExtensionFunctionInfo>> {
        let mut builder: MutObjectArrayBuilder<jvmtiExtensionFunctionInfo> = MutObjectArrayBuilder::new();
        let res = jvmti_call_result!( self.jvmti_raw(), GetExtensionFunctions,
            &mut builder.count,
            &mut builder.items
        );
        jvmti_error_code_to_result(res)?;
        Ok(builder.build())
    }

    pub fn set_extension_event_callback(&self, extension_event_index: jint, callback: jvmtiExtensionEvent) -> Result<()> {
        jvmti_call!(self.jvmti_raw(), SetExtensionEventCallback,
            extension_event_index,
            callback
        )
    }
}
