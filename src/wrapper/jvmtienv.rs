use std::{
    marker::PhantomData,
    ptr,
};

use log::warn;

use crate::sys;
use crate::wrapper::{
    errors::*,
    enums::*,
    objects::*,
    utils::*,
    builder::*,
    JCapabilities,
    JThreadInfo,
    JThreadGroupInfo,
    JSignature,
    JFieldName,
    JMethodName,
};
use std::ptr::{null, null_mut};
use std::ffi::c_void;
use std::os::raw::{c_int, c_uchar, c_char};
use std::any::Any;
use crate::sys::{JThrowable, jvmtiThreadInfo, jint, jobject, jthreadGroup, jvmtiThreadGroupInfo, jvmtiThreadState, jvmtiFrameInfo, JObject, jlong, jfloat, jdouble, JNIString, jrawMonitorID, JMethodID, jvmtiMonitorUsage, JFieldID, JClass, jboolean, jvmtiClassStatus, jvmtiEventCallbacks, jvmtiCapabilities, jvmtiMonitorStackDepthInfo, jvmtiStackInfo, jvmtiLocalVariableEntry, jvmtiClassDefinition, jvmtiError, jvmtiObjectReferenceCallback, jvmtiHeapRootCallback, jvmtiStackReferenceCallback, jvmtiHeapObjectFilter, jvmtiHeapObjectCallback, jvmtiHeapCallbacks, jniNativeInterface, jvmtiExtensionFunctionInfo, jvmtiExtensionEventInfo, jvmtiExtensionEvent, jvmtiJlocationFormat, jvmtiPhase, jvmtiTimerInfo, jvmtiVerboseFlag, jvmtiLineNumberEntry, jthread, jmemory, jlocation, jvmtiTimerKind, jvmtiStartFunction};
use std::borrow::BorrowMut;
use jni_sys::{jmethodID, jfieldID, jclass, JNIEnv};
use log::{debug, error};

#[macro_export]
macro_rules! jvmti_catch {
    ($event:expr, $name:tt, $callback:block) => {
        debug!("looking up JVMTIEnv method {}", stringify!($name));
        let call = $callback;
        match &$event.jvmti.$name() {
            Ok(val) => call(val),
            Err(e) => error!("call jvmti an exception occurs: {}", e)
        }
    };
    ($event:expr, $name:tt, $callback:block $(, $args:expr )* ) => {
        debug!("looking up JVMTIEnv method {}", stringify!($name));
        let call = $callback;
        match &$event.jvmti.$name($($args),*,) {
            Ok(val) => call(val),
            Err(e) => error!("call jvmti an exception occurs: {}", e)
        }
    };
}

#[derive(Clone)]
#[repr(transparent)]
pub struct JVMTIEnv<'a> {
    internal: *mut sys::JVMTIEnv,
    lifetime: PhantomData<&'a ()>,
}

impl<'a> JVMTIEnv<'a> {
    pub unsafe fn from_raw(ptr: *mut sys::JVMTIEnv) -> Result<Self> {
        non_null!(ptr, "from_raw ptr argument");
        Ok(JVMTIEnv {
            internal: ptr,
            lifetime: PhantomData,
        })
    }

    fn build_string(&self, value: *mut c_char) -> Result<JString> {
        Ok(JString::with_pointer(value, self))
    }

    pub fn get_capabilities(&self) -> Result<JCapabilities> {
        let mut capabilities: jvmtiCapabilities = jvmtiCapabilities {
            _bitfield_1: jvmtiCapabilities::newEmptyBitfield(),
        };
        let res = jvmti_call_result!(self.internal, GetCapabilities,
            &mut capabilities
        );
        jvmti_error_code_to_result(res)?;
        Ok(JCapabilities::merge(capabilities))
    }

    pub fn get_potential_capabilities(&self) -> Result<JCapabilities> {
        let mut capabilities: jvmtiCapabilities = jvmtiCapabilities {
            _bitfield_1: jvmtiCapabilities::newEmptyBitfield(),
        };
        let res = jvmti_call_result!(self.internal, GetPotentialCapabilities,
            &mut capabilities
        );
        jvmti_error_code_to_result(res)?;
        Ok(JCapabilities::merge(capabilities))
    }

    pub fn add_capabilities(&self, caps: &JCapabilities) -> Result<()> {
        let capabilities_ptr: jvmtiCapabilities = (*caps).into();
        jvmti_call!(self.internal, AddCapabilities,
            &capabilities_ptr
        )
    }

    pub fn relinquish_capabilities(&self, caps: &JCapabilities) -> Result<()> {
        let capabilities_ptr: jvmtiCapabilities = (*caps).into();
        jvmti_call!(self.internal, RelinquishCapabilities,
            &capabilities_ptr
        )
    }

    pub fn set_event_notification_mode(&self, mode: JvmtiEventMode,
                                       event_type: JvmtiEvent,
                                       event_thread: &Option<JThreadID>) -> Result<()> {
        jvmti_call!(self.internal, SetEventNotificationMode,
            mode.into(),
            event_type.into(),
            event_thread.map_or_else(|| ptr::null_mut(), |t| t.into())
        )
    }

    pub fn get_all_threads(&self) -> Result<Vec<JThreadID>> {
        let mut builder: MutObjectArrayBuilder<sys::jthread> = MutObjectArrayBuilder::new();
        let res = jvmti_call_result!(self.internal, GetAllThreads,
            &mut builder.count,
            &mut builder.items
        );
        jvmti_error_code_to_result(res)?;
        Ok(builder.build())
    }

    pub fn suspend_thread(&self, thread: &JThreadID) -> Result<()> {
        jvmti_call!(self.internal, SuspendThread,
            thread.into()
        )
    }

    pub fn resume_thread(&self, thread: &JThreadID) -> Result<()> {
        jvmti_call!(self.internal, ResumeThread,
            thread.into()
        )
    }

    pub fn stop_thread(&self, thread: &JThreadID, exception: &JThrowable) -> Result<()> {
        jvmti_call!(self.internal, StopThread,
            thread.into(),
            exception.into_inner()
        )
    }

