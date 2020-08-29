use std::ptr;

use crate::{
    errors::*,
    JVMTIEnv,
    objects::*,
    sys::*,
};

impl<'a> JVMTIEnv<'a> {
    pub fn get_object_size(&self, obj: &JObject) -> Result<jlong> {
        Ok(jvmti_call_number_result!(self.jvmti_raw(), jlong,
            GetObjectSize,
            obj.into_inner()
        ))
    }

    pub fn get_object_hash_code(&self, obj: &JObject) -> Result<jint> {
        Ok(jvmti_call_number_result!(self.jvmti_raw(), jint,
            GetObjectHashCode,
            obj.into_inner()
        ))
    }

    pub fn get_object_monitor_usage(&self, obj: &JObject) -> Result<JMonitorUsage> {
        let mut usage: jvmtiMonitorUsage = jvmtiMonitorUsage {
            owner: ptr::null_mut(),
            entry_count: 0,
            waiter_count: 0,
            waiters: ptr::null_mut(),
            notify_waiter_count: 0,
            notify_waiters: ptr::null_mut(),
        };
        let res = jvmti_call_result!(self.jvmti_raw(), GetObjectMonitorUsage,
            obj.into_inner(),
            &mut usage
        );
        jvmti_error_code_to_result(res)?;
        Ok(JMonitorUsage::new(usage, self))
    }
}
