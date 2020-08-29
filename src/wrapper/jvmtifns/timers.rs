use crate::{
    errors::*,
    JVMTIEnv,
    objects::*,
    sys::*,
};

impl<'a> JVMTIEnv<'a> {
    pub fn get_current_thread_cpu_time(&self) -> Result<jlong> {
        Ok(jvmti_call_number_result!(self.jvmti_raw(), jlong,
            GetCurrentThreadCpuTime
        ))
    }

    pub fn get_thread_cpu_time(&self, thread: &JThreadID) -> Result<jlong> {
        Ok(jvmti_call_number_result!(self.jvmti_raw(), jlong,
            GetThreadCpuTime,
            thread.into()
        ))
    }

    pub fn get_time(&self) -> Result<jlong> {
        Ok(jvmti_call_number_result!(self.jvmti_raw(), jlong,
            GetTime
        ))
    }

    pub fn get_current_thread_cpu_timer_info(&self) -> Result<JTimerInfo> {
        let res = jvmti_call_number_result!(self.jvmti_raw(), {JTimerInfo::empty_raw()},
            GetCurrentThreadCpuTimerInfo
        );
        Ok(res.into())
    }

    pub fn get_thread_cpu_timer_info(&self) -> Result<JTimerInfo> {
        let res = jvmti_call_number_result!(self.jvmti_raw(), {JTimerInfo::empty_raw()},
            GetThreadCpuTimerInfo
        );
        Ok(res.into())
    }

    pub fn get_timer_info(&self) -> Result<JTimerInfo> {
        let res = jvmti_call_number_result!(self.jvmti_raw(), {JTimerInfo::empty_raw()},
            GetTimerInfo
        );
        Ok(res.into())
    }

    pub fn get_available_processors(&self) -> Result<jint> {
        Ok(jvmti_call_number_result!(self.jvmti_raw(), jint,
            GetAvailableProcessors
        ))
    }
}
