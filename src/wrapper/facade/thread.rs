use std::ffi::c_void;

use crate::{sys::*, objects::*, errors::*, JThreadInfo, JvmtiThreadState, JvmtiError, JvmtiThreadPriority, JVMTIFacadeEnv};

impl<'a> JVMTIFacadeEnv<'a> {
    pub fn get_thread_state(&self, thread: &JThreadID) -> Result<JvmtiThreadState> {
        self.jvmti_rust().get_thread_state(thread)
    }

    pub fn get_current_thread(&self) -> Result<Option<JThreadID>> {
        self.jvmti_rust().get_current_thread()
    }

    pub fn get_all_threads(&self) -> Result<Vec<JThreadID>> {
        self.jvmti_rust().get_all_threads()
    }

    pub fn suspend_thread(&self, thread: &JThreadID) -> Result<()> {
        self.jvmti_rust().suspend_thread(thread)
    }

    pub fn resume_thread(&self, thread: &JThreadID) -> Result<()> {
        self.jvmti_rust().resume_thread(thread)
    }

    pub fn stop_thread(&self, thread: &JThreadID, exception: &JThrowable) -> Result<()> {
        self.jvmti_rust().stop_thread(thread, exception)
    }

    pub fn interrupt_thread(&self, thread: &JThreadID) -> Result<()> {
        self.jvmti_rust().interrupt_thread(thread)
    }

    pub fn suspend_thread_list(&self, threads: &Vec<JThreadID>) -> Result<JvmtiError> {
        self.jvmti_rust().suspend_thread_list(threads)
    }

    pub fn resume_thread_list(&self, threads: &Vec<JThreadID>) -> Result<JvmtiError> {
        self.jvmti_rust().resume_thread_list(threads)
    }

    pub fn get_thread_info(&self, thread: &JThreadID) -> Result<JThreadInfo> {
        self.jvmti_rust().get_thread_info(thread)
    }

    pub fn get_owned_monitor_info(&self, thread: &JThreadID) -> Result<Vec<JObject>> {
        self.jvmti_rust().get_owned_monitor_info(thread)
    }

    pub fn get_owned_monitor_stack_depth_info(&self, thread: &JThreadID) -> Result<Vec<JMonitorStackDepthInfo>> {
        self.jvmti_rust().get_owned_monitor_stack_depth_info(thread)
    }

    pub fn get_current_contended_monitor(&self, thread: &JThreadID) -> Result<Option<JObject>> {
        self.jvmti_rust().get_current_contended_monitor(thread)
    }

    pub fn run_agent_thread(&self, thread: &JThreadID,
                            proc: jvmtiStartFunction,
                            arg: *const c_void,
                            priority: JvmtiThreadPriority,
    ) -> Result<()> {
        self.jvmti_rust().run_agent_thread(thread, proc, arg, priority)
    }

    pub fn get_thread_local_storage(&self, thread: &JThreadID) -> Result<JLocalStorage> {
        self.jvmti_rust().get_thread_local_storage(thread)
    }

    pub fn set_thread_local_storage(&self, thread: &JThreadID, data: &JLocalStorage) -> Result<()> {
        self.jvmti_rust().set_thread_local_storage(thread, data)
    }
}
