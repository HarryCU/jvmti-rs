use crate::{errors::*, objects::*, JVMTIFacadeEnv};
use crate::sys::jint;

impl<'a> JVMTIFacadeEnv<'a> {
    pub fn get_stack_trace(&self, thread: &JThreadID, start_depth: jint, max_frame_count: jint) -> Result<(JFrameInfo, jint)> {
        self.jvmti_rust().get_stack_trace(thread, start_depth, max_frame_count)
    }

    pub fn get_all_stack_traces(&self, max_frame_count: jint) -> Result<(Vec<JStackInfo>, jint)> {
        self.jvmti_rust().get_all_stack_traces(max_frame_count)
    }

    pub fn get_thread_list_stack_traces(&self, threads: &Vec<JThreadID>, max_frame_count: jint) -> Result<Vec<JStackInfo>> {
        self.jvmti_rust().get_thread_list_stack_traces(threads, max_frame_count)
    }

    pub fn get_frame_count(&self, thread: &JThreadID) -> Result<jint> {
        self.jvmti_rust().get_frame_count(thread)
    }

    pub fn get_frame_location(&self, thread: &JThreadID, depth: jint) -> Result<Option<JFrameInfo>> {
        self.jvmti_rust().get_frame_location(thread, depth)
    }

    pub fn pop_frame(&self, thread: &JThreadID) -> Result<()> {
        self.jvmti_rust().pop_frame(thread)
    }

    pub fn notify_frame_pop(&self, thread: &JThreadID, depth: jint) -> Result<()> {
        self.jvmti_rust().notify_frame_pop(thread, depth)
    }
}
