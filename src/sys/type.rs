#![allow(non_snake_case, non_camel_case_types)]

use jni_sys::*;
use std::os::raw::c_char;
use std::os::raw::c_uchar;
use std::os::raw::c_void;
use std::option::Option;

use crate::sys::constant::{jvmtiError, jvmtiTimerKind, jvmtiParamKind, jvmtiParamTypes, jvmtiHeapReferenceKind, jvmtiPrimitiveType,
                           jvmtiIterationControl, jvmtiHeapRootKind,
                           jvmtiObjectReferenceKind};
use crate::sys::jvmtienv::jvmtiEnv;
use crate::sys::utils::BitfieldUnit;

pub type jthread = jobject;
pub type jthreadGroup = jobject;
pub type jlocation = jlong;
pub type jmemory = *mut c_uchar;
pub type jniNativeInterface = JNINativeInterface_;

pub type jvmtiCapabilities = _jvmtiCapabilities;
pub type jvmtiEventCallbacks = _jvmtiEventCallbacks;
pub type jrawMonitorID = *mut _jrawMonitorID;
pub type jvmtiThreadInfo = _jvmtiThreadInfo;
pub type jvmtiMonitorStackDepthInfo = _jvmtiMonitorStackDepthInfo;
pub type jvmtiThreadGroupInfo = _jvmtiThreadGroupInfo;
pub type jvmtiFrameInfo = _jvmtiFrameInfo;
pub type jvmtiStackInfo = _jvmtiStackInfo;
pub type jvmtiHeapReferenceInfoField = _jvmtiHeapReferenceInfoField;
pub type jvmtiHeapReferenceInfoArray = _jvmtiHeapReferenceInfoArray;
pub type jvmtiHeapReferenceInfoConstantPool = _jvmtiHeapReferenceInfoConstantPool;
pub type jvmtiHeapReferenceInfoStackLocal = _jvmtiHeapReferenceInfoStackLocal;
pub type jvmtiHeapReferenceInfoJniLocal = _jvmtiHeapReferenceInfoJniLocal;
pub type jvmtiHeapReferenceInfoReserved = _jvmtiHeapReferenceInfoReserved;
pub type jvmtiHeapReferenceInfo = _jvmtiHeapReferenceInfo;
pub type jvmtiHeapCallbacks = _jvmtiHeapCallbacks;
pub type jvmtiClassDefinition = _jvmtiClassDefinition;
pub type jvmtiMonitorUsage = _jvmtiMonitorUsage;
pub type jvmtiLineNumberEntry = _jvmtiLineNumberEntry;
pub type jvmtiLocalVariableEntry = _jvmtiLocalVariableEntry;
pub type jvmtiParamInfo = _jvmtiParamInfo;
pub type jvmtiExtensionFunctionInfo = _jvmtiExtensionFunctionInfo;
pub type jvmtiExtensionEventInfo = _jvmtiExtensionEventInfo;
pub type jvmtiTimerInfo = _jvmtiTimerInfo;
pub type jvmtiAddrLocationMap = _jvmtiAddrLocationMap;


pub type jvmtiStartFunction = Option<
    unsafe extern "C" fn(
        jvmti_env: *mut jvmtiEnv,
        jni_env: *mut JNIEnv,
        arg: *mut c_void,
    ),
>;

pub type jvmtiHeapIterationCallback = Option<
    unsafe extern "C" fn(
        class_tag: jlong,
        size: jlong,
        tag_ptr: *mut jlong,
        length: jint,
        user_data: *mut c_void,
    ) -> jint,
>;

pub type jvmtiHeapReferenceCallback = Option<
    unsafe extern "C" fn(
        reference_kind: jvmtiHeapReferenceKind,
        reference_info: *const jvmtiHeapReferenceInfo,
        class_tag: jlong,
        referrer_class_tag: jlong,
        size: jlong,
        tag_ptr: *mut jlong,
        referrer_tag_ptr: *mut jlong,
        length: jint,
        user_data: *mut c_void,
    ) -> jint,
>;

pub type jvmtiPrimitiveFieldCallback = Option<
    unsafe extern "C" fn(
        kind: jvmtiHeapReferenceKind,
        info: *const jvmtiHeapReferenceInfo,
        object_class_tag: jlong,
        object_tag_ptr: *mut jlong,
        value: jvalue,
        value_type: jvmtiPrimitiveType,
        user_data: *mut c_void,
    ) -> jint,