    pub fn interrupt_thread(&self, thread: &JThreadID) -> Result<()> {
        jvmti_call!(self.internal, InterruptThread,
            thread.into()
        )
    }

    pub fn run_agent_thread(&self, thread: &JThreadID,
                            proc: jvmtiStartFunction,
                            arg: *const c_void,
                            priority: JvmtiThreadPriority,
    ) -> Result<()> {
        jvmti_call!(self.internal, RunAgentThread,
            thread.into(),
            proc,
            arg,
            priority.into()
        )
    }

    pub fn get_thread_info(&self, thread: &JThreadID) -> Result<JThreadInfo> {
        let mut into = JThreadInfo::empty_raw();
        let res = jvmti_call_result!(self.internal, GetThreadInfo,
            thread.into(),
            &mut into
        );
        jvmti_error_code_to_result(res)?;
        Ok(JThreadInfo::new(into))
    }

    pub fn get_owned_monitor_info(&self, thread: &JThreadID) -> Result<Vec<JObject>> {
        let mut builder: MutObjectArrayBuilder<jobject> = MutObjectArrayBuilder::new();
        let res = jvmti_call_result!(self.internal, GetOwnedMonitorInfo,
            thread.into(),
            &mut builder.count,
            &mut builder.items
        );
        jvmti_error_code_to_result(res)?;
        Ok(builder.build())
    }

    pub fn get_current_contended_monitor(&self, thread: &JThreadID) -> Result<Option<JObject>> {
        let res = jvmti_call_ptr_result!(self.internal, jobject,
            GetCurrentContendedMonitor,
            thread.into()
        );
        if res.is_null() {
            return Ok(None);
        }
        Ok(Some(res.into()))
    }

