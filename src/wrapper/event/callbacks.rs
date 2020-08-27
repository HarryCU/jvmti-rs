use log::error;
use std::os::raw::{c_void, c_char, c_uchar};
use std::panic;

use crate::sys;
use crate::sys::{jthread, jobject, jclass, jlong, jmethodID, jlocation, jint, jboolean, jvmtiAddrLocationMap, jvalue, jfieldID, jmemory};
use crate::wrapper::*;

static mut EVENT_CALL_TABLE: EventHandlers = empty_event_handlers!();

macro_rules! global_event {
    () => {{
        unsafe {
            EVENT_CALL_TABLE
        }
    }};
}

macro_rules! define_event {
    ($event_name:ident, $event_field:ident, $event_type:ty) => {
        pub fn $event_name(event: $event_type) { unsafe { EVENT_CALL_TABLE.$event_field = event } }
    };
}

pub struct EventAdjuster();

impl EventAdjuster {
    fn global() -> EventHandlers {
        return global_event!();
    }

    define_event!(on_breakpoint, breakpoint, EventBreakpointFn);
    define_event!(on_class_file_load_hook, class_file_load_hook, EventClassFileLoadHookFn);
    define_event!(on_class_load, class_load, EventClassLoadFn);
    define_event!(on_class_prepare, class_prepare, EventClassPrepareFn);
    define_event!(on_compiled_method_load, compiled_method_load, CompiledMethodLoadFn);
    define_event!(on_compiled_method_unload, compiled_method_unload, CompiledMethodUnloadFn);
    define_event!(on_data_dump_request, data_dump_request, DataDumpRequestFn);
    define_event!(on_dynamic_code_generated, dynamic_code_generated, DynamicCodeGeneratedFn);
    define_event!(on_exception, exception, ExceptionFn);
    define_event!(on_exception_catch, exception_catch, ExceptionCatchFn);
    define_event!(on_field_access, field_access, FieldAccessFn);
    define_event!(on_field_modification, field_modification, FieldModificationFn);
    define_event!(on_frame_pop, frame_pop, FramePopFn);
    define_event!(on_garbage_collection_finish, garbage_collection_finish, GarbageCollectionFinishFn);
    define_event!(on_garbage_collection_start, garbage_collection_start, GarbageCollectionStartFn);
    define_event!(on_method_entry, method_entry, MethodEntryFn);
    define_event!(on_method_exit, method_exit, MethodExitFn);
    define_event!(on_monitor_contended_enter, monitor_contended_enter, MonitorContendedEnterFn);
    define_event!(on_monitor_contended_entered, monitor_contended_entered, MonitorContendedEnteredFn);
    define_event!(on_monitor_wait, monitor_wait, MonitorWaitFn);
    define_event!(on_monitor_waited, monitor_waited, MonitorWaitedFn);
    define_event!(on_native_method_bind, native_method_bind, NativeMethodBindFn);
    define_event!(on_object_free, object_free, ObjectFreeFn);
    define_event!(on_resource_exhausted, resource_exhausted, ResourceExhaustedFn);
    define_event!(on_single_step, single_step, SingleStepFn);
    define_event!(on_thread_end, thread_end, ThreadEndFn);
    define_event!(on_thread_start, thread_start, ThreadStartFn);
    define_event!(on_vm_death, vm_death, VmDeathFn);
    define_event!(on_vm_init, vm_init, VmInitFn);
    define_event!(on_vm_object_alloc, vm_object_alloc, VmObjectAllocFn);
    define_event!(on_vm_start, vm_start, VmStartFn);
}

#[no_mangle]
pub extern "C" fn jvmti_event_breakpoint_handler(jvmti_env: *mut sys::JVMTIEnv,
                                                 jni_env: *mut sys::JNIEnv,
                                                 thread: jthread,
                                                 method: jmethodID,
                                                 location: jlocation) {
    unsafe {
        let jvmti = jvmti!(jvmti_env);
        let jni = jni!(jni_env);
        let event = BreakpointEvent {
            jvmti: &jvmti,
            jni: &jni,
            thread: thread.into(),
            method: method.into(),
            location,
        };
        event_call!("jvmti_event_breakpoint_handler", breakpoint, event)
    }
}