>;

pub type jvmtiArrayPrimitiveValueCallback = Option<
    unsafe extern "C" fn(
        class_tag: jlong,
        size: jlong,
        tag_ptr: *mut jlong,
        element_count: jint,
        element_type: jvmtiPrimitiveType,
        elements: *const c_void,
        user_data: *mut c_void,
    ) -> jint,
>;

pub type jvmtiStringPrimitiveValueCallback = Option<
    unsafe extern "C" fn(
        class_tag: jlong,
        size: jlong,
        tag_ptr: *mut jlong,
        value: *const jchar,
        value_length: jint,
        user_data: *mut c_void,
    ) -> jint,
>;

pub type jvmtiReservedCallback = Option<unsafe extern "C" fn() -> jint>;

pub type jvmtiHeapObjectCallback = Option<
    unsafe extern "C" fn(
        class_tag: jlong,
        size: jlong,
        tag_ptr: *mut jlong,
        user_data: *mut c_void,
    ) -> jvmtiIterationControl,
>;

pub type jvmtiHeapRootCallback = Option<
    unsafe extern "C" fn(
        root_kind: jvmtiHeapRootKind,
        class_tag: jlong,
        size: jlong,
        tag_ptr: *mut jlong,
        user_data: *mut c_void,
    ) -> jvmtiIterationControl,
>;

pub type jvmtiStackReferenceCallback = Option<
    unsafe extern "C" fn(
        root_kind: jvmtiHeapRootKind,
        class_tag: jlong,
        size: jlong,
        tag_ptr: *mut jlong,
        thread_tag: jlong,
        depth: jint,
        method: jmethodID,
        slot: jint,
        user_data: *mut c_void,
    ) -> jvmtiIterationControl,
>;

pub type jvmtiObjectReferenceCallback = Option<
    unsafe extern "C" fn(
        reference_kind: jvmtiObjectReferenceKind,
        class_tag: jlong,
        size: jlong,
        tag_ptr: *mut jlong,
        referrer_tag: jlong,
        referrer_index: jint,
        user_data: *mut c_void,
    ) -> jvmtiIterationControl,
>;

pub type jvmtiExtensionFunction = Option<
    unsafe extern "C" fn(
        jvmti_env: *mut jvmtiEnv,
        ...
    ) -> jvmtiError
>;

pub type jvmtiExtensionEvent = Option<
    unsafe extern "C" fn(
        jvmti_env: *mut jvmtiEnv,
        ...)
>;

pub type jvmtiEventReserved = Option<unsafe extern "C" fn()>;

pub type jvmtiEventBreakpoint = Option<
    unsafe extern "C" fn(
        jvmti_env: *mut jvmtiEnv,
        jni_env: *mut JNIEnv,
        thread: jthread,
        method: jmethodID,
        location: jlocation,
    ),
>;

pub type jvmtiEventClassFileLoadHook = Option<
    unsafe extern "C" fn(
        jvmti_env: *mut jvmtiEnv,
        jni_env: *mut JNIEnv,
        class_being_redefined: jclass,
        loader: jobject,
        name: *const c_char,
        protection_domain: jobject,
        class_data_len: jint,
        class_data: *const c_uchar,
        new_class_data_len: *mut jint,
        new_class_data: *mut *mut c_uchar,
    ),
>;

pub type jvmtiEventClassLoad = Option<
    unsafe extern "C" fn(
        jvmti_env: *mut jvmtiEnv,
        jni_env: *mut JNIEnv,
        thread: jthread,
        klass: jclass,
    ),
>;

pub type jvmtiEventClassPrepare = Option<
    unsafe extern "C" fn(
        jvmti_env: *mut jvmtiEnv,
        jni_env: *mut JNIEnv,
        thread: jthread,
        klass: jclass,
    ),
>;

pub type jvmtiEventCompiledMethodLoad = Option<
    unsafe extern "C" fn(
        jvmti_env: *mut jvmtiEnv,
        method: jmethodID,
        code_size: jint,
        code_addr: *const c_void,
        map_length: jint,
        map: *const jvmtiAddrLocationMap,
        compile_info: *const c_void,
    ),
>;

pub type jvmtiEventCompiledMethodUnload = Option<
    unsafe extern "C" fn(
        jvmti_env: *mut jvmtiEnv,
        method: jmethodID,
        code_addr: *const c_void,
    ),