    pub fn get_top_thread_groups(&self) -> Result<Vec<JThreadGroupID>> {
        let mut thread_group_builder: MutObjectArrayBuilder<jthreadGroup> = MutObjectArrayBuilder::new();
        let res = jvmti_call_result!( self.internal, GetTopThreadGroups,
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
        let res = jvmti_call_result!(self.internal, GetThreadGroupInfo,
            thread_group.into(),
            &mut into_ptr
        );

        jvmti_error_code_to_result(res)?;

        Ok(JThreadGroupInfo::new(into_ptr))
    }

    pub fn get_thread_group_children(&self, thread_group: &JThreadGroupID) -> Result<(Vec<JThreadID>, Vec<JThreadGroupID>)> {
        let mut thread_builder: MutObjectArrayBuilder<sys::jthread> = MutObjectArrayBuilder::new();
        let mut thread_group_builder: MutObjectArrayBuilder<jthreadGroup> = MutObjectArrayBuilder::new();

        let res = jvmti_call_result!(self.internal, GetThreadGroupChildren,
            thread_group.into(),
            &mut thread_builder.count,
            &mut thread_builder.items,
            &mut thread_group_builder.count,
            &mut thread_group_builder.items
        );

        jvmti_error_code_to_result(res)?;

        Ok((thread_builder.build(), thread_group_builder.build()))
    }

    pub fn get_frame_count(&self, thread: &JThreadID) -> Result<jint> {
        Ok(jvmti_call_number_result!(self.internal, jint,
            GetFrameCount,
            thread.into()
        ))
    }

    pub fn get_thread_state(&self, thread: &JThreadID) -> Result<JvmtiThreadState> {
        let res = jvmti_call_number_result!(self.internal, jint,
            GetThreadState,
            thread.into()
        );
        Ok((res as jvmtiThreadState).into())
    }

    pub fn get_current_thread(&self) -> Result<Option<JThreadID>> {
        let mut thread_ptr: sys::jthread = ptr::null_mut();
        let res = jvmti_call_result!(self.internal, GetCurrentThread,
            &mut thread_ptr
        );

        jvmti_error_code_to_result(res)?;

        if thread_ptr.is_null() {
            return Ok(None);
        }
        Ok(Some(thread_ptr.into()))
    }

    pub fn get_frame_location(&self, thread: &JThreadID, dept: jint) -> Result<Option<JFrameInfo>> {
        let mut info: jvmtiFrameInfo = jvmtiFrameInfo {
            method: ptr::null_mut(),
            location: 0,
        };

        let res = jvmti_call_result!(self.internal, GetFrameLocation,
            thread.into(),
            dept,
            &mut info.method,
            &mut info.location
        );

        jvmti_error_code_to_result(res)?;

        if info.method.is_null() || info.location == 0 {
            return Ok(None);
        }
        Ok(Some(info.into()))
    }

    pub fn notify_frame_pop(&self, thread: &JThreadID, dept: jint) -> Result<()> {
        jvmti_call!(self.internal, NotifyFramePop,
            thread.into(),
            dept
        )
    }

    pub fn get_local_object(&self, thread: &JThreadID, dept: jint, slot: jint) -> Result<Option<JObject>> {
        let res = jvmti_call_ptr_result!(self.internal, jobject,
            GetLocalObject,
            thread.into(),
            dept,
            slot
        );
        if res.is_null() {
            return Ok(None);
        }
        Ok(Some(res.into()))
    }

    pub fn get_local_int(&self, thread: &JThreadID, dept: jint, slot: jint) -> Result<jint> {
        Ok(jvmti_call_number_result!(self.internal, jint,
            GetLocalInt,
            thread.into(),
            dept,
            slot
        ))
    }

    pub fn get_local_long(&self, thread: &JThreadID, dept: jint, slot: jint) -> Result<jlong> {
        Ok(jvmti_call_number_result!(self.internal, jlong,
            GetLocalLong,
            thread.into(),
            dept,
            slot
       ))
    }

    pub fn get_local_float(&self, thread: &JThreadID, dept: jint, slot: jint) -> Result<jfloat> {
        Ok(jvmti_call_number_result!(self.internal, jfloat,
            GetLocalFloat,
            thread.into(),
            dept,
            slot
       ))
    }

    pub fn get_local_double(&self, thread: &JThreadID, dept: jint, slot: jint) -> Result<jdouble> {
        Ok(jvmti_call_number_result!(self.internal, jdouble,
            GetLocalDouble,
            thread.into(),
            dept,
            slot
       ))
    }

    pub fn set_local_object(&self, thread: &JThreadID, dept: jint, slot: jint, obj: Option<JObject>) -> Result<()> {
        jvmti_call!(self.internal, SetLocalObject,
            thread.into(),
            dept,
            slot,
            obj.map_or_else(|| ptr::null_mut(), |e| e.into_inner())
        )
    }

    pub fn set_local_int(&self, thread: &JThreadID, dept: jint, slot: jint, value: jint) -> Result<()> {
        jvmti_call!(self.internal, SetLocalInt,
            thread.into(),
            dept,
            slot,
            value
        )
    }

    pub fn set_local_long(&self, thread: &JThreadID, dept: jint, slot: jint, value: jlong) -> Result<()> {
        jvmti_call!(self.internal, SetLocalLong,
            thread.into(),
            dept,
            slot,
            value
        )
    }

    pub fn set_local_float(&self, thread: &JThreadID, dept: jint, slot: jint, value: jfloat) -> Result<()> {
        jvmti_call!(self.internal, SetLocalFloat,
            thread.into(),
            dept,
            slot,
            value
        )
    }

    pub fn set_local_double(&self, thread: &JThreadID, dept: jint, slot: jint, value: jdouble) -> Result<()> {
        jvmti_call!(self.internal, SetLocalDouble,
            thread.into(),
            dept,
            slot,
            value
        )
    }

    pub fn create_raw_monitor<N>(&self, name: N) -> Result<Option<JRawMonitorID>>
        where
            N: Into<JNIString>, {
        let ffi_name = name.into();
        let mut value_ptr: jrawMonitorID = ptr::null_mut();
        let res = jvmti_call_result!(self.internal, CreateRawMonitor,
            ffi_name.as_ptr(),
            &mut value_ptr
        );
        jvmti_error_code_to_result(res)?;

        if value_ptr.is_null() {
            return Ok(None);
        }
        Ok(Some(value_ptr.into()))
    }

    pub fn destroy_raw_monitor(&self, monitor_id: &JRawMonitorID) -> Result<()> {
        jvmti_call!(self.internal, DestroyRawMonitor,
            monitor_id.into()
        )
    }

    pub fn raw_monitor_enter(&self, monitor_id: &JRawMonitorID) -> Result<()> {
        jvmti_call!(self.internal, RawMonitorEnter,
            monitor_id.into()
        )
    }

    pub fn raw_monitor_exit(&self, monitor_id: &JRawMonitorID) -> Result<()> {
        jvmti_call!(self.internal, RawMonitorExit,
            monitor_id.into()
        )
    }

    pub fn raw_monitor_wait(&self, monitor_id: &JRawMonitorID, millis: jlong) -> Result<()> {
        jvmti_call!(self.internal, RawMonitorWait,
            monitor_id.into(),
            millis
        )
    }

    pub fn raw_monitor_notify(&self, monitor_id: &JRawMonitorID) -> Result<()> {
        jvmti_call!(self.internal, RawMonitorNotify,
            monitor_id.into()
        )
    }

    pub fn raw_monitor_notify_all(&self, monitor_id: &JRawMonitorID) -> Result<()> {
        jvmti_call!(self.internal, RawMonitorNotifyAll,
            monitor_id.into()
        )
    }

    pub fn set_breakpoint(&self, method: &JMethodID, location: JLocation) -> Result<()> {
        jvmti_call!(self.internal, SetBreakpoint,
            method.into_inner(),
            location.into()
        )
    }

    pub fn clear_breakpoint(&self, method: &JMethodID, location: JLocation) -> Result<()> {
        jvmti_call!(self.internal, ClearBreakpoint,
            method.into_inner(),
            location.into()
        )
    }

    pub fn set_field_access_watch(&self, klass: &JClass, field: &JFieldID) -> Result<()> {
        jvmti_call!(self.internal, SetFieldAccessWatch,
            klass.into_inner(),
            field.into_inner()
        )
    }

    pub fn clear_field_access_watch(&self, klass: &JClass, field: &JFieldID) -> Result<()> {
        jvmti_call!(self.internal, ClearFieldAccessWatch,
            klass.into_inner(),
            field.into_inner()
        )
    }

    pub fn set_field_modification_watch(&self, klass: &JClass, field: &JFieldID) -> Result<()> {
        jvmti_call!(self.internal, SetFieldModificationWatch,
            klass.into_inner(),
            field.into_inner()
        )
    }

    pub fn clear_field_modification_watch(&self, klass: &JClass, field: &JFieldID) -> Result<()> {
        jvmti_call!(self.internal, ClearFieldModificationWatch,
            klass.into_inner(),
            field.into_inner()
        )
    }

    pub fn is_modifiable_class(&self, klass: &JClass) -> Result<bool> {
        let res = jvmti_call_number_result!(self.internal, jboolean,
            IsModifiableClass,
            klass.into_inner()
        );
        Ok(to_bool(res))
    }

    pub fn allocate(&self, size: jlong) -> Result<Option<JMemoryAllocate>> {
        let mut mem_ptr: jmemory = ptr::null_mut();
        let res = jvmti_call_result!(self.internal, Allocate,
            size,
            &mut mem_ptr
        );
        jvmti_error_code_to_result(res)?;
        if mem_ptr.is_null() {
            return Ok(None);
        }
        Ok(Some(JMemoryAllocate::new(mem_ptr, size, &self)))
    }

    pub fn deallocate<T>(&self, str: &T) -> Result<()>
        where
            T: JDeallocate<'a> {
        jvmti_call!(self.internal, Deallocate,
           str.as_ptr()
        );
    }

    pub fn get_class_signature(&self, klass: &JClass) -> Result<JSignature> {
        let mut signature = ptr::null_mut();
        let mut generic = ptr::null_mut();
        let res = jvmti_call_result!(self.internal, GetClassSignature,
            klass.into_inner(),
            &mut signature,
            &mut generic
        );
        jvmti_error_code_to_result(res)?;

        JSignature::new(self.build_string(signature)?, self.build_string(generic)?)
    }

    pub fn get_class_status(&self, klass: &JClass) -> Result<JvmtiClassStatus> {
        let res = jvmti_call_number_result!(self.internal, jint,
            GetClassStatus,
            klass.into_inner()
        );
        Ok((res as jvmtiClassStatus).into())
    }

