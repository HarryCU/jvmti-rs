#![allow(non_snake_case, non_camel_case_types)]

use jni_sys::*;
use std::os::raw::c_char;
use std::os::raw::c_void;
use std::os::raw::c_uchar;
use std::option::Option;

use crate::sys::r#type::*;
use crate::sys::constant::{jvmtiEventMode, jvmtiEvent, jvmtiError, jvmtiHeapObjectFilter, jvmtiJlocationFormat, jvmtiPhase, jvmtiVerboseFlag};

pub type jvmtiEnv = *const jvmtiInterface_;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jvmtiInterface_ {
    pub reserved1: *mut c_void,
    pub SetEventNotificationMode: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            mode: jvmtiEventMode,
            event_type: jvmtiEvent,
            event_thread: jthread,
            ...
        ) -> jvmtiError,
    >,
    pub reserved3: *mut c_void,
    pub GetAllThreads: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            threads_count_ptr: *mut jint,
            threads_ptr: *mut *mut jthread,
        ) -> jvmtiError,
    >,
    pub SuspendThread: Option<
        unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread) -> jvmtiError,
    >,
    pub ResumeThread: Option<
        unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread) -> jvmtiError,
    >,
    pub StopThread: Option<
        unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread, exception: jobject) -> jvmtiError,
    >,
    pub InterruptThread: Option<
        unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread) -> jvmtiError,
    >,
    pub GetThreadInfo: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            thread: jthread,
            info_ptr: *mut jvmtiThreadInfo,
        ) -> jvmtiError,
    >,
    pub GetOwnedMonitorInfo: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            thread: jthread,
            owned_monitor_count_ptr: *mut jint,
            owned_monitors_ptr: *mut *mut jobject,
        ) -> jvmtiError,
    >,
    pub GetCurrentContendedMonitor: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            thread: jthread,
            monitor_ptr: *mut jobject,
        ) -> jvmtiError,
    >,
    pub RunAgentThread: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            thread: jthread,
            proc_: jvmtiStartFunction,
            arg: *const c_void,
            priority: jint,
        ) -> jvmtiError,
    >,
    pub GetTopThreadGroups: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            group_count_ptr: *mut jint,
            groups_ptr: *mut *mut jthreadGroup,
        ) -> jvmtiError,
    >,
    pub GetThreadGroupInfo: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            group: jthreadGroup,
            info_ptr: *mut jvmtiThreadGroupInfo,
        ) -> jvmtiError,
    >,
    pub GetThreadGroupChildren: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            group: jthreadGroup,
            thread_count_ptr: *mut jint,
            threads_ptr: *mut *mut jthread,
            group_count_ptr: *mut jint,
            groups_ptr: *mut *mut jthreadGroup,
        ) -> jvmtiError,
    >,
    pub GetFrameCount: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            thread: jthread,
            count_ptr: *mut jint,
        ) -> jvmtiError,
    >,
    pub GetThreadState: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            thread: jthread,
            thread_state_ptr: *mut jint,
        ) -> jvmtiError,
    >,
    pub GetCurrentThread: Option<
        unsafe extern "C" fn(env: *mut jvmtiEnv, thread_ptr: *mut jthread) -> jvmtiError,
    >,
    pub GetFrameLocation: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            thread: jthread,
            depth: jint,
            method_ptr: *mut jmethodID,
            location_ptr: *mut jlocation,
        ) -> jvmtiError,
    >,
    pub NotifyFramePop: Option<
        unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread, depth: jint) -> jvmtiError,
    >,
    pub GetLocalObject: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            thread: jthread,
            depth: jint,
            slot: jint,
            value_ptr: *mut jobject,
        ) -> jvmtiError,
    >,
    pub GetLocalInt: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            thread: jthread,
            depth: jint,
            slot: jint,
            value_ptr: *mut jint,
        ) -> jvmtiError,
    >,
    pub GetLocalLong: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            thread: jthread,
            depth: jint,
            slot: jint,
            value_ptr: *mut jlong,
        ) -> jvmtiError,
    >,
    pub GetLocalFloat: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            thread: jthread,
            depth: jint,
            slot: jint,
            value_ptr: *mut jfloat,
        ) -> jvmtiError,
    >,
    pub GetLocalDouble: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            thread: jthread,
            depth: jint,
            slot: jint,
            value_ptr: *mut jdouble,
        ) -> jvmtiError,
    >,
    pub SetLocalObject: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            thread: jthread,
            depth: jint,
            slot: jint,
            value: jobject,
        ) -> jvmtiError,
    >,
    pub SetLocalInt: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            thread: jthread,
            depth: jint,
            slot: jint,
            value: jint,
        ) -> jvmtiError,
    >,
    pub SetLocalLong: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            thread: jthread,
            depth: jint,
            slot: jint,
            value: jlong,
        ) -> jvmtiError,
    >,
    pub SetLocalFloat: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            thread: jthread,
            depth: jint,
            slot: jint,
            value: jfloat,
        ) -> jvmtiError,
    >,
    pub SetLocalDouble: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            thread: jthread,
            depth: jint,
            slot: jint,
            value: jdouble,
        ) -> jvmtiError,
    >,
    pub CreateRawMonitor: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            name: *const c_char,
            monitor_ptr: *mut jrawMonitorID,
        ) -> jvmtiError,
    >,
    pub DestroyRawMonitor: Option<
        unsafe extern "C" fn(env: *mut jvmtiEnv, monitor: jrawMonitorID) -> jvmtiError,
    >,
    pub RawMonitorEnter: Option<
        unsafe extern "C" fn(env: *mut jvmtiEnv, monitor: jrawMonitorID) -> jvmtiError,
    >,
    pub RawMonitorExit: Option<
        unsafe extern "C" fn(env: *mut jvmtiEnv, monitor: jrawMonitorID) -> jvmtiError,
    >,
    pub RawMonitorWait: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            monitor: jrawMonitorID,
            millis: jlong,
        ) -> jvmtiError,
    >,
    pub RawMonitorNotify: Option<
        unsafe extern "C" fn(env: *mut jvmtiEnv, monitor: jrawMonitorID) -> jvmtiError,
    >,
    pub RawMonitorNotifyAll: Option<
        unsafe extern "C" fn(env: *mut jvmtiEnv, monitor: jrawMonitorID) -> jvmtiError,
    >,
    pub SetBreakpoint: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            method: jmethodID,
            location: jlocation,
        ) -> jvmtiError,
    >,
    pub ClearBreakpoint: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            method: jmethodID,
            location: jlocation,
        ) -> jvmtiError,
    >,
    pub reserved40: *mut c_void,
    pub SetFieldAccessWatch: Option<
        unsafe extern "C" fn(env: *mut jvmtiEnv, klass: jclass, field: jfieldID) -> jvmtiError,
    >,
    pub ClearFieldAccessWatch: Option<
        unsafe extern "C" fn(env: *mut jvmtiEnv, klass: jclass, field: jfieldID) -> jvmtiError,
    >,
    pub SetFieldModificationWatch: Option<
        unsafe extern "C" fn(env: *mut jvmtiEnv, klass: jclass, field: jfieldID) -> jvmtiError,
    >,
    pub ClearFieldModificationWatch: Option<
        unsafe extern "C" fn(env: *mut jvmtiEnv, klass: jclass, field: jfieldID) -> jvmtiError,
    >,
    pub IsModifiableClass: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            klass: jclass,
            is_modifiable_class_ptr: *mut jboolean,
        ) -> jvmtiError,
    >,
    pub Allocate: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            size: jlong,
            mem_ptr: *mut *mut c_uchar,
        ) -> jvmtiError,
    >,
    pub Deallocate: Option<
        unsafe extern "C" fn(env: *mut jvmtiEnv, mem: *mut c_uchar) -> jvmtiError,
    >,
    pub GetClassSignature: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            klass: jclass,
            signature_ptr: *mut *mut c_char,
            generic_ptr: *mut *mut c_char,
        ) -> jvmtiError,
    >,
    pub GetClassStatus: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            klass: jclass,
            status_ptr: *mut jint,
        ) -> jvmtiError,
    >,
    pub GetSourceFileName: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            klass: jclass,
            source_name_ptr: *mut *mut c_char,
        ) -> jvmtiError,
    >,
    pub GetClassModifiers: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            klass: jclass,
            modifiers_ptr: *mut jint,
        ) -> jvmtiError,
    >,
    pub GetClassMethods: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            klass: jclass,
            method_count_ptr: *mut jint,
            methods_ptr: *mut *mut jmethodID,
        ) -> jvmtiError,
    >,
    pub GetClassFields: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            klass: jclass,
            field_count_ptr: *mut jint,
            fields_ptr: *mut *mut jfieldID,
        ) -> jvmtiError,
    >,
    pub GetImplementedInterfaces: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            klass: jclass,
            interface_count_ptr: *mut jint,
            interfaces_ptr: *mut *mut jclass,
        ) -> jvmtiError,
    >,
    pub IsInterface: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            klass: jclass,
            is_interface_ptr: *mut jboolean,
        ) -> jvmtiError,
    >,
    pub IsArrayClass: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            klass: jclass,
            is_array_class_ptr: *mut jboolean,
        ) -> jvmtiError,
    >,
    pub GetClassLoader: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            klass: jclass,
            classloader_ptr: *mut jobject,
        ) -> jvmtiError,
    >,
    pub GetObjectHashCode: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            object: jobject,
            hash_code_ptr: *mut jint,
        ) -> jvmtiError,
    >,
    pub GetObjectMonitorUsage: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            object: jobject,
            info_ptr: *mut jvmtiMonitorUsage,
        ) -> jvmtiError,
    >,
    pub GetFieldName: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            klass: jclass,
            field: jfieldID,
            name_ptr: *mut *mut c_char,
            signature_ptr: *mut *mut c_char,
            generic_ptr: *mut *mut c_char,
        ) -> jvmtiError,
    >,
    pub GetFieldDeclaringClass: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            klass: jclass,
            field: jfieldID,
            declaring_class_ptr: *mut jclass,
        ) -> jvmtiError,
    >,
    pub GetFieldModifiers: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            klass: jclass,
            field: jfieldID,
            modifiers_ptr: *mut jint,
        ) -> jvmtiError,
    >,
    pub IsFieldSynthetic: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            klass: jclass,
            field: jfieldID,
            is_synthetic_ptr: *mut jboolean,
        ) -> jvmtiError,
    >,
    pub GetMethodName: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            method: jmethodID,
            name_ptr: *mut *mut c_char,
            signature_ptr: *mut *mut c_char,
            generic_ptr: *mut *mut c_char,
        ) -> jvmtiError,
    >,
    pub GetMethodDeclaringClass: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            method: jmethodID,
            declaring_class_ptr: *mut jclass,
        ) -> jvmtiError,
    >,
    pub GetMethodModifiers: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            method: jmethodID,
            modifiers_ptr: *mut jint,
        ) -> jvmtiError,
    >,
    pub reserved67: *mut c_void,
    pub GetMaxLocals: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            method: jmethodID,
            max_ptr: *mut jint,
        ) -> jvmtiError,
    >,
    pub GetArgumentsSize: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            method: jmethodID,
            size_ptr: *mut jint,
        ) -> jvmtiError,
    >,
    pub GetLineNumberTable: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            method: jmethodID,
            entry_count_ptr: *mut jint,
            table_ptr: *mut *mut jvmtiLineNumberEntry,
        ) -> jvmtiError,
    >,
    pub GetMethodLocation: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            method: jmethodID,
            start_location_ptr: *mut jlocation,
            end_location_ptr: *mut jlocation,
        ) -> jvmtiError,
    >,
    pub GetLocalVariableTable: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            method: jmethodID,
            entry_count_ptr: *mut jint,
            table_ptr: *mut *mut jvmtiLocalVariableEntry,
        ) -> jvmtiError,
    >,
    pub SetNativeMethodPrefix: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            prefix: *const c_char,
        ) -> jvmtiError,
    >,
    pub SetNativeMethodPrefixes: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            prefix_count: jint,
            prefixes: *mut *mut c_char,
        ) -> jvmtiError,
    >,
    pub GetBytecodes: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            method: jmethodID,
            bytecode_count_ptr: *mut jint,
            bytecodes_ptr: *mut *mut c_uchar,
        ) -> jvmtiError,
    >,
    pub IsMethodNative: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            method: jmethodID,
            is_native_ptr: *mut jboolean,
        ) -> jvmtiError,
    >,
    pub IsMethodSynthetic: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            method: jmethodID,
            is_synthetic_ptr: *mut jboolean,
        ) -> jvmtiError,
    >,
    pub GetLoadedClasses: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            class_count_ptr: *mut jint,
            classes_ptr: *mut *mut jclass,
        ) -> jvmtiError,
    >,
    pub GetClassLoaderClasses: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            initiating_loader: jobject,
            class_count_ptr: *mut jint,
            classes_ptr: *mut *mut jclass,
        ) -> jvmtiError,
    >,
    pub PopFrame: Option<
        unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread) -> jvmtiError,
    >,
    pub ForceEarlyReturnObject: Option<
        unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread, value: jobject) -> jvmtiError,
    >,
    pub ForceEarlyReturnInt: Option<
        unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread, value: jint) -> jvmtiError,
    >,
    pub ForceEarlyReturnLong: Option<
        unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread, value: jlong) -> jvmtiError,
    >,
    pub ForceEarlyReturnFloat: Option<
        unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread, value: jfloat) -> jvmtiError,
    >,
    pub ForceEarlyReturnDouble: Option<
        unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread, value: jdouble) -> jvmtiError,
    >,
    pub ForceEarlyReturnVoid: Option<
        unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread) -> jvmtiError,
    >,
    pub RedefineClasses: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            class_count: jint,
            class_definitions: *const jvmtiClassDefinition,
        ) -> jvmtiError,
    >,
    pub GetVersionNumber: Option<
        unsafe extern "C" fn(env: *mut jvmtiEnv, version_ptr: *mut jint) -> jvmtiError,
    >,
    pub GetCapabilities: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            capabilities_ptr: *mut jvmtiCapabilities,
        ) -> jvmtiError,
    >,
    pub GetSourceDebugExtension: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            klass: jclass,
            source_debug_extension_ptr: *mut *mut c_char,
        ) -> jvmtiError,
    >,
    pub IsMethodObsolete: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            method: jmethodID,
            is_obsolete_ptr: *mut jboolean,
        ) -> jvmtiError,
    >,
    pub SuspendThreadList: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            request_count: jint,
            request_list: *const jthread,
            results: *mut jvmtiError,
        ) -> jvmtiError,
    >,
    pub ResumeThreadList: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            request_count: jint,
            request_list: *const jthread,
            results: *mut jvmtiError,
        ) -> jvmtiError,
    >,
    pub reserved94: *mut c_void,
    pub reserved95: *mut c_void,
    pub reserved96: *mut c_void,
    pub reserved97: *mut c_void,
    pub reserved98: *mut c_void,
    pub reserved99: *mut c_void,
    pub GetAllStackTraces: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            max_frame_count: jint,
            stack_info_ptr: *mut *mut jvmtiStackInfo,
            thread_count_ptr: *mut jint,
        ) -> jvmtiError,
    >,
    pub GetThreadListStackTraces: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            thread_count: jint,
            thread_list: *const jthread,
            max_frame_count: jint,
            stack_info_ptr: *mut *mut jvmtiStackInfo,
        ) -> jvmtiError,
    >,
    pub GetThreadLocalStorage: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            thread: jthread,
            data_ptr: *mut *mut c_void,
        ) -> jvmtiError,
    >,
    pub SetThreadLocalStorage: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            thread: jthread,
            data: *const c_void,
        ) -> jvmtiError,
    >,
    pub GetStackTrace: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            thread: jthread,
            start_depth: jint,
            max_frame_count: jint,
            frame_buffer: *mut jvmtiFrameInfo,
            count_ptr: *mut jint,
        ) -> jvmtiError,
    >,
    pub reserved105: *mut c_void,
    pub GetTag: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            object: jobject,
            tag_ptr: *mut jlong,
        ) -> jvmtiError,
    >,
    pub SetTag: Option<
        unsafe extern "C" fn(env: *mut jvmtiEnv, object: jobject, tag: jlong) -> jvmtiError,
    >,
    pub ForceGarbageCollection:
    Option<unsafe extern "C" fn(env: *mut jvmtiEnv) -> jvmtiError>,
    pub IterateOverObjectsReachableFromObject: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            object: jobject,
            object_reference_callback: jvmtiObjectReferenceCallback,
            user_data: *const c_void,
        ) -> jvmtiError,
    >,
    pub IterateOverReachableObjects: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            heap_root_callback: jvmtiHeapRootCallback,
            stack_ref_callback: jvmtiStackReferenceCallback,
            object_ref_callback: jvmtiObjectReferenceCallback,
            user_data: *const c_void,
        ) -> jvmtiError,
    >,
    pub IterateOverHeap: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            object_filter: jvmtiHeapObjectFilter,
            heap_object_callback: jvmtiHeapObjectCallback,
            user_data: *const c_void,
        ) -> jvmtiError,
    >,
    pub IterateOverInstancesOfClass: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            klass: jclass,
            object_filter: jvmtiHeapObjectFilter,
            heap_object_callback: jvmtiHeapObjectCallback,
            user_data: *const c_void,
        ) -> jvmtiError,
    >,
    pub reserved113: *mut c_void,
    pub GetObjectsWithTags: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            tag_count: jint,
            tags: *const jlong,
            count_ptr: *mut jint,
            object_result_ptr: *mut *mut jobject,
            tag_result_ptr: *mut *mut jlong,
        ) -> jvmtiError,
    >,
    pub FollowReferences: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            heap_filter: jint,
            klass: jclass,
            initial_object: jobject,
            callbacks: *const jvmtiHeapCallbacks,
            user_data: *const c_void,
        ) -> jvmtiError,
    >,
    pub IterateThroughHeap: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            heap_filter: jint,
            klass: jclass,
            callbacks: *const jvmtiHeapCallbacks,
            user_data: *const c_void,
        ) -> jvmtiError,
    >,
    pub reserved117: *mut c_void,
    pub reserved118: *mut c_void,
    pub reserved119: *mut c_void,
    pub SetJNIFunctionTable: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            function_table: *const jniNativeInterface,
        ) -> jvmtiError,
    >,
    pub GetJNIFunctionTable: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            function_table: *mut *mut jniNativeInterface,
        ) -> jvmtiError,
    >,
    pub SetEventCallbacks: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            callbacks: *const jvmtiEventCallbacks,
            size_of_callbacks: jint,
        ) -> jvmtiError
    >,
    pub GenerateEvents: Option<
        unsafe extern "C" fn(env: *mut jvmtiEnv, event_type: jvmtiEvent) -> jvmtiError,
    >,
    pub GetExtensionFunctions: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            extension_count_ptr: *mut jint,
            extensions: *mut *mut jvmtiExtensionFunctionInfo,
        ) -> jvmtiError,
    >,
    pub GetExtensionEvents: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            extension_count_ptr: *mut jint,
            extensions: *mut *mut jvmtiExtensionEventInfo,
        ) -> jvmtiError,
    >,
    pub SetExtensionEventCallback: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            extension_event_index: jint,
            callback: jvmtiExtensionEvent,
        ) -> jvmtiError,
    >,
    pub DisposeEnvironment:
    Option<unsafe extern "C" fn(env: *mut jvmtiEnv) -> jvmtiError>,
    pub GetErrorName: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            error: jvmtiError,
            name_ptr: *mut *mut c_char,
        ) -> jvmtiError,
    >,
    pub GetJLocationFormat: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            format_ptr: *mut jvmtiJlocationFormat,
        ) -> jvmtiError,
    >,
    pub GetSystemProperties: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            count_ptr: *mut jint,
            property_ptr: *mut *mut *mut c_char,
        ) -> jvmtiError,
    >,
    pub GetSystemProperty: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            property: *const c_char,
            value_ptr: *mut *mut c_char,
        ) -> jvmtiError,
    >,
    pub SetSystemProperty: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            property: *const c_char,
            value: *const c_char,
        ) -> jvmtiError,
    >,
    pub GetPhase: Option<
        unsafe extern "C" fn(env: *mut jvmtiEnv, phase_ptr: *mut jvmtiPhase) -> jvmtiError,
    >,
    pub GetCurrentThreadCpuTimerInfo: Option<
        unsafe extern "C" fn(env: *mut jvmtiEnv, info_ptr: *mut jvmtiTimerInfo) -> jvmtiError,
    >,
    pub GetCurrentThreadCpuTime: Option<
        unsafe extern "C" fn(env: *mut jvmtiEnv, nanos_ptr: *mut jlong) -> jvmtiError,
    >,
    pub GetThreadCpuTimerInfo: Option<
        unsafe extern "C" fn(env: *mut jvmtiEnv, info_ptr: *mut jvmtiTimerInfo) -> jvmtiError,
    >,
    pub GetThreadCpuTime: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            thread: jthread,
            nanos_ptr: *mut jlong,
        ) -> jvmtiError,
    >,
    pub GetTimerInfo: Option<
        unsafe extern "C" fn(env: *mut jvmtiEnv, info_ptr: *mut jvmtiTimerInfo) -> jvmtiError,
    >,
    pub GetTime: Option<
        unsafe extern "C" fn(env: *mut jvmtiEnv, nanos_ptr: *mut jlong) -> jvmtiError,
    >,
    pub GetPotentialCapabilities: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            capabilities_ptr: *mut jvmtiCapabilities,
        ) -> jvmtiError,
    >,
    pub reserved141: *mut c_void,
    pub AddCapabilities: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            capabilities_ptr: *const jvmtiCapabilities,
        ) -> jvmtiError,
    >,
    pub RelinquishCapabilities: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            capabilities_ptr: *const jvmtiCapabilities,
        ) -> jvmtiError,
    >,
    pub GetAvailableProcessors: Option<
        unsafe extern "C" fn(env: *mut jvmtiEnv, processor_count_ptr: *mut jint) -> jvmtiError,
    >,
    pub GetClassVersionNumbers: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            klass: jclass,
            minor_version_ptr: *mut jint,
            major_version_ptr: *mut jint,
        ) -> jvmtiError,
    >,
    pub GetConstantPool: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            klass: jclass,
            constant_pool_count_ptr: *mut jint,
            constant_pool_byte_count_ptr: *mut jint,
            constant_pool_bytes_ptr: *mut *mut c_uchar,
        ) -> jvmtiError,
    >,
    pub GetEnvironmentLocalStorage: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            data_ptr: *mut *mut c_void,
        ) -> jvmtiError,
    >,
    pub SetEnvironmentLocalStorage: Option<
        unsafe extern "C" fn(env: *mut jvmtiEnv, data: *const c_void) -> jvmtiError,
    >,
    pub AddToBootstrapClassLoaderSearch: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            segment: *const c_char,
        ) -> jvmtiError,
    >,
    pub SetVerboseFlag: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            flag: jvmtiVerboseFlag,
            value: jboolean,
        ) -> jvmtiError,
    >,
    pub AddToSystemClassLoaderSearch: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            segment: *const c_char,
        ) -> jvmtiError,
    >,
    pub RetransformClasses: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            class_count: jint,
            classes: *const jclass,
        ) -> jvmtiError,
    >,
    pub GetOwnedMonitorStackDepthInfo: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            thread: jthread,
            monitor_info_count_ptr: *mut jint,
            monitor_info_ptr: *mut *mut jvmtiMonitorStackDepthInfo,
        ) -> jvmtiError,
    >,
    pub GetObjectSize: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            object: jobject,
            size_ptr: *mut jlong,
        ) -> jvmtiError,
    >,
    pub GetLocalInstance: Option<
        unsafe extern "C" fn(
            env: *mut jvmtiEnv,
            thread: jthread,
            depth: jint,
            value_ptr: *mut jobject,
        ) -> jvmtiError,
    >,
}