>;

pub type jvmtiEventDataDumpRequest = Option<unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv)>;

pub type jvmtiEventDynamicCodeGenerated = Option<
    unsafe extern "C" fn(
        jvmti_env: *mut jvmtiEnv,
        name: *const c_char,
        address: *const c_void,
        length: jint,
    ),
>;

pub type jvmtiEventException = Option<
    unsafe extern "C" fn(
        jvmti_env: *mut jvmtiEnv,
        jni_env: *mut JNIEnv,
        thread: jthread,
        method: jmethodID,
        location: jlocation,
        exception: jobject,
        catch_method: jmethodID,
        catch_location: jlocation,
    ),
>;

pub type jvmtiEventExceptionCatch = Option<
    unsafe extern "C" fn(
        jvmti_env: *mut jvmtiEnv,
        jni_env: *mut JNIEnv,
        thread: jthread,
        method: jmethodID,
        location: jlocation,
        exception: jobject,
    ),
>;

pub type jvmtiEventFieldAccess = Option<
    unsafe extern "C" fn(
        jvmti_env: *mut jvmtiEnv,
        jni_env: *mut JNIEnv,
        thread: jthread,
        method: jmethodID,
        location: jlocation,
        field_klass: jclass,
        object: jobject,
        field: jfieldID,
    ),
>;

pub type jvmtiEventFieldModification = Option<
    unsafe extern "C" fn(
        jvmti_env: *mut jvmtiEnv,
        jni_env: *mut JNIEnv,
        thread: jthread,
        method: jmethodID,
        location: jlocation,
        field_klass: jclass,
        object: jobject,
        field: jfieldID,
        signature_type: c_char,
        new_value: jvalue,
    ),
>;

pub type jvmtiEventFramePop = Option<
    unsafe extern "C" fn(
        jvmti_env: *mut jvmtiEnv,
        jni_env: *mut JNIEnv,
        thread: jthread,
        method: jmethodID,
        was_popped_by_exception: jboolean,
    ),
>;

pub type jvmtiEventGarbageCollectionFinish = Option<unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv)>;

pub type jvmtiEventGarbageCollectionStart = Option<unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv)>;

pub type jvmtiEventMethodEntry = Option<
    unsafe extern "C" fn(
        jvmti_env: *mut jvmtiEnv,
        jni_env: *mut JNIEnv,
        thread: jthread,
        method: jmethodID,
    ),
>;

pub type jvmtiEventMethodExit = Option<
    unsafe extern "C" fn(
        jvmti_env: *mut jvmtiEnv,
        jni_env: *mut JNIEnv,
        thread: jthread,
        method: jmethodID,
        was_popped_by_exception: jboolean,
        return_value: jvalue,
    ),
>;

pub type jvmtiEventMonitorContendedEnter = Option<
    unsafe extern "C" fn(
        jvmti_env: *mut jvmtiEnv,
        jni_env: *mut JNIEnv,
        thread: jthread,
        object: jobject,
    ),
>;

pub type jvmtiEventMonitorContendedEntered = Option<
    unsafe extern "C" fn(
        jvmti_env: *mut jvmtiEnv,
        jni_env: *mut JNIEnv,
        thread: jthread,
        object: jobject,
    ),
>;

pub type jvmtiEventMonitorWait = Option<
    unsafe extern "C" fn(
        jvmti_env: *mut jvmtiEnv,
        jni_env: *mut JNIEnv,
        thread: jthread,
        object: jobject,
        timeout: jlong,
    ),
>;

pub type jvmtiEventMonitorWaited = Option<
    unsafe extern "C" fn(
        jvmti_env: *mut jvmtiEnv,
        jni_env: *mut JNIEnv,
        thread: jthread,
        object: jobject,
        timed_out: jboolean,
    ),
>;

pub type jvmtiEventNativeMethodBind = Option<
    unsafe extern "C" fn(
        jvmti_env: *mut jvmtiEnv,
        jni_env: *mut JNIEnv,
        thread: jthread,
        method: jmethodID,
        address: *mut c_void,
        new_address_ptr: *mut *mut c_void,
    ),
>;

pub type jvmtiEventObjectFree = Option<unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv, tag: jlong)>;

pub type jvmtiEventResourceExhausted = Option<
    unsafe extern "C" fn(
        jvmti_env: *mut jvmtiEnv,
        jni_env: *mut JNIEnv,
        flags: jint,
        reserved: *const c_void,
        description: *const c_char,
    ),
