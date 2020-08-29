use crate::{errors::*, objects::*, JThreadGroupInfo, JVMTIFacadeEnv};

impl<'a> JVMTIFacadeEnv<'a> {
    pub fn get_top_thread_groups(&self) -> Result<Vec<JThreadGroupID>> {
        self.jvmti_rust().get_top_thread_groups()
    }

    pub fn get_thread_group_info(&self, thread_group: &JThreadGroupID) -> Result<JThreadGroupInfo> {
        self.jvmti_rust().get_thread_group_info(thread_group)
    }

    pub fn get_thread_group_children(&self, thread_group: &JThreadGroupID) -> Result<(Vec<JThreadID>, Vec<JThreadGroupID>)> {
        self.jvmti_rust().get_thread_group_children(thread_group)
    }
}
