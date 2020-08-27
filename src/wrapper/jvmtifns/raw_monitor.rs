use std::ptr;

use crate::{
    errors::*,
    objects::*,
    JVMTIEnv,
};
use jni::strings::JNIString;
use crate::sys::{jrawMonitorID, jlong};

impl<'a> JVMTIEnv<'a> {
    pub fn create_raw_monitor<N>(&self, name: N) -> Result<Option<JRawMonitorID>>
        where
            N: Into<JNIString>, {
        let ffi_name = name.into();
        let mut value_ptr: jrawMonitorID = ptr::null_mut();
        let res = jvmti_call_result!(self.jvmti_raw(), CreateRawMonitor,
            ffi_name.as_ptr(),
            &mut value_ptr
        );
        jvmti_error_code_to_result(res)?;

        if value_ptr.is_null() {
            return Ok(None);
        }
        Ok(Some(value_ptr.into()))
    }

    pub fn destroy_raw_monitor(&self, monitor_id: &JRawMonitorID) -> Result<()> {
        jvmti_call!(self.jvmti_raw(), DestroyRawMonitor,
            monitor_id.into()
        )
    }

    pub fn raw_monitor_enter(&self, monitor_id: &JRawMonitorID) -> Result<()> {
        jvmti_call!(self.jvmti_raw(), RawMonitorEnter,
            monitor_id.into()
        )
    }

    pub fn raw_monitor_exit(&self, monitor_id: &JRawMonitorID) -> Result<()> {
        jvmti_call!(self.jvmti_raw(), RawMonitorExit,
            monitor_id.into()
        )
    }

    pub fn raw_monitor_wait(&self, monitor_id: &JRawMonitorID, millis: jlong) -> Result<()> {
        jvmti_call!(self.jvmti_raw(), RawMonitorWait,
            monitor_id.into(),
            millis
        )
    }

    pub fn raw_monitor_notify(&self, monitor_id: &JRawMonitorID) -> Result<()> {
        jvmti_call!(self.jvmti_raw(), RawMonitorNotify,
            monitor_id.into()
        )
    }

    pub fn raw_monitor_notify_all(&self, monitor_id: &JRawMonitorID) -> Result<()> {
        jvmti_call!(self.jvmti_raw(), RawMonitorNotifyAll,
            monitor_id.into()
        )
    }
}
