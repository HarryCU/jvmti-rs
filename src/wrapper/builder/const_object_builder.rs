use crate::sys;
use crate::wrapper::{ObjectArrayBuilder, JAddrLocationMap, Builder, slice_raw, JCompiledMethodLoadRecordStackFrame, JCompiledMethodLoadRecordStackInfo, JVMTIEnv};

macro_rules! define_builder {
    ($sys_type:ident, $wrapper_type:ident) => (
        impl<'a> Builder<$wrapper_type<'a>> for ObjectArrayBuilder<sys::$sys_type> {
            fn build(&self) -> Vec<$wrapper_type<'a>> {
                if self.count == 0 || self.items.is_null() {
                    return vec![];
                }
                let items = slice_raw(self.items, self.count);
                let res: Vec<$wrapper_type<'a>> = items.iter()
                    .map(|&e| (e).into())
                    .collect();
                res
            }
        }
    );
}

define_builder!(jvmtiAddrLocationMap, JAddrLocationMap);
