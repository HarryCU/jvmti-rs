use crate::{errors::*, objects::*, JvmtiEventMode, JvmtiEvent, JVMTIFacadeEnv};

impl<'a> JVMTIFacadeEnv<'a> {
    pub fn set_event_callbacks(&self, callbacks: &JEventCallbacks) -> Result<()> {
        self.jvmti_rust().set_event_callbacks(callbacks)
    }

    pub fn set_event_notification_mode(&self, mode: JvmtiEventMode,
                                       event_type: JvmtiEvent,
                                       event_thread: &Option<JThreadID>) -> Result<()> {
        self.jvmti_rust().set_event_notification_mode(mode, event_type, event_thread)
    }

    pub fn generate_events(&self, event_type: JvmtiEvent) -> Result<()> {
        self.jvmti_rust().generate_events(event_type)
    }
}
