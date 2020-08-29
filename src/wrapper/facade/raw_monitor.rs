use crate::{errors::*, objects::*, JVMTIFacadeEnv};
use jni::strings::JNIString;
use crate::sys::jlong;

impl<'a> JVMTIFacadeEnv<'a> {
    pub fn create_raw_monitor<N>(&self, name: N) -> Result<Option<JRawMonitorID>>
        where
            N: Into<JNIString>, {
        self.jvmti_rust().create_raw_monitor(name)
    }

    pub fn destroy_raw_monitor(&self, monitor_id: &JRawMonitorID) -> Result<()> {
        self.jvmti_rust().destroy_raw_monitor(monitor_id)
    }

    pub fn raw_monitor_enter(&self, monitor_id: &JRawMonitorID) -> Result<()> {
        self.jvmti_rust().raw_monitor_enter(monitor_id)
    }

    pub fn raw_monitor_exit(&self, monitor_id: &JRawMonitorID) -> Result<()> {
        self.jvmti_rust().raw_monitor_exit(monitor_id)
    }

    pub fn raw_monitor_wait(&self, monitor_id: &JRawMonitorID, millis: jlong) -> Result<()> {
        self.jvmti_rust().raw_monitor_wait(monitor_id, millis)
    }

    pub fn raw_monitor_notify(&self, monitor_id: &JRawMonitorID) -> Result<()> {
        self.jvmti_rust().raw_monitor_notify(monitor_id)
    }

    pub fn raw_monitor_notify_all(&self, monitor_id: &JRawMonitorID) -> Result<()> {
        self.jvmti_rust().raw_monitor_notify_all(monitor_id)
    }
}
