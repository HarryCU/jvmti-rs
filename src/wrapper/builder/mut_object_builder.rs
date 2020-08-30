use std::os::raw::c_void;

use log::error;

use crate::{builder::*, JVMTIEnv, JvmtiError, objects::*,
            slice_raw, sys::*};

macro_rules! define_builder {
    ($sys_type:ident, $wrapper_type:ident) => (
        impl<'a> Builder<$wrapper_type<'a>> for MutObjectArrayBuilder<$sys_type> {
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
    ($sys_type:ident, $wrapper_type:ident, $convert_type:ty) => (
        impl<'a> Builder<$wrapper_type<'a>> for MutObjectArrayBuilder<$sys_type> {
            fn build(&self) -> Vec<$wrapper_type<'a>> {
                if self.count == 0 || self.items.is_null() {
                    return vec![];
                }
                let items = slice_raw(self.items, self.count);
                let res: Vec<$wrapper_type<'a>> = items.iter()
                    .map(|&e| (e as $convert_type).into())
                    .collect();
                res
            }
        }
    )
}

macro_rules! define_auto_deallocate_builder {
    ($sys_type:ident, $wrapper_type:ident) => (
        impl<'a> AutoDeallocateBuilder<'a, $wrapper_type<'a>> for MutAutoDeallocateObjectArrayBuilder<$sys_type> {
            fn build(&self, jvmti: &'a JVMTIEnv<'a>) -> Vec<$wrapper_type<'a>> {
                if self.count == 0 || self.items.is_null() {
                    return vec![];
                }
                let items = slice_raw(self.items, self.count);
                let res: Vec<$wrapper_type<'a>> = items.iter()
                    .map(|&e| (e).into())
                    .collect();
                match jvmti.deallocate_raw(self.items as *mut c_void as jmemory) {
                    Err(e)=> error!("JVMTI deallocate memory fail {}",e),
                    _=>{}
                }
                res
            }
        }
    );
    ($sys_type:ident, $wrapper_type:ident, $convert_type:ty) => (
        impl<'a> AutoDeallocateBuilder<'a, $wrapper_type<'a>> for MutAutoDeallocateObjectArrayBuilder<$sys_type> {
            fn build(&self, jvmti: &'a JVMTIEnv<'a>) -> Vec<$wrapper_type<'a>> {
                if self.count == 0 || self.items.is_null() {
                    return vec![];
                }
                let items = slice_raw(self.items, self.count);
                let res: Vec<$wrapper_type<'a>> = items.iter()
                    .map(|&e| (e as $convert_type).into())
                    .collect();
                match jvmti.deallocate_raw(self.items as *mut c_void as jmemory) {
                    Err(e)=> error!("JVMTI deallocate memory fail {}",e),
                    _=>{}
                }
                res
            }
        }
    )
}

define_auto_deallocate_builder!(jobject, JObject);
define_auto_deallocate_builder!(jvmtiMonitorStackDepthInfo, JMonitorStackDepthInfo);
define_auto_deallocate_builder!(jvmtiParamInfo, JParamInfo);
define_builder!(jvmtiFrameInfo, JFrameInfo);
define_auto_deallocate_builder!(jvmtiStackInfo, JStackInfo);

define_auto_deallocate_builder!(jthread, JThreadID, jthread);
define_auto_deallocate_builder!(jthreadGroup, JThreadGroupID, jthreadGroup);
define_auto_deallocate_builder!(jmethodID, JMethodID, jmethodID);
define_auto_deallocate_builder!(jfieldID, JFieldID, jfieldID);
define_auto_deallocate_builder!(jclass, JClass, jclass);
define_auto_deallocate_builder!(jvmtiLineNumberEntry, JLineNumberEntry, jvmtiLineNumberEntry);