    pub fn get_source_file_name(&self, klass: &JClass) -> Result<JString> {
        let mut source_name = ptr::null_mut();
        let res = jvmti_call_result!(self.internal, GetSourceFileName,
            klass.into_inner(),
            &mut source_name
        );
        jvmti_error_code_to_result(res)?;
        self.build_string(source_name)
    }

    pub fn get_class_modifiers(&self, klass: &JClass) -> Result<jint> {
        Ok(jvmti_call_number_result!(self.internal, jint,
            GetClassModifiers,
            klass.into_inner()
        ))
    }

    pub fn get_class_methods(&self, klass: &JClass) -> Result<Vec<JMethodID>> {
        let mut builder: MutObjectArrayBuilder<jmethodID> = MutObjectArrayBuilder::new();
        let res = jvmti_call_result!( self.internal, GetClassMethods,
            klass.into_inner(),
            &mut builder.count,
            &mut builder.items
        );
        jvmti_error_code_to_result(res)?;
        Ok(builder.build())
    }

    pub fn get_class_fields(&self, klass: &JClass) -> Result<Vec<JFieldID>> {
        let mut builder: MutObjectArrayBuilder<jfieldID> = MutObjectArrayBuilder::new();
        let res = jvmti_call_result!( self.internal, GetClassFields,
            klass.into_inner(),
            &mut builder.count,
            &mut builder.items
        );
        jvmti_error_code_to_result(res)?;
        Ok(builder.build())
    }

    pub fn get_implemented_interfaces(&self, klass: &JClass) -> Result<Vec<JClass>> {
        let mut builder: MutObjectArrayBuilder<jclass> = MutObjectArrayBuilder::new();
        let res = jvmti_call_result!( self.internal, GetImplementedInterfaces,
            klass.into_inner(),
            &mut builder.count,
            &mut builder.items
        );
        jvmti_error_code_to_result(res)?;
        Ok(builder.build())
    }

    pub fn is_interface(&self, klass: &JClass) -> Result<bool> {
        let res = jvmti_call_number_result!(self.internal, jboolean,
            IsInterface,
            klass.into_inner()
        );
        Ok(to_bool(res))
    }

    pub fn is_array_class(&self, klass: &JClass) -> Result<bool> {
        let res = jvmti_call_number_result!(self.internal, jboolean,
            IsArrayClass,
            klass.into_inner()
        );
        Ok(to_bool(res))
    }

    pub fn get_class_loader(&self, klass: &JClass) -> Result<JClassLoader> {
        let mut value_ptr: jobject = ptr::null_mut();
        let res = jvmti_call_result!(self.internal, GetClassLoader,
            klass.into_inner(),
            &mut value_ptr
        );
        jvmti_error_code_to_result(res)?;
        return Ok(value_ptr.into());
    }

    pub fn get_object_hash_code(&self, obj: &JObject) -> Result<jint> {
        Ok(jvmti_call_number_result!(self.internal, jint,
            GetObjectHashCode,
            obj.into_inner()
        ))
    }

    pub fn get_object_monitor_usage(&self, obj: &JObject) -> Result<JMonitorUsage> {
        let mut into_ptr: jvmtiMonitorUsage = jvmtiMonitorUsage {
            owner: ptr::null_mut(),
            entry_count: 0,
            waiter_count: 0,
            waiters: ptr::null_mut(),
            notify_waiter_count: 0,
            notify_waiters: ptr::null_mut(),
        };
        let res = jvmti_call_result!(self.internal, GetObjectMonitorUsage,
            obj.into_inner(),
            &mut into_ptr
        );

        jvmti_error_code_to_result(res)?;

        Ok(into_ptr.into())
    }

    pub fn get_field_name(&self, klass: &JClass, field: &JFieldID) -> Result<JFieldName> {
        let mut name = ptr::null_mut();
        let mut signature = ptr::null_mut();
        let mut generic = ptr::null_mut();
        let res = jvmti_call_result!(self.internal, GetFieldName,
            klass.into_inner(),
            field.into_inner(),
            &mut name,
            &mut signature,
            &mut generic
        );
        jvmti_error_code_to_result(res)?;

        let signature = JSignature::new(self.build_string(signature)?, self.build_string(generic)?)?;
        Ok(JFieldName::new(self.build_string(name)?, signature))
    }

    pub fn get_field_declaring_class(&self, klass: &JClass, field: &JFieldID) -> Result<JObject> {
        let mut value_ptr: jclass = ptr::null_mut();
        let res = jvmti_call_result!(self.internal, GetFieldDeclaringClass,
            klass.into_inner(),
            field.into_inner(),
            &mut value_ptr
        );
        jvmti_error_code_to_result(res)?;
        Ok(value_ptr.into())
    }

    pub fn get_field_modifiers(&self, klass: &JClass, field: &JFieldID) -> Result<jint> {
        Ok(jvmti_call_number_result!(self.internal, jint,
            GetFieldModifiers,
            klass.into_inner(),
            field.into_inner()
        ))
    }

    pub fn is_field_synthetic(&self, klass: &JClass, field: &JFieldID) -> Result<bool> {
        let res = jvmti_call_number_result!(self.internal, jboolean,
            IsFieldSynthetic,
            klass.into_inner(),
            field.into_inner()
        );
        Ok(to_bool(res))
    }

    pub fn get_method_name(&self, method: &JMethodID) -> Result<JMethodName> {
        let mut name = ptr::null_mut();
        let mut signature = ptr::null_mut();
        let mut generic = ptr::null_mut();
        let res = jvmti_call_result!(self.internal, GetMethodName,
            method.into_inner(),
            &mut name,
            &mut signature,
            &mut generic
        );
        jvmti_error_code_to_result(res)?;

        let signature = JSignature::new(self.build_string(signature)?, self.build_string(generic)?)?;
        Ok(JMethodName::new(self.build_string(name)?, signature))
    }

