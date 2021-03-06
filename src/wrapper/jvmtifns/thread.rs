use std::{
    ffi::c_void,
    ptr,
};
use jni::sys::jobject;
use crate::{builder::*, errors::*, JThreadInfo, JVMTIEnv, JvmtiError, JvmtiThreadPriority, JvmtiThreadState, objects::*, sys::*};

impl<'a> JVMTIEnv<'a> {
    pub fn get_thread_state(&self, thread: &JThreadID) -> Result<JvmtiThreadState> {
        let res = jvmti_call_number_result!(self.jvmti_raw(), jint,
            GetThreadState,
            thread.into()
        );
        Ok((res as jvmtiThreadState).into())
    }

    pub fn get_current_thread(&self) -> Result<Option<JThreadID>> {
        let mut thread_ptr: jthread = ptr::null_mut();
        let res = jvmti_call_result!(self.jvmti_raw(), GetCurrentThread,
            &mut thread_ptr
        );

        jvmti_error_code_to_result(res)?;

        if thread_ptr.is_null() {
            return Ok(None);
        }
        Ok(Some(thread_ptr.into()))
    }

    pub fn get_all_threads(&self) -> Result<Vec<JThreadID>> {
        let mut builder: MutAutoDeallocateObjectArrayBuilder<jthread> = MutAutoDeallocateObjectArrayBuilder::new();
        let res = jvmti_call_result!(self.jvmti_raw(), GetAllThreads,
            &mut builder.count,
            &mut builder.items
        );
        jvmti_error_code_to_result(res)?;
        Ok(builder.build(self))
    }

    pub fn suspend_thread(&self, thread: &JThreadID) -> Result<()> {
        jvmti_call!(self.jvmti_raw(), SuspendThread,
            thread.into()
        )
    }

    pub fn resume_thread(&self, thread: &JThreadID) -> Result<()> {
        jvmti_call!(self.jvmti_raw(), ResumeThread,
            thread.into()
        )
    }

    pub fn stop_thread(&self, thread: &JThreadID, exception: &JThrowable) -> Result<()> {
        jvmti_call!(self.jvmti_raw(), StopThread,
            thread.into(),
            exception.into_inner()
        )
    }

    pub fn interrupt_thread(&self, thread: &JThreadID) -> Result<()> {
        jvmti_call!(self.jvmti_raw(), InterruptThread,
            thread.into()
        )
    }

    pub fn suspend_thread_list(&self, threads: &Vec<JThreadID>) -> Result<JvmtiError> {
        if threads.is_empty() {
            return Ok(JvmtiError::EmptyArgument);
        }
        let thread_ids: Vec<jthread> = threads.iter().map(|&e| e.into()).collect();
        let res = jvmti_call_number_result!(self.jvmti_raw(), jvmtiError,
            SuspendThreadList,
            thread_ids.len() as jint,
            thread_ids.as_ptr()
        );
        Ok(res.into())
    }

    pub fn resume_thread_list(&self, threads: &Vec<JThreadID>) -> Result<JvmtiError> {
        if threads.is_empty() {
            return Ok(JvmtiError::EmptyArgument);
        }
        let thread_ids: Vec<jthread> = threads.iter().map(|&e| e.into()).collect();
        let res = jvmti_call_number_result!(self.jvmti_raw(), jvmtiError,
            ResumeThreadList,
            thread_ids.len() as jint,
            thread_ids.as_ptr()
        );
        Ok(res.into())
    }

    pub fn get_thread_info(&self, thread: &JThreadID) -> Result<JThreadInfo> {
        let mut into = JThreadInfo::empty_raw();
        let res = jvmti_call_result!(self.jvmti_raw(), GetThreadInfo,
            thread.into(),
            &mut into
        );
        jvmti_error_code_to_result(res)?;
        Ok(JThreadInfo::new(into))
    }

    pub fn get_owned_monitor_info(&self, thread: &JThreadID) -> Result<Vec<JObject>> {
        let mut builder: MutAutoDeallocateObjectArrayBuilder<jobject> = MutAutoDeallocateObjectArrayBuilder::new();
        let res = jvmti_call_result!(self.jvmti_raw(), GetOwnedMonitorInfo,
            thread.into(),
            &mut builder.count,
            &mut builder.items
        );
        jvmti_error_code_to_result(res)?;
        Ok(builder.build(self))
    }

    pub fn get_owned_monitor_stack_depth_info(&self, thread: &JThreadID) -> Result<Vec<JMonitorStackDepthInfo>> {
        let mut builder: MutAutoDeallocateObjectArrayBuilder<jvmtiMonitorStackDepthInfo> = MutAutoDeallocateObjectArrayBuilder::new();
        let res = jvmti_call_result!( self.jvmti_raw(), GetOwnedMonitorStackDepthInfo,
            thread.into(),
            &mut builder.count,
            &mut builder.items
        );
        jvmti_error_code_to_result(res)?;
        Ok(builder.build(self))
    }

    pub fn get_current_contended_monitor(&self, thread: &JThreadID) -> Result<Option<JObject>> {
        let res = jvmti_call_ptr_result!(self.jvmti_raw(), jobject,
            GetCurrentContendedMonitor,
            thread.into()
        );
        if res.is_null() {
            return Ok(None);
        }
        Ok(Some(res.into()))
    }

    pub fn run_agent_thread(&self, thread: &JThreadID,
                            proc: jvmtiStartFunction,
                            arg: *const c_void,
                            priority: JvmtiThreadPriority,
    ) -> Result<()> {
        jvmti_call!(self.jvmti_raw(), RunAgentThread,
            thread.into(),
            proc,
            arg,
            priority.into()
        )
    }

    pub fn get_thread_local_storage(&self, thread: &JThreadID) -> Result<JLocalStorage> {
        let mut data_ptr: *mut c_void = ptr::null_mut() as *mut c_void;
        let res = jvmti_call_result!(self.jvmti_raw(), GetThreadLocalStorage,
            thread.into(),
            &mut data_ptr
        );
        jvmti_error_code_to_result(res)?;
        Ok(JLocalStorage::new(data_ptr))
    }

    pub fn set_thread_local_storage(&self, thread: &JThreadID, data: &JLocalStorage) -> Result<()> {
        jvmti_call!(self.jvmti_raw(), SetThreadLocalStorage,
            thread.into(),
            data.as_ptr()
        )
    }
}
