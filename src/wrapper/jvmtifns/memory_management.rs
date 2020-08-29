use std::ptr;

use crate::{
    sys::*,
    errors::*,
    objects::*,
    JVMTIEnv,
};

impl<'a> JVMTIEnv<'a> {
    pub fn allocate(&self, size: jlong) -> Result<Option<JMemoryAllocate>> {
        let mut mem_ptr: jmemory = ptr::null_mut();
        let res = jvmti_call_result!(self.jvmti_raw(), Allocate,
            size,
            &mut mem_ptr
        );
        jvmti_error_code_to_result(res)?;
        if mem_ptr.is_null() {
            return Ok(None);
        }
        Ok(Some(JMemoryAllocate::new(mem_ptr, size, &self)))
    }

    pub fn deallocate<T>(&self, memory: &T) -> Result<()>
        where
            T: JDeallocate<'a> {
        jvmti_call!(self.jvmti_raw(), Deallocate,
           memory.as_deallocate_ptr()
        );
    }
}