    pub fn get_method_declaring_class(&self, method: &JMethodID) -> Result<JObject> {
        let mut value_ptr: jclass = ptr::null_mut();
        let res = jvmti_call_result!(self.internal, GetMethodDeclaringClass,
            method.into_inner(),
            &mut value_ptr
        );
        jvmti_error_code_to_result(res)?;
        Ok(value_ptr.into())
    }

    pub fn get_method_modifiers(&self, method: &JMethodID) -> Result<jint> {
        Ok(jvmti_call_number_result!(self.internal, jint,
            GetMethodModifiers,
            method.into_inner()
        ))
    }

    pub fn set_event_callbacks(&self, callbacks: &JEventCallbacks) -> Result<()> {
        let jvmti_callbacks: *const jvmtiEventCallbacks = &(*callbacks).into();
        jvmti_call!(self.internal, SetEventCallbacks,
            jvmti_callbacks,
            JEventCallbacks::size_of_callbacks()
        )
    }

    pub fn get_max_locals(&self, method: &JMethodID) -> Result<jint> {
        Ok(jvmti_call_number_result!(self.internal, jint,
            GetMaxLocals,
            method.into_inner()
        ))
    }

    pub fn get_arguments_size(&self, method: &JMethodID) -> Result<jint> {
        Ok(jvmti_call_number_result!(self.internal, jint,
            GetArgumentsSize,
            method.into_inner()
        ))
    }

    pub fn get_line_number_table(&self, method: &JMethodID) -> Result<Vec<JLineNumberEntry>> {
        let mut builder: MutObjectArrayBuilder<jvmtiLineNumberEntry> = MutObjectArrayBuilder::new();
        let res = jvmti_call_result!( self.internal, GetLineNumberTable,
            method.into_inner(),
            &mut builder.count,
            &mut builder.items
        );
        jvmti_error_code_to_result(res)?;
        Ok(builder.build())
    }

    pub fn get_method_location(&self, method: &JMethodID) -> Result<JMethodLocation> {
        let mut start_location: sys::jlocation = 0 as sys::jlocation;
        let mut end_location: sys::jlocation = 0 as sys::jlocation;
        let res = jvmti_call_result!(self.internal, GetMethodLocation,
            method.into_inner(),
            &mut start_location,
            &mut end_location
        );
        jvmti_error_code_to_result(res)?;
        Ok(JMethodLocation::new(start_location, end_location))
    }

    pub fn get_local_variable_table(&self, method: &JMethodID) -> Result<Vec<JLocalVariableEntry>> {
        let mut builder: MutObjectArrayBuilder<jvmtiLocalVariableEntry> = MutObjectArrayBuilder::new();
        let res = jvmti_call_result!( self.internal, GetLocalVariableTable,
            method.into_inner(),
            &mut builder.count,
            &mut builder.items
        );
        jvmti_error_code_to_result(res)?;
        Ok(builder.build(self))
    }

    pub fn set_native_method_prefix<S>(&self, prefix: S) -> Result<()>
        where
            S: Into<JNIString> {
        let value = prefix.into();
        jvmti_call!(self.internal, SetNativeMethodPrefix,
            value.as_ptr()
        )
    }

    pub fn get_bytecodes(&self, method: &JMethodID) -> Result<JMemoryAllocate> {
        let mut bytecode_count_ptr: jint = 0 as jint;
        let mut bytecodes_ptr: jmemory = ptr::null_mut();
        let res = jvmti_call_result!(self.internal, GetBytecodes,
            method.into_inner(),
            &mut bytecode_count_ptr,
            &mut bytecodes_ptr
        );
        jvmti_error_code_to_result(res)?;
        Ok(JMemoryAllocate::new(bytecodes_ptr, bytecode_count_ptr as jlong, self))
    }

    pub fn is_method_native(&self, method: &JMethodID) -> Result<bool> {
        let res = jvmti_call_number_result!(self.internal, jboolean,
            IsMethodNative,
            method.into_inner()
        );
        Ok(to_bool(res))
    }

    pub fn is_method_synthetic(&self, method: &JMethodID) -> Result<bool> {
        let res = jvmti_call_number_result!(self.internal, jboolean,
            IsMethodSynthetic,
            method.into_inner()
        );
        Ok(to_bool(res))
    }

    pub fn get_loaded_classes(&self) -> Result<Vec<JClass>> {
        let mut builder: MutObjectArrayBuilder<jclass> = MutObjectArrayBuilder::new();
        let res = jvmti_call_result!( self.internal, GetLoadedClasses,
            &mut builder.count,
            &mut builder.items
        );
        jvmti_error_code_to_result(res)?;
        Ok(builder.build())
    }

    pub fn get_class_loader_classes(&self, initiating_loader: &JObject) -> Result<Vec<JClass>> {
        let mut builder: MutObjectArrayBuilder<jclass> = MutObjectArrayBuilder::new();
        let res = jvmti_call_result!( self.internal, GetClassLoaderClasses,
            initiating_loader.into_inner(),
            &mut builder.count,
            &mut builder.items
        );
        jvmti_error_code_to_result(res)?;
        Ok(builder.build())
    }

    pub fn retransform_classes(&self, classes: &Vec<JClass>) -> Result<()> {
        if classes.is_empty() {
            return Ok(());
        }
        let items: Vec<jobject> = classes.iter().map(|&e| e.into_inner()).collect();
        jvmti_call!( self.internal, RetransformClasses,
            items.len() as jint,
            items.as_ptr()
        )
    }

    pub fn pop_frame(&self, thread: &JThreadID) -> Result<()> {
        jvmti_call!(self.internal, PopFrame,
            thread.into()
        )
    }

    pub fn force_early_return_object(&self, thread: &JThreadID, value: &JObject) -> Result<()> {
        jvmti_call!(self.internal, ForceEarlyReturnObject,
            thread.into(),
            value.into_inner()
        )
    }

    pub fn force_early_return_int(&self, thread: &JThreadID, value: jint) -> Result<()> {
        jvmti_call!(self.internal, ForceEarlyReturnInt,
            thread.into(),
            value
        )
    }

    pub fn force_early_return_long(&self, thread: &JThreadID, value: jlong) -> Result<()> {
        jvmti_call!(self.internal, ForceEarlyReturnLong,
            thread.into(),
            value
        )
    }

