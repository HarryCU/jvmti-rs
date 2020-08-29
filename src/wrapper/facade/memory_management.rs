use crate::{sys::*, errors::*, objects::*, JVMTIFacadeEnv};

impl<'a> JVMTIFacadeEnv<'a> {
    pub fn allocate(&self, size: jlong) -> Result<Option<JMemoryAllocate>> {
        self.jvmti_rust().allocate(size)
    }

    pub fn deallocate<T>(&self, memory: &T) -> Result<()>
        where
            T: JDeallocate<'a> {
        self.jvmti_rust().deallocate(memory)
    }
}