>;

pub type jvmtiEventSingleStep = Option<
    unsafe extern "C" fn(
        jvmti_env: *mut jvmtiEnv,
        jni_env: *mut JNIEnv,
        thread: jthread,
        method: jmethodID,
        location: jlocation,
    ),
>;

pub type jvmtiEventThreadEnd = Option<
    unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv, jni_env: *mut JNIEnv, thread: jthread),
>;

pub type jvmtiEventThreadStart = Option<
    unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv, jni_env: *mut JNIEnv, thread: jthread),
>;

pub type jvmtiEventVMDeath = Option<unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv, jni_env: *mut JNIEnv)>;

pub type jvmtiEventVMInit = Option<
    unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv, jni_env: *mut JNIEnv, thread: jthread),
>;

pub type jvmtiEventVMObjectAlloc = Option<
    unsafe extern "C" fn(
        jvmti_env: *mut jvmtiEnv,
        jni_env: *mut JNIEnv,
        thread: jthread,
        object: jobject,
        object_klass: jclass,
        size: jlong,
    ),
>;

pub type jvmtiEventVMStart =
Option<unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv, jni_env: *mut JNIEnv)>;


#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Copy, Clone)]
pub struct _jvmtiCapabilities {
    pub _bitfield_1: BitfieldUnit<[u8; 16usize], u8>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _jvmtiEventCallbacks {
    pub VMInit: jvmtiEventVMInit,
    pub VMDeath: jvmtiEventVMDeath,
    pub ThreadStart: jvmtiEventThreadStart,
    pub ThreadEnd: jvmtiEventThreadEnd,
    pub ClassFileLoadHook: jvmtiEventClassFileLoadHook,
    pub ClassLoad: jvmtiEventClassLoad,
    pub ClassPrepare: jvmtiEventClassPrepare,
    pub VMStart: jvmtiEventVMStart,
    pub Exception: jvmtiEventException,
    pub ExceptionCatch: jvmtiEventExceptionCatch,
    pub SingleStep: jvmtiEventSingleStep,
    pub FramePop: jvmtiEventFramePop,
    pub Breakpoint: jvmtiEventBreakpoint,
    pub FieldAccess: jvmtiEventFieldAccess,
    pub FieldModification: jvmtiEventFieldModification,
    pub MethodEntry: jvmtiEventMethodEntry,
    pub MethodExit: jvmtiEventMethodExit,
    pub NativeMethodBind: jvmtiEventNativeMethodBind,
    pub CompiledMethodLoad: jvmtiEventCompiledMethodLoad,
    pub CompiledMethodUnload: jvmtiEventCompiledMethodUnload,
    pub DynamicCodeGenerated: jvmtiEventDynamicCodeGenerated,
    pub DataDumpRequest: jvmtiEventDataDumpRequest,
    pub reserved72: jvmtiEventReserved,
    pub MonitorWait: jvmtiEventMonitorWait,
    pub MonitorWaited: jvmtiEventMonitorWaited,
    pub MonitorContendedEnter: jvmtiEventMonitorContendedEnter,
    pub MonitorContendedEntered: jvmtiEventMonitorContendedEntered,
    pub reserved77: jvmtiEventReserved,
    pub reserved78: jvmtiEventReserved,
    pub reserved79: jvmtiEventReserved,
    pub ResourceExhausted: jvmtiEventResourceExhausted,
    pub GarbageCollectionStart: jvmtiEventGarbageCollectionStart,
    pub GarbageCollectionFinish: jvmtiEventGarbageCollectionFinish,
    pub ObjectFree: jvmtiEventObjectFree,
    pub VMObjectAlloc: jvmtiEventVMObjectAlloc,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _jrawMonitorID {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _jvmtiThreadInfo {
    pub name: *mut c_char,
    pub priority: jint,
    pub is_daemon: jboolean,
    pub thread_group: jthreadGroup,
    pub context_class_loader: jobject,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _jvmtiMonitorStackDepthInfo {
    pub monitor: jobject,
    pub stack_depth: jint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _jvmtiThreadGroupInfo {
    pub parent: jthreadGroup,
    pub name: *mut c_char,
    pub max_priority: jint,
    pub is_daemon: jboolean,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _jvmtiFrameInfo {
    pub method: jmethodID,
    pub location: jlocation,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _jvmtiStackInfo {
    pub thread: jthread,
    pub state: jint,
    pub frame_buffer: *mut jvmtiFrameInfo,
    pub frame_count: jint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _jvmtiHeapReferenceInfoField {
    pub index: jint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _jvmtiHeapReferenceInfoArray {
    pub index: jint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _jvmtiHeapReferenceInfoConstantPool {
    pub index: jint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _jvmtiHeapReferenceInfoStackLocal {
    pub thread_tag: jlong,
    pub thread_id: jlong,
    pub depth: jint,
    pub method: jmethodID,
    pub location: jlocation,
    pub slot: jint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _jvmtiHeapReferenceInfoJniLocal {
    pub thread_tag: jlong,
    pub thread_id: jlong,
    pub depth: jint,
    pub method: jmethodID,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _jvmtiHeapReferenceInfoReserved {
    pub reserved1: jlong,
    pub reserved2: jlong,
    pub reserved3: jlong,
    pub reserved4: jlong,
    pub reserved5: jlong,
    pub reserved6: jlong,
    pub reserved7: jlong,
    pub reserved8: jlong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union _jvmtiHeapReferenceInfo {
    pub field: jvmtiHeapReferenceInfoField,
    pub array: jvmtiHeapReferenceInfoArray,
    pub constant_pool: jvmtiHeapReferenceInfoConstantPool,
    pub stack_local: jvmtiHeapReferenceInfoStackLocal,
    pub jni_local: jvmtiHeapReferenceInfoJniLocal,
    pub other: jvmtiHeapReferenceInfoReserved,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _jvmtiHeapCallbacks {
    pub heap_iteration_callback: jvmtiHeapIterationCallback,
    pub heap_reference_callback: jvmtiHeapReferenceCallback,
    pub primitive_field_callback: jvmtiPrimitiveFieldCallback,
    pub array_primitive_value_callback: jvmtiArrayPrimitiveValueCallback,
    pub string_primitive_value_callback: jvmtiStringPrimitiveValueCallback,
    pub reserved5: jvmtiReservedCallback,
    pub reserved6: jvmtiReservedCallback,
    pub reserved7: jvmtiReservedCallback,
    pub reserved8: jvmtiReservedCallback,
    pub reserved9: jvmtiReservedCallback,
    pub reserved10: jvmtiReservedCallback,
    pub reserved11: jvmtiReservedCallback,
    pub reserved12: jvmtiReservedCallback,
    pub reserved13: jvmtiReservedCallback,
    pub reserved14: jvmtiReservedCallback,
    pub reserved15: jvmtiReservedCallback,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _jvmtiClassDefinition {
    pub klass: jclass,
    pub class_byte_count: jint,
    pub class_bytes: *const c_uchar,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _jvmtiMonitorUsage {
    pub owner: jthread,
    pub entry_count: jint,
    pub waiter_count: jint,
    pub waiters: *mut jthread,
    pub notify_waiter_count: jint,
    pub notify_waiters: *mut jthread,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _jvmtiLineNumberEntry {
    pub start_location: jlocation,
    pub line_number: jint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _jvmtiLocalVariableEntry {
    pub start_location: jlocation,
    pub length: jint,
    pub name: *mut c_char,
    pub signature: *mut c_char,
    pub generic_signature: *mut c_char,
    pub slot: jint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _jvmtiParamInfo {
    pub name: *mut c_char,
    pub kind: jvmtiParamKind,
    pub base_type: jvmtiParamTypes,
    pub null_ok: jboolean,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _jvmtiExtensionFunctionInfo {
    pub func: jvmtiExtensionFunction,
    pub id: *mut c_char,
    pub short_description: *mut c_char,
    pub param_count: jint,
    pub params: *mut jvmtiParamInfo,
    pub error_count: jint,
    pub errors: *mut jvmtiError,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _jvmtiExtensionEventInfo {
    pub extension_event_index: jint,
    pub id: *mut c_char,
    pub short_description: *mut c_char,
    pub param_count: jint,
    pub params: *mut jvmtiParamInfo,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _jvmtiTimerInfo {
    pub max_value: jlong,
    pub may_skip_forward: jboolean,
    pub may_skip_backward: jboolean,
    pub kind: jvmtiTimerKind,
    pub reserved1: jlong,
    pub reserved2: jlong,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _jvmtiAddrLocationMap {
    pub start_address: *const c_void,
    pub location: jlocation,
}