    pub fn force_early_return_float(&self, thread: &JThreadID, value: jfloat) -> Result<()> {
        jvmti_call!(self.internal, ForceEarlyReturnFloat,
            thread.into(),
            value
        )
    }

    pub fn force_early_return_double(&self, thread: &JThreadID, value: jdouble) -> Result<()> {
        jvmti_call!(self.internal, ForceEarlyReturnDouble,
            thread.into(),
            value
        )
    }

    pub fn force_early_return_void(&self, thread: &JThreadID) -> Result<()> {
        jvmti_call!(self.internal, ForceEarlyReturnVoid,
            thread.into()
        )
    }

    pub fn force_garbage_collection(&self) -> Result<()> {
        jvmti_call!(self.internal, ForceGarbageCollection)
    }

    pub fn dispose_environment(&self) -> Result<()> {
        jvmti_call!(self.internal, DisposeEnvironment)
    }

    pub fn generate_events(&self, event_type: JvmtiEvent) -> Result<()> {
        jvmti_call!(self.internal, GenerateEvents,
            event_type.into()
        )
    }

    pub fn define_class_definition(&self, klass: JClass<'a>, size: jlong, code_bytes: jmemory) -> Result<JClassDefinition> {
        let definition = JClassDefinition::new(klass, code_bytes, size, self);
        Ok(definition)
    }

    pub fn redefine_classes(&self, classes: &Vec<JClassDefinition>) -> Result<()> {
        if classes.is_empty() {
            return Ok(());
        }
        let definitions: Vec<jvmtiClassDefinition> = classes.iter().map(|e| (e).into()).collect();
        jvmti_call!(self.internal, RedefineClasses,
            definitions.len() as jint,
            definitions.as_ptr()
        )
    }

    pub fn get_version_number(&self) -> Result<jint> {
        Ok(jvmti_call_number_result!(self.internal, jint,
            GetVersionNumber
        ))
    }

    pub fn get_source_debug_extension(&self, klass: &JClass) -> Result<JString> {
        let mut source_debug_extension = ptr::null_mut();
        let res = jvmti_call_result!(self.internal, GetSourceDebugExtension,
            klass.into_inner(),
            &mut source_debug_extension
        );
        jvmti_error_code_to_result(res)?;
        self.build_string(source_debug_extension)
    }

    pub fn is_method_obsolete(&self, method: &JMethodID) -> Result<bool> {
        let res = jvmti_call_number_result!(self.internal, jboolean,
            IsMethodObsolete,
            method.into_inner()
        );
        Ok(to_bool(res))
    }