#[no_mangle]
pub extern "C" fn jvmti_event_class_file_load_hook_handler(jvmti_env: *mut sys::JVMTIEnv,
                                                           jni_env: *mut sys::JNIEnv,
                                                           class_being_redefined: jclass,
                                                           loader: jobject,
                                                           name: *const c_char,
                                                           protection_domain: jobject,
                                                           class_data_len: jint,
                                                           class_data: *const c_uchar,
                                                           new_class_data_len: *mut jint,
                                                           new_class_data: *mut jmemory) {
    unsafe {
        let jvmti = jvmti!(jvmti_env);
        let jni = jni!(jni_env);
        let event = ClassFileLoadHookEvent {
            jvmti: &jvmti,
            jni: &jni,
            class_being_redefined: class_being_redefined.into(),
            loader: loader.into(),
            name: stringify(name),
            protection_domain: protection_domain.into(),
            class_data_len,
            class_data,
            new_class_data_len,
            new_class_data,
        };
        event_call!("jvmti_event_class_file_load_hook_handler", class_file_load_hook, event)
    }
}

#[no_mangle]
pub extern "C" fn jvmti_event_class_load_handler(jvmti_env: *mut sys::JVMTIEnv,
                                                 jni_env: *mut sys::JNIEnv,
                                                 thread: jthread,
                                                 klass: jclass) {
    unsafe {
        let jvmti = jvmti!(jvmti_env);
        let jni = jni!(jni_env);
        let event = ClassLoadEvent {
            jvmti: &jvmti,
            jni: &jni,
            thread: thread.into(),
            klass: klass.into(),
        };
        event_call!("jvmti_event_class_load_handler", class_load, event)
    }
}

#[no_mangle]
pub extern "C" fn jvmti_event_class_prepare_handler(jvmti_env: *mut sys::JVMTIEnv,
                                                    jni_env: *mut sys::JNIEnv,
                                                    thread: jthread,
                                                    klass: jclass) {
    unsafe {
        let jvmti = jvmti!(jvmti_env);
        let jni = jni!(jni_env);
        let event = ClassPrepareEvent {
            jvmti: &jvmti,
            jni: &jni,
            thread: thread.into(),
            klass: klass.into(),
        };
        event_call!("jvmti_event_class_prepare_handler", class_prepare, event)
    }
}

#[no_mangle]
pub extern "C" fn jvmti_event_compiled_method_load_handler(jvmti_env: *mut sys::JVMTIEnv,
                                                           method: jmethodID,
                                                           code_size: jint,
                                                           code_addr: *const c_void,
                                                           map_length: jint,
                                                           map: *const jvmtiAddrLocationMap,
                                                           compile_info: *const c_void) {
    unsafe {
        let jvmti = jvmti!(jvmti_env);
        let builder: ObjectArrayBuilder<jvmtiAddrLocationMap> = ObjectArrayBuilder::new(map_length, map);
        let compiled_method_load_records = parse_compiled_method_load_record(compile_info);
        let event = CompiledMethodLoadEvent {
            jvmti: &jvmti,
            method: method.into(),
            code_size,
            code_addr,
            addr_location_map: builder.build(),
            compiled_records: &compiled_method_load_records,
        };
        event_call!("jvmti_event_compiled_method_load_handler", compiled_method_load, event)
    }
}

#[no_mangle]
pub extern "C" fn jvmti_event_compiled_method_unload_handler(jvmti_env: *mut sys::JVMTIEnv,
                                                             method: jmethodID,
                                                             code_addr: *const c_void) {
    unsafe {
        let jvmti = jvmti!(jvmti_env);
        let event = CompiledMethodUnloadEvent {
            jvmti: &jvmti,
            method: method.into(),
            code_addr,
        };
        event_call!("jvmti_event_compiled_method_unload_handler", compiled_method_unload, event)
    }
}

#[no_mangle]
pub extern "C" fn jvmti_event_data_dump_request_handler(jvmti_env: *mut sys::JVMTIEnv) {
    unsafe {
        let jvmti = jvmti!(jvmti_env);
        let event = DataDumpRequestEvent {
            jvmti: &jvmti,
        };
        event_call!("jvmti_event_data_dump_request_handler", data_dump_request, event)
    }
}

#[no_mangle]
pub extern "C" fn jvmti_event_dynamic_code_generated_handler(jvmti_env: *mut sys::JVMTIEnv,
                                                             name: *const c_char,
                                                             address: *const c_void,
                                                             length: jint) {
    unsafe {
        let jvmti = jvmti!(jvmti_env);
        let event = DynamicCodeGeneratedEvent {
            jvmti: &jvmti,
            name: stringify(name),
            address,
            length,
        };
        event_call!("jvmti_event_dynamic_code_generated_handler", dynamic_code_generated, event)
    }
}

#[no_mangle]
pub extern "C" fn jvmti_event_exception_handler(jvmti_env: *mut sys::JVMTIEnv,
                                                jni_env: *mut sys::JNIEnv,
                                                thread: jthread,
                                                method: jmethodID,
                                                location: jlocation,
                                                exception: jobject,
                                                catch_method: jmethodID,
                                                catch_location: jlocation) {
    unsafe {
        let jvmti = jvmti!(jvmti_env);
        let jni = jni!(jni_env);
        let event = ExceptionEvent {
            jvmti: &jvmti,
            jni: &jni,
            thread: thread.into(),
            method: method.into(),
            location,
            exception: exception.into(),
            catch_method: catch_method.into(),
            catch_location: catch_location.into(),
        };
        event_call!("jvmti_event_exception_handler", exception, event)
    }
}

#[no_mangle]
pub extern "C" fn jvmti_event_exception_catch_handler(jvmti_env: *mut sys::JVMTIEnv,
                                                      jni_env: *mut sys::JNIEnv,
                                                      thread: jthread,
                                                      method: jmethodID,
                                                      location: jlocation,
                                                      exception: jobject) {
    unsafe {
        let jvmti = jvmti!(jvmti_env);
        let jni = jni!(jni_env);
        let event = ExceptionCatchEvent {
            jvmti: &jvmti,
            jni: &jni,
            thread: thread.into(),
            method: method.into(),
            location,
            exception: exception.into(),
        };
        event_call!("jvmti_event_exception_catch_handler", exception_catch, event)
    }
}

#[no_mangle]
pub extern "C" fn jvmti_event_field_access_handler(jvmti_env: *mut sys::JVMTIEnv,
                                                   jni_env: *mut sys::JNIEnv,
                                                   thread: jthread,
                                                   method: jmethodID,
                                                   location: jlocation,
                                                   field_klass: jclass,
                                                   object: jobject,
                                                   field: jfieldID) {
    unsafe {
        let jvmti = jvmti!(jvmti_env);
        let jni = jni!(jni_env);
        let event = FieldAccessEvent {
            jvmti: &jvmti,
            jni: &jni,
            thread: thread.into(),
            method: method.into(),
            location,
            field_klass: field_klass.into(),
            object: object.into(),
            field: field.into(),
        };
        event_call!("jvmti_event_field_access_handler", field_access, event)
    }
}

#[no_mangle]
pub extern "C" fn jvmti_event_field_modification_handler(jvmti_env: *mut sys::JVMTIEnv,
                                                         jni_env: *mut sys::JNIEnv,
                                                         thread: jthread,
                                                         method: jmethodID,
                                                         location: jlocation,
                                                         field_klass: jclass,
                                                         object: jobject,
                                                         field: jfieldID,
                                                         signature_type: c_char,
                                                         new_value: jvalue) {
    unsafe {
        let jvmti = jvmti!(jvmti_env);
        let jni = jni!(jni_env);
        let event = FieldModificationEvent {
            jvmti: &jvmti,
            jni: &jni,
            thread: thread.into(),
            method: method.into(),
            location,
            field_klass: field_klass.into(),
            object: object.into(),
            field: field.into(),
            signature_type,
            new_value,
        };
        event_call!("jvmti_event_field_modification_handler", field_modification, event)
    }
}

#[no_mangle]
pub extern "C" fn jvmti_event_frame_pop_handler(jvmti_env: *mut sys::JVMTIEnv,
                                                jni_env: *mut sys::JNIEnv,
                                                thread: jthread,
                                                method: jmethodID,
                                                was_popped_by_exception: jboolean) {
    unsafe {
        let jvmti = jvmti!(jvmti_env);
        let jni = jni!(jni_env);
        let event = FramePopEvent {
            jvmti: &jvmti,
            jni: &jni,
            thread: thread.into(),
            method: method.into(),
            was_popped_by_exception: to_bool(was_popped_by_exception),
        };
        event_call!("jvmti_event_frame_pop_handler", frame_pop, event)
    }
}

#[no_mangle]
pub extern "C" fn jvmti_event_garbage_collection_finish_handler(jvmti_env: *mut sys::JVMTIEnv) {
    unsafe {
        let jvmti = jvmti!(jvmti_env);
        let event = GarbageCollectionFinishEvent {
            jvmti: &jvmti,
        };
        event_call!("jvmti_event_garbage_collection_finish_handler", garbage_collection_finish, event)
    }
}

#[no_mangle]
pub extern "C" fn jvmti_event_garbage_collection_start_handler(jvmti_env: *mut sys::JVMTIEnv) {
    unsafe {
        let jvmti = jvmti!(jvmti_env);
        let event = GarbageCollectionStartEvent {
            jvmti: &jvmti,
        };
        event_call!("jvmti_event_garbage_collection_start_handler", garbage_collection_start, event)
    }
}

#[no_mangle]
pub extern "C" fn jvmti_event_method_entry_handler(jvmti_env: *mut sys::JVMTIEnv,
                                                   jni_env: *mut sys::JNIEnv,
                                                   thread: jthread,
                                                   method: jmethodID) {
    unsafe {
        let jvmti = jvmti!(jvmti_env);
        let jni = jni!(jni_env);
        let event = MethodEntryEvent {
            jvmti: &jvmti,
            jni: &jni,
            thread: thread.into(),
            method: method.into(),
        };
        event_call!("jvmti_event_method_entry_handler", method_entry, event)
    }
}

#[no_mangle]
pub extern "C" fn jvmti_event_method_exit_handler(jvmti_env: *mut sys::JVMTIEnv,
                                                  jni_env: *mut sys::JNIEnv,
                                                  thread: jthread,
                                                  method: jmethodID,
                                                  was_popped_by_exception: jboolean,
                                                  return_value: jvalue) {
    unsafe {
        let jvmti = jvmti!(jvmti_env);
        let jni = jni!(jni_env);
        let event = MethodExitEvent {
            jvmti: &jvmti,
            jni: &jni,
            thread: thread.into(),
            method: method.into(),
            was_popped_by_exception: to_bool(was_popped_by_exception),
            return_value,
        };
        event_call!("jvmti_event_method_exit_handler", method_exit, event)
    }
}

#[no_mangle]
pub extern "C" fn jvmti_event_monitor_contended_enter_handler(jvmti_env: *mut sys::JVMTIEnv,
                                                              jni_env: *mut sys::JNIEnv,
                                                              thread: jthread,
                                                              object: jobject) {
    unsafe {
        let jvmti = jvmti!(jvmti_env);
        let jni = jni!(jni_env);
        let event = MonitorContendedEnterEvent {
            jvmti: &jvmti,
            jni: &jni,
            thread: thread.into(),
            object: object.into(),
        };
        event_call!("jvmti_event_monitor_contended_enter_handler", monitor_contended_enter, event)
    }
}

#[no_mangle]
pub extern "C" fn jvmti_event_monitor_contended_entered_handler(jvmti_env: *mut sys::JVMTIEnv,
                                                                jni_env: *mut sys::JNIEnv,
                                                                thread: jthread,
                                                                object: jobject) {
    unsafe {
        let jvmti = jvmti!(jvmti_env);
        let jni = jni!(jni_env);
        let event = MonitorContendedEnteredEvent {
            jvmti: &jvmti,
            jni: &jni,
            thread: thread.into(),
            object: object.into(),
        };
        event_call!("jvmti_event_monitor_contended_entered_handler", monitor_contended_entered, event)
    }
}

#[no_mangle]
pub extern "C" fn jvmti_event_monitor_wait_handler(jvmti_env: *mut sys::JVMTIEnv,
                                                   jni_env: *mut sys::JNIEnv,
                                                   thread: jthread,
                                                   object: jobject,
                                                   timeout: jlong) {
    unsafe {
        let jvmti = jvmti!(jvmti_env);
        let jni = jni!(jni_env);
        let event = MonitorWaitEvent {
            jvmti: &jvmti,
            jni: &jni,
            thread: thread.into(),
            object: object.into(),
            timeout,
        };
        event_call!("jvmti_event_monitor_wait_handler", monitor_wait, event)
    }
}

#[no_mangle]
pub extern "C" fn jvmti_event_monitor_waited_handler(jvmti_env: *mut sys::JVMTIEnv,
                                                     jni_env: *mut sys::JNIEnv,
                                                     thread: jthread,
                                                     object: jobject,
                                                     timed_out: jboolean) {
    unsafe {
        let jvmti = jvmti!(jvmti_env);
        let jni = jni!(jni_env);
        let event = MonitorWaitedEvent {
            jvmti: &jvmti,
            jni: &jni,
            thread: thread.into(),
            object: object.into(),
            timed_out: to_bool(timed_out),
        };
        event_call!("jvmti_event_monitor_waited_handler", monitor_waited, event)
    }
}

#[no_mangle]
pub extern "C" fn jvmti_event_native_method_bind_handler(jvmti_env: *mut sys::JVMTIEnv,
                                                         jni_env: *mut sys::JNIEnv,
                                                         thread: jthread,
                                                         method: jmethodID,
                                                         address: *mut c_void,
                                                         new_address_ptr: *mut *mut c_void) {
    unsafe {
        let jvmti = jvmti!(jvmti_env);
        let jni = jni!(jni_env);
        let event = NativeMethodBindEvent {
            jvmti: &jvmti,
            jni: &jni,
            thread: thread.into(),
            method: method.into(),
            address,
            new_address_ptr,
        };
        event_call!("jvmti_event_native_method_bind_handler", native_method_bind, event)
    }
}

#[no_mangle]
pub extern "C" fn jvmti_event_object_free_handler(jvmti_env: *mut sys::JVMTIEnv, tag: jlong) {
    unsafe {
        let jvmti = jvmti!(jvmti_env);
        let event = ObjectFreeEvent {
            jvmti: &jvmti,
            tag,
        };
        event_call!("jvmti_event_object_free_handler", object_free, event)
    }
}

#[no_mangle]
pub extern "C" fn jvmti_event_resource_exhausted_handler(jvmti_env: *mut sys::JVMTIEnv,
                                                         jni_env: *mut sys::JNIEnv,
                                                         flags: jint,
                                                         reserved: *const c_void,
                                                         description: *const c_char) {
    unsafe {
        let jvmti = jvmti!(jvmti_env);
        let jni = jni!(jni_env);
        let event = ResourceExhaustedEvent {
            jvmti: &jvmti,
            jni: &jni,
            flags,
            reserved,
            description: stringify(description),
        };
        event_call!("jvmti_event_resource_exhausted_handler", resource_exhausted, event)
    }
}

#[no_mangle]
pub extern "C" fn jvmti_event_single_step_handler(jvmti_env: *mut sys::JVMTIEnv,
                                                  jni_env: *mut sys::JNIEnv,
                                                  thread: jthread,
                                                  method: jmethodID,
                                                  location: jlocation) {
    unsafe {
        let jvmti = jvmti!(jvmti_env);
        let jni = jni!(jni_env);
        let event = SingleStepEvent {
            jvmti: &jvmti,
            jni: &jni,
            thread: thread.into(),
            method: method.into(),
            location,
        };
        event_call!("jvmti_event_single_step_handler", single_step, event)
    }
}

#[no_mangle]
pub extern "C" fn jvmti_event_thread_end_handler(jvmti_env: *mut sys::JVMTIEnv,
                                                 jni_env: *mut sys::JNIEnv,
                                                 thread: jthread) {
    unsafe {
        let jvmti = jvmti!(jvmti_env);
        let jni = jni!(jni_env);
        let event = ThreadEndEvent {
            jvmti: &jvmti,
            jni: &jni,
            thread: thread.into(),
        };
        event_call!("jvmti_event_thread_end_handler", thread_end, event)
    }
}

#[no_mangle]
pub extern "C" fn jvmti_event_thread_start_handler(jvmti_env: *mut sys::JVMTIEnv,
                                                   jni_env: *mut sys::JNIEnv,
                                                   thread: jthread) {
    unsafe {
        let jvmti = jvmti!(jvmti_env);
        let jni = jni!(jni_env);
        let event = ThreadStartEvent {
            jvmti: &jvmti,
            jni: &jni,
            thread: thread.into(),
        };
        event_call!("jvmti_event_thread_start_handler", thread_start, event)
    }
}

#[no_mangle]
pub extern "C" fn jvmti_event_vm_death_handler(jvmti_env: *mut sys::JVMTIEnv,
                                               jni_env: *mut sys::JNIEnv) {
    unsafe {
        let jvmti = jvmti!(jvmti_env);
        let jni = jni!(jni_env);
        let event = VmDeathEvent {
            jvmti: &jvmti,
            jni: &jni,
        };
        event_call!("jvmti_event_vm_death_handler", vm_death, event)
    }
}

#[no_mangle]
pub extern "C" fn jvmti_event_vm_init_handler(jvmti_env: *mut sys::JVMTIEnv,
                                              jni_env: *mut sys::JNIEnv,
                                              thread: jthread) {
    unsafe {
        let jvmti = jvmti!(jvmti_env);
        let jni = jni!(jni_env);
        let event = VmInitEvent {
            jvmti: &jvmti,
            jni: &jni,
            thread: thread.into(),
        };
        event_call!("jvmti_event_vm_init_handler", vm_init, event)
    }
}

#[no_mangle]
pub extern "C" fn jvmti_event_vm_object_alloc_handler(jvmti_env: *mut sys::JVMTIEnv,
                                                      jni_env: *mut sys::JNIEnv,
                                                      thread: jthread,
                                                      object: jobject,
                                                      object_klass: jclass,
                                                      size: jlong) {
    unsafe {
        let jvmti = jvmti!(jvmti_env);
        let jni = jni!(jni_env);
        let event = VmObjectAllocEvent {
            jvmti: &jvmti,
            jni: &jni,
            thread: thread.into(),
            object: object.into(),
            object_klass: object_klass.into(),
            size,
        };
        event_call!("jvmti_event_vm_object_alloc_handler", vm_object_alloc, event)
    }
}

#[no_mangle]
pub extern "C" fn jvmti_event_vm_start_handler(jvmti_env: *mut sys::JVMTIEnv,
                                               jni_env: *mut sys::JNIEnv) {
    unsafe {
        let jvmti = jvmti!(jvmti_env);
        let jni = jni!(jni_env);
        let event = VmStartEvent {
            jvmti: &jvmti,
            jni: &jni,
        };
        event_call!("jvmti_event_vm_start_handler", vm_start, event)
    }
}
