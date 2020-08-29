use std::ptr;

use crate::{errors::*, builder::*, objects::*, JVMTIEnv};
use crate::sys;
use crate::sys::{jvmtiFrameInfo, jint, jvmtiStackInfo};

impl<'a> JVMTIEnv<'a> {
    pub fn get_stack_trace(&self, thread: &JThreadID, start_depth: jint, max_frame_count: jint) -> Result<(JFrameInfo, jint)> {
        let mut frame_info: jvmtiFrameInfo = jvmtiFrameInfo {
            method: ptr::null_mut(),
            location: 0 as sys::jlocation,
        };
        let mut count: jint = 0 as jint;
        let res = jvmti_call_result!(self.jvmti_raw(), GetStackTrace,
            thread.into(),
            start_depth,
            max_frame_count,
            &mut frame_info,
            &mut count
        );
        jvmti_error_code_to_result(res)?;
        Ok((frame_info.into(), count))
    }

    pub fn get_all_stack_traces(&self, max_frame_count: jint) -> Result<(Vec<JStackInfo>, jint)> {
        let mut builder: MutObjectArrayBuilder<jvmtiStackInfo> = MutObjectArrayBuilder::with_size(max_frame_count);
        let mut thread_count: jint = 0 as jint;
        let res = jvmti_call_result!(self.jvmti_raw(), GetAllStackTraces,
            max_frame_count,
            &mut builder.items,
            &mut thread_count
        );
        jvmti_error_code_to_result(res)?;
        Ok((builder.build(), thread_count))
    }

    pub fn get_thread_list_stack_traces(&self, threads: &Vec<JThreadID>, max_frame_count: jint) -> Result<Vec<JStackInfo>> {
        if threads.is_empty() {
            return Ok(vec![]);
        }
        let mut builder: MutObjectArrayBuilder<jvmtiStackInfo> = MutObjectArrayBuilder::with_size(max_frame_count);
        let jthreads: Vec<sys::jthread> = threads.iter().map(|&e| e.into()).collect();
        let res = jvmti_call_result!( self.jvmti_raw(), GetThreadListStackTraces,
            jthreads.len() as jint,
            jthreads.as_ptr(),
            max_frame_count,
            &mut builder.items
        );
        jvmti_error_code_to_result(res)?;
        Ok(builder.build())
    }

    pub fn get_frame_count(&self, thread: &JThreadID) -> Result<jint> {
        Ok(jvmti_call_number_result!(self.jvmti_raw(), jint,
            GetFrameCount,
            thread.into()
        ))
    }

    pub fn get_frame_location(&self, thread: &JThreadID, depth: jint) -> Result<Option<JFrameInfo>> {
        let mut info: jvmtiFrameInfo = jvmtiFrameInfo {
            method: ptr::null_mut(),
            location: 0,
        };

        let res = jvmti_call_result!(self.jvmti_raw(), GetFrameLocation,
            thread.into(),
            depth,
            &mut info.method,
            &mut info.location
        );

        jvmti_error_code_to_result(res)?;

        if info.method.is_null() || info.location == 0 {
            return Ok(None);
        }
        Ok(Some(info.into()))
    }

    pub fn pop_frame(&self, thread: &JThreadID) -> Result<()> {
        jvmti_call!(self.jvmti_raw(), PopFrame,
            thread.into()
        )
    }

    pub fn notify_frame_pop(&self, thread: &JThreadID, depth: jint) -> Result<()> {
        jvmti_call!(self.jvmti_raw(), NotifyFramePop,
            thread.into(),
            depth
        )
    }
}