    pub fn suspend_thread_list(&self, threads: &Vec<JThreadID>) -> Result<JvmtiError> {
        if threads.is_empty() {
            return Ok(JvmtiError::EmptyArgument);
        }
        let thread_ids: Vec<jthread> = threads.iter().map(|&e| e.into()).collect();
        let res = jvmti_call_number_result!(self.internal, jvmtiError,
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
        let res = jvmti_call_number_result!(self.internal, jvmtiError,
            ResumeThreadList,
            thread_ids.len() as jint,
            thread_ids.as_ptr()
        );
        Ok(res.into())
    }

    pub fn get_available_processors(&self) -> Result<jint> {
        Ok(jvmti_call_number_result!(self.internal, jint,
            GetAvailableProcessors
        ))
    }

    pub fn get_object_size(&self, object: &JObject) -> Result<jlong> {
        Ok(jvmti_call_number_result!(self.internal, jlong,
            GetObjectSize,
            object.into_inner()
        ))
    }

    pub fn get_all_stack_traces(&self, max_frame_count: jint) -> Result<(Vec<JStackInfo>, jint)> {
        let mut builder: MutObjectArrayBuilder<jvmtiStackInfo> = MutObjectArrayBuilder::with_size(max_frame_count);
        let mut thread_count: jint = 0 as jint;
        let res = jvmti_call_result!(self.internal, GetAllStackTraces,
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
        let jthreads: Vec<jthread> = threads.iter().map(|&e| e.into()).collect();
        let res = jvmti_call_result!( self.internal, GetThreadListStackTraces,
            jthreads.len() as jint,
            jthreads.as_ptr(),
            max_frame_count,
            &mut builder.items
        );
        jvmti_error_code_to_result(res)?;
        Ok(builder.build())
    }

    pub fn get_thread_local_storage(&self, thread: &JThreadID) -> Result<JLocalStorage> {
        let mut data_ptr: *mut c_void = ptr::null_mut() as *mut c_void;
        let res = jvmti_call_result!(self.internal, GetThreadLocalStorage,
            thread.into(),
            &mut data_ptr
        );
        jvmti_error_code_to_result(res)?;
        Ok(JLocalStorage::new(data_ptr))
    }

    pub fn set_thread_local_storage(&self, thread: &JThreadID, data: &JLocalStorage) -> Result<()> {
        jvmti_call!(self.internal, SetThreadLocalStorage,
            thread.into(),
            data.as_ptr()
        )
    }

    pub fn get_stack_trace(&self, thread: &JThreadID, start_depth: jint, max_frame_count: jint) -> Result<(JFrameInfo, jint)> {
        let mut frame_info: jvmtiFrameInfo = jvmtiFrameInfo {
            method: ptr::null_mut(),
            location: 0 as jlocation,
        };
        let mut count: jint = 0 as jint;
        let res = jvmti_call_result!(self.internal, GetStackTrace,
            thread.into(),
            start_depth,
            max_frame_count,
            &mut frame_info,
            &mut count
        );
        jvmti_error_code_to_result(res)?;
        Ok((frame_info.into(), count))
    }

    pub fn get_tag(&self, object: &JObject) -> Result<jlong> {
        Ok(jvmti_call_number_result!(self.internal, jlong,
            GetTag,
            object.into_inner()
       ))
    }

    pub fn set_tag(&self, object: &JObject, tag: jlong) -> Result<()> {
        jvmti_call!(self.internal, SetTag,
            object.into_inner(),
            tag
        )
    }

    pub fn iterate_over_objects_reachable_from_object(&self, object: &JObject, callback: jvmtiObjectReferenceCallback, user_data: *const c_void) -> Result<()> {
        jvmti_call!(self.internal, IterateOverObjectsReachableFromObject,
            object.into_inner(),
            callback,
            user_data
        )
    }

    pub fn iterate_over_reachable_objects(&self, heap_root_callback: jvmtiHeapRootCallback,
                                          stack_ref_callback: jvmtiStackReferenceCallback,
                                          object_ref_callback: jvmtiObjectReferenceCallback,
                                          user_data: *const c_void,
    ) -> Result<()> {
        jvmti_call!(self.internal, IterateOverReachableObjects,
            heap_root_callback,
            stack_ref_callback,
            object_ref_callback,
            user_data
        )
    }

    pub fn iterate_over_heap(&self, object_filter: jvmtiHeapObjectFilter,
                             heap_object_callback: jvmtiHeapObjectCallback,
                             user_data: *const c_void,
    ) -> Result<()> {
        jvmti_call!(self.internal, IterateOverHeap,
            object_filter,
            heap_object_callback,
            user_data
        )
    }

    pub fn iterate_over_instances_of_class(&self, klass: &JClass,
                                           object_filter: jvmtiHeapObjectFilter,
                                           heap_object_callback: jvmtiHeapObjectCallback,
                                           user_data: *const c_void,
    ) -> Result<()> {
        jvmti_call!(self.internal, IterateOverInstancesOfClass,
            klass.into_inner(),
            object_filter,
            heap_object_callback,
            user_data
        )
    }

    pub fn follow_references(&self, heap_filter: jint, klass: &JClass,
                             initial_object: &JObject,
                             callbacks: &Vec<jvmtiHeapCallbacks>,
                             user_data: *const c_void,
    ) -> Result<()> {
        jvmti_call!(self.internal, FollowReferences,
            heap_filter,
            klass.into_inner(),
            initial_object.into_inner(),
            callbacks.as_ptr(),
            user_data
        )
    }

    pub fn iterate_through_heap(&self, heap_filter: jint, klass: &JClass,
                                callbacks: &Vec<jvmtiHeapCallbacks>,
                                user_data: *const c_void,
    ) -> Result<()> {
        jvmti_call!(self.internal, IterateThroughHeap,
            heap_filter,
            klass.into_inner(),
            callbacks.as_ptr(),
            user_data
        )
    }

    pub fn set_extension_event_callback(&self, extension_event_index: jint, callback: jvmtiExtensionEvent) -> Result<()> {
        jvmti_call!(self.internal, SetExtensionEventCallback,
            extension_event_index,
            callback
        )
    }

    pub fn get_error_name(&self, error: JvmtiError) -> Result<JString> {
        let mut name = ptr::null_mut();
        let err: jvmtiError = error.into();
        let res = jvmti_call_result!(self.internal, GetErrorName,
            err,
            &mut name
        );
        jvmti_error_code_to_result(res)?;
        self.build_string(name)
    }

    pub fn get_system_properties(&self) -> Result<Vec<JString>> {
        let mut count: jint = 0 as jint;
        let mut properties: *mut *mut c_char = ptr::null_mut();

        let res = jvmti_call_result!(self.internal, GetSystemProperties,
            &mut count,
            &mut properties
        );
        jvmti_error_code_to_result(res)?;

        if count == 0 {
            return Ok(vec![]);
        }

        let items = slice_raw(properties, count);
        let mut strings: Vec<JString> = Vec::with_capacity(count as usize);
        for &item in items.iter() {
            strings.push(self.build_string(item)?)
        }
        Ok(strings)
    }

    pub fn get_system_property<S>(&self, property: S) -> Result<JString>
        where
            S: Into<JNIString> {
        let ffi_name = property.into();

        let mut value = ptr::null_mut();
        let res = jvmti_call_result!(self.internal, GetSystemProperty,
            ffi_name.as_ptr(),
            &mut value
        );
        jvmti_error_code_to_result(res)?;
        self.build_string(value)
    }

    pub fn set_system_property<S>(&self, property: S, value: S) -> Result<()>
        where
            S: Into<JNIString> {
        let ffi_name = property.into();
        let ffi_value = value.into();
        jvmti_call!(self.internal, SetSystemProperty,
            ffi_name.as_ptr(),
            ffi_value.as_ptr()
        );
    }

    pub fn set_jni(&self, jni: &JNIEnv) -> Result<()> {
        jvmti_call!(self.internal, SetJNIFunctionTable,
          *jni
        )
    }

    pub fn get_jni(&self) -> Result<jni::JNIEnv> {
        let mut jni_env = ptr::null_mut();
        let res = jvmti_call_result!(self.internal, GetJNIFunctionTable,
            &mut jni_env
        );
        jvmti_error_code_to_result(res)?;
        let env = jni_env;
        unsafe {
            let mut jni = jni_env as JNIEnv;
            Ok(jni!(&mut jni as *mut JNIEnv))
        }
    }

    pub fn get_class_version_numbers(&self, klass: &JClass) -> Result<JClassVersionNumber> {
        let mut minor_version: jint = 0 as jint;
        let mut major_version: jint = 0 as jint;

        let res = jvmti_call_result!(self.internal, GetClassVersionNumbers,
            klass.into_inner(),
            &mut minor_version,
            &mut major_version
        );
        jvmti_error_code_to_result(res)?;
        Ok(JClassVersionNumber::new(minor_version, major_version))
    }

    pub fn get_environment_local_storage(&self) -> Result<JLocalStorage> {
        let mut data_ptr: *mut c_void = ptr::null_mut() as *mut c_void;
        let res = jvmti_call_result!(self.internal, GetEnvironmentLocalStorage,
            &mut data_ptr
        );
        jvmti_error_code_to_result(res)?;
        Ok(JLocalStorage::new(data_ptr))
    }

    pub fn set_environment_local_storage(&self, data: &JLocalStorage) -> Result<()> {
        jvmti_call!(self.internal, SetEnvironmentLocalStorage,
            data.as_ptr()
        )
    }

    pub fn get_local_instance(&self, thread: &JThreadID, depth: jint) -> Result<Option<JObject>> {
        let res = jvmti_call_ptr_result!(self.internal, jobject,
            GetLocalInstance,
            thread.into(),
            depth
        );
        if res.is_null() {
            return Ok(None);
        }
        Ok(Some(res.into()))
    }

    pub fn set_verbose_flag(&self, flag: JvmtiVerboseFlag, value: bool) -> Result<()> {
        jvmti_call!(self.internal, SetVerboseFlag,
            flag.into(),
            to_jboolean(value)
        )
    }

    pub fn add_to_bootstrap_class_loader_search<S>(&self, segment: S) -> Result<()>
        where
            S: Into<JNIString> {
        let seg = segment.into();
        jvmti_call!(self.internal, AddToBootstrapClassLoaderSearch,
            seg.as_ptr()
        )
    }

    pub fn add_to_system_class_loader_search<S>(&self, segment: S) -> Result<()>
        where
            S: Into<JNIString> {
        let seg = segment.into();
        jvmti_call!(self.internal, AddToSystemClassLoaderSearch,
            seg.as_ptr()
        )
    }

    pub fn get_current_thread_cpu_time(&self) -> Result<jlong> {
        Ok(jvmti_call_number_result!(self.internal, jlong,
            GetCurrentThreadCpuTime
        ))
    }

    pub fn get_thread_cpu_time(&self, thread: &JThreadID) -> Result<jlong> {
        Ok(jvmti_call_number_result!(self.internal, jlong,
            GetThreadCpuTime,
            thread.into()
        ))
    }

    pub fn get_time(&self) -> Result<jlong> {
        Ok(jvmti_call_number_result!(self.internal, jlong,
            GetTime
        ))
    }

    pub fn get_current_thread_cpu_timer_info(&self) -> Result<JTimerInfo> {
        let res = jvmti_call_number_result!(self.internal, {JTimerInfo::empty_raw()},
            GetCurrentThreadCpuTimerInfo
        );
        Ok(res.into())
    }
    pub fn get_thread_cpu_timer_info(&self) -> Result<JTimerInfo> {
        let res = jvmti_call_number_result!(self.internal, {JTimerInfo::empty_raw()},
            GetThreadCpuTimerInfo
        );
        Ok(res.into())
    }
    pub fn get_timer_info(&self) -> Result<JTimerInfo> {
        let res = jvmti_call_number_result!(self.internal, {JTimerInfo::empty_raw()},
            GetTimerInfo
        );
        Ok(res.into())
    }

    pub fn get_constant_pool(&self, klass: &JClass) -> Result<JConstantPool> {
        let mut constant_pool_count: jint = 0 as jint;
        let mut constant_pool_byte_count: jint = 0 as jint;
        let mut constant_pool_bytes: jmemory = ptr::null_mut();
        let res = jvmti_call_result!(self.internal, GetConstantPool,
            klass.into_inner(),
            &mut constant_pool_count,
            &mut constant_pool_byte_count,
            &mut constant_pool_bytes
        );
        jvmti_error_code_to_result(res)?;

        Ok(JConstantPool::new(constant_pool_count, constant_pool_byte_count, constant_pool_bytes, self))
    }

    pub fn get_jlocation_format(&self, thread: &JThreadID) -> Result<JvmtiJlocationFormat> {
        let res = jvmti_call_number_result!(self.internal, jvmtiJlocationFormat,
            GetJLocationFormat
        );
        Ok(res.into())
    }

    pub fn get_phase(&self) -> Result<JvmtiPhase> {
        let res = jvmti_call_number_result!(self.internal, jvmtiPhase,
            GetPhase
        );
        Ok(res.into())
    }

    pub fn get_owned_monitor_stack_depth_info(&self, thread: &JThreadID) -> Result<Vec<JMonitorStackDepthInfo>> {
        let mut builder: MutObjectArrayBuilder<jvmtiMonitorStackDepthInfo> = MutObjectArrayBuilder::new();
        let res = jvmti_call_result!( self.internal, GetOwnedMonitorStackDepthInfo,
            thread.into(),
            &mut builder.count,
            &mut builder.items
        );
        jvmti_error_code_to_result(res)?;
        Ok(builder.build())
    }

    pub fn get_extension_events(&self) -> Result<Vec<JExtensionEventInfo>> {
        let mut builder: MutObjectArrayBuilder<jvmtiExtensionEventInfo> = MutObjectArrayBuilder::new();
        let res = jvmti_call_result!( self.internal, GetExtensionEvents,
            &mut builder.count,
            &mut builder.items
        );
        jvmti_error_code_to_result(res)?;
        Ok(builder.build())
    }

    pub fn get_extension_functions(&self) -> Result<Vec<JExtensionFunctionInfo>> {
        let mut builder: MutObjectArrayBuilder<jvmtiExtensionFunctionInfo> = MutObjectArrayBuilder::new();
        let res = jvmti_call_result!( self.internal, GetExtensionFunctions,
            &mut builder.count,
            &mut builder.items
        );
        jvmti_error_code_to_result(res)?;
        Ok(builder.build())
    }

    pub fn get_objects_with_tags(&self, tags: &Vec<jlong>) -> Result<Vec<JTagObject>> {
        if tags.is_empty() {
            return Ok(vec![]);
        }

        let mut count: jint = 0 as jint;
        let mut object_result: *mut jobject = ptr::null_mut();
        let mut tag_result: *mut jlong = ptr::null_mut();

        let res = jvmti_call_result!( self.internal, GetObjectsWithTags,
            tags.len() as jint,
            tags.as_ptr(),
            &mut count,
            &mut object_result,
            &mut tag_result
        );
        jvmti_error_code_to_result(res)?;
        if count == 0 || object_result.is_null() || tag_result.is_null() {
            return Ok(vec![]);
        }
        // TODO: to be optimized
        let objects = slice_raw(object_result, count);
        let tags = slice_raw(tag_result, count);
        let mut jtag_objects: Vec<JTagObject> = Vec::with_capacity(count as usize);
        for idx in 0..(count as usize) {
            let o: jobject = objects[idx..idx + 1][0];
            let t: jlong = tags[idx..idx + 1][0];
            jtag_objects.push(JTagObject::new(o, t))
        }
        Ok(jtag_objects)
    }
}
