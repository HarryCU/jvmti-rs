use std::ptr;

use crate::{errors::*, JVMTIEnv, JvmtiEvent, JvmtiEventMode, objects::*};
use crate::sys::jvmtiEventCallbacks;

impl<'a> JVMTIEnv<'a> {
    pub fn set_event_callbacks(&self, callbacks: &JEventCallbacks) -> Result<()> {
        let jvmti_callbacks: *const jvmtiEventCallbacks = &(*callbacks).into();
        jvmti_call!(self.jvmti_raw(), SetEventCallbacks,
            jvmti_callbacks,
            JEventCallbacks::size_of_callbacks()
        )
    }

    pub fn set_event_notification_mode(&self, mode: JvmtiEventMode,
                                       event_type: JvmtiEvent,
                                       event_thread: &Option<JThreadID>) -> Result<()> {
        jvmti_call!(self.jvmti_raw(), SetEventNotificationMode,
            mode.into(),
            event_type.into(),
            event_thread.map_or_else(|| ptr::null_mut(), |t| t.into())
        )
    }

    pub fn generate_events(&self, event_type: JvmtiEvent) -> Result<()> {
        jvmti_call!(self.jvmti_raw(), GenerateEvents,
            event_type.into()
        )
    }
}