impl<'a> Builder<JvmtiError> for MutObjectArrayBuilder<jvmtiError> {
    fn build(&self) -> Vec<JvmtiError> {
        if self.count == 0 || self.items.is_null() {
            return vec![];
        }
        let items = slice_raw(self.items, self.count);
        let res: Vec<JvmtiError> = items.iter()
            .map(|&e| (e as jvmtiError).into())
            .collect();
        res
    }
}

impl<'a> Builder<JCompiledMethodLoadRecordStackInfo<'a>> for MutObjectArrayBuilder<PCStackInfo> {
    fn build(&self) -> Vec<JCompiledMethodLoadRecordStackInfo<'a>> {
        if self.count == 0 || self.items.is_null() {
            return vec![];
        }
        let res = slice_raw(self.items, self.count).iter().map(|e| {
            let method_ids = slice_raw(e.methods, e.numstackframes);
            let byte_code_indices = slice_raw(e.bcis, e.numstackframes);
            let mut stack_frames = Vec::new();
            if e.numstackframes > 0 {
                for i in 0..(e.numstackframes as usize) {
                    stack_frames.push(JCompiledMethodLoadRecordStackFrame {
                        method: method_ids[i].into(),
                        byte_code_index: byte_code_indices[i],
                    });
                }
            }
            JCompiledMethodLoadRecordStackInfo { pc_address: e.pc as usize, stack_frames: stack_frames }
        }).collect();
        res
    }
}

impl<'a> AutoDeallocateBuilder<'a, JExtensionFunctionInfo<'a>> for MutAutoDeallocateObjectArrayBuilder<jvmtiExtensionFunctionInfo> {
    fn build(&self, jvmti: &'a JVMTIEnv<'a>) -> Vec<JExtensionFunctionInfo<'a>> {
        if self.count == 0 || self.items.is_null() {
            return vec![];
        }
        let items = slice_raw(self.items, self.count);
        let res: Vec<JExtensionFunctionInfo<'a>> = items.iter()
            .map(|&e| JExtensionFunctionInfo::new(e as jvmtiExtensionFunctionInfo, jvmti))
            .collect();

        match jvmti.deallocate_raw(self.items as *mut c_void as jmemory) {
            Err(e) => error!("JVMTI deallocate memory fail {}", e),
            _ => {}
        }
        res
    }
}

impl<'a> AutoDeallocateBuilder<'a, JExtensionEventInfo<'a>> for MutAutoDeallocateObjectArrayBuilder<jvmtiExtensionEventInfo> {
    fn build(&self, jvmti: &'a JVMTIEnv<'a>) -> Vec<JExtensionEventInfo<'a>> {
        if self.count == 0 || self.items.is_null() {
            return vec![];
        }
        let items = slice_raw(self.items, self.count);
        let res: Vec<JExtensionEventInfo<'a>> = items.iter()
            .map(|&e| JExtensionEventInfo::new(e as jvmtiExtensionEventInfo, jvmti))
            .collect();

        match jvmti.deallocate_raw(self.items as *mut c_void as jmemory) {
            Err(e) => error!("JVMTI deallocate memory fail {}", e),
            _ => {}
        }
        res
    }
}

impl<'a> AutoDeallocateBuilder<'a, JLocalVariableEntry<'a>> for MutAutoDeallocateObjectArrayBuilder<jvmtiLocalVariableEntry> {
    fn build(&self, jvmti: &'a JVMTIEnv<'a>) -> Vec<JLocalVariableEntry<'a>> {
        if self.count == 0 || self.items.is_null() {
            return vec![];
        }
        let items = slice_raw(self.items, self.count);
        let res: Vec<JLocalVariableEntry<'a>> = items.iter()
            .map(|&e| {
                let entry = e as jvmtiLocalVariableEntry;
                JLocalVariableEntry::new(jvmti, &entry).unwrap()
            })
            .collect();
        match jvmti.deallocate_raw(self.items as *mut c_void as jmemory) {
            Err(e) => error!("JVMTI deallocate memory fail {}", e),
            _ => {}
        }
        res
    }
}
