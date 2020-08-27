use std::ptr;
use crate::{errors::*, builder::*, objects::*, JVMTIEnv, JThreadGroupInfo};
use crate::sys;
use crate::sys::{jthreadGroup, jvmtiThreadGroupInfo};

impl<'a> JVMTIEnv<'a> {
    pub fn get_top_thread_groups(&self) -> Result<Vec<JThreadGroupID>> {
        let mut thread_group_builder: MutObjectArrayBuilder<jthreadGroup> = MutObjectArrayBuilder::new();
        let res = jvmti_call_result!( self.jvmti_raw(), GetTopThreadGroups,
            &mut thread_group_builder.count,
            &mut thread_group_builder.items
        );
        jvmti_error_code_to_result(res)?;
        Ok(thread_group_builder.build())
    }

    pub fn get_thread_group_info(&self, thread_group: &JThreadGroupID) -> Result<JThreadGroupInfo> {
        let mut into_ptr: jvmtiThreadGroupInfo = jvmtiThreadGroupInfo {
            parent: ptr::null_mut(),
            name: ptr::null_mut(),
            max_priority: 0,
            is_daemon: 0,
        };
        let res = jvmti_call_result!(self.jvmti_raw(), GetThreadGroupInfo,
            thread_group.into(),
            &mut into_ptr
        );

        jvmti_error_code_to_result(res)?;

        Ok(JThreadGroupInfo::new(into_ptr))
    }

    pub fn get_thread_group_children(&self, thread_group: &JThreadGroupID) -> Result<(Vec<JThreadID>, Vec<JThreadGroupID>)> {
        let mut thread_builder: MutObjectArrayBuilder<sys::jthread> = MutObjectArrayBuilder::new();
        let mut thread_group_builder: MutObjectArrayBuilder<jthreadGroup> = MutObjectArrayBuilder::new();

        let res = jvmti_call_result!(self.jvmti_raw(), GetThreadGroupChildren,
            thread_group.into(),
            &mut thread_builder.count,
            &mut thread_builder.items,
            &mut thread_group_builder.count,
            &mut thread_group_builder.items
        );

        jvmti_error_code_to_result(res)?;

        Ok((thread_builder.build(), thread_group_builder.build()))
    }
}
