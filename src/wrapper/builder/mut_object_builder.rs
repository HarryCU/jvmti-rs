use crate::wrapper::{JThreadID, Builder, slice_raw, MutObjectArrayBuilder,
                     JThreadGroupID, JCompiledMethodLoadRecordStackInfo,
                     JCompiledMethodLoadRecordStackFrame, JLineNumberEntry, JLocalVariableEntry,
                     JVMTIEnv, JvmtiError, JFrameInfo, WithJvmtiEvnBuilder, JStackInfo, JMonitorStackDepthInfo,
                     JParamInfo, JExtensionEventInfo, JExtensionFunctionInfo};
use crate::sys;
use crate::sys::{JMethodID, JFieldID, JClass, jvmtiLocalVariableEntry, JObject};

macro_rules! define_builder {
    ($sys_type:ident, $wrapper_type:ident) => (
        impl<'a> Builder<$wrapper_type<'a>> for MutObjectArrayBuilder<sys::$sys_type> {
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
        impl<'a> Builder<$wrapper_type<'a>> for MutObjectArrayBuilder<sys::$sys_type> {
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

define_builder!(jobject, JObject);
define_builder!(jvmtiMonitorStackDepthInfo, JMonitorStackDepthInfo);
define_builder!(jvmtiParamInfo, JParamInfo);
define_builder!(jvmtiExtensionEventInfo, JExtensionEventInfo);
define_builder!(jvmtiFrameInfo, JFrameInfo);
define_builder!(jvmtiStackInfo, JStackInfo);
define_builder!(jvmtiExtensionFunctionInfo, JExtensionFunctionInfo);

define_builder!(jthread, JThreadID, sys::jthread);
define_builder!(jthreadGroup, JThreadGroupID, sys::jthreadGroup);
define_builder!(jmethodID, JMethodID, sys::jmethodID);
define_builder!(jfieldID, JFieldID, sys::jfieldID);
define_builder!(jclass, JClass, sys::jclass);
define_builder!(jvmtiLineNumberEntry, JLineNumberEntry, sys::jvmtiLineNumberEntry);

impl<'a> Builder<JvmtiError> for MutObjectArrayBuilder<sys::jvmtiError> {
    fn build(&self) -> Vec<JvmtiError> {
        if self.count == 0 || self.items.is_null() {
            return vec![];
        }
        let items = slice_raw(self.items, self.count);
        let res: Vec<JvmtiError> = items.iter()
            .map(|&e| (e as sys::jvmtiError).into())
            .collect();
        res
    }
}

impl<'a> Builder<JCompiledMethodLoadRecordStackInfo<'a>> for MutObjectArrayBuilder<sys::PCStackInfo> {
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

impl<'a> WithJvmtiEvnBuilder<JLocalVariableEntry<'a>> for MutObjectArrayBuilder<sys::jvmtiLocalVariableEntry> {
    fn build<'b>(&self, env: &JVMTIEnv<'b>) -> Vec<JLocalVariableEntry<'a>> {
        if self.count == 0 || self.items.is_null() {
            return vec![];
        }
        let items = slice_raw(self.items, self.count);
        let res: Vec<JLocalVariableEntry<'a>> = items.iter()
            .map(|&e| {
                let entry = e as jvmtiLocalVariableEntry;
                JLocalVariableEntry::new(env, &entry).unwrap()
            })
            .collect();
        res
    }
}
