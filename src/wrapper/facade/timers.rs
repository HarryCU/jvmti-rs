use crate::{sys::*, errors::*, objects::*, JVMTIFacadeEnv};

impl<'a> JVMTIFacadeEnv<'a> {
    pub fn get_current_thread_cpu_time(&self) -> Result<jlong> {
        self.jvmti_rust().get_current_thread_cpu_time()
    }

    pub fn get_thread_cpu_time(&self, thread: &JThreadID) -> Result<jlong> {
        self.jvmti_rust().get_thread_cpu_time(thread)
    }

    pub fn get_time(&self) -> Result<jlong> {
        self.jvmti_rust().get_time()
    }

    pub fn get_current_thread_cpu_timer_info(&self) -> Result<JTimerInfo> {
        self.jvmti_rust().get_current_thread_cpu_timer_info()
    }

    pub fn get_thread_cpu_timer_info(&self) -> Result<JTimerInfo> {
        self.jvmti_rust().get_thread_cpu_timer_info()
    }

    pub fn get_timer_info(&self) -> Result<JTimerInfo> {
        self.jvmti_rust().get_timer_info()
    }

    pub fn get_available_processors(&self) -> Result<jint> {
        self.jvmti_rust().get_available_processors()
    }
}
