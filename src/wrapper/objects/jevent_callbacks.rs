use crate::sys::jvmtiEventCallbacks;
use std::marker::PhantomData;
use jni_sys::jint;
use std::mem::size_of;
use crate::*;

#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct JEventCallbacks<'a> {
    internal: jvmtiEventCallbacks,
    lifetime: PhantomData<&'a ()>,
}

impl<'a> JEventCallbacks<'a> {
    pub fn new() -> Self {
        JEventCallbacks {
            internal: jvmtiEventCallbacks {
                VMInit: None,
                VMDeath: None,
                ThreadStart: None,
                ThreadEnd: None,
                ClassFileLoadHook: None,
                ClassLoad: None,
                ClassPrepare: None,
                VMStart: None,
                Exception: None,
                ExceptionCatch: None,
                SingleStep: None,
                FramePop: None,
                Breakpoint: None,
                FieldAccess: None,
                FieldModification: None,
                MethodEntry: None,
                MethodExit: None,
                NativeMethodBind: None,
                CompiledMethodLoad: None,
                CompiledMethodUnload: None,
                DynamicCodeGenerated: None,
                DataDumpRequest: None,
                reserved72: None,
                MonitorWait: None,
                MonitorWaited: None,
                MonitorContendedEnter: None,
                MonitorContendedEntered: None,
                reserved77: None,
                reserved78: None,
                reserved79: None,
                ResourceExhausted: None,
                GarbageCollectionStart: None,
                GarbageCollectionFinish: None,
                ObjectFree: None,
                VMObjectAlloc: None,
            },
            lifetime: PhantomData,
        }
    }

    pub fn size_of_callbacks() -> jint {
        size_of::<jvmtiEventCallbacks>() as jint
    }

    pub fn vm_init_enabled(&mut self) {
        self.internal.VMInit = Some(jvmti_event_vm_init_handler)
    }

    pub fn vm_death_enabled(&mut self) {
        self.internal.VMDeath = Some(jvmti_event_vm_death_handler)
    }

    pub fn thread_start_enabled(&mut self) {
        self.internal.ThreadStart = Some(jvmti_event_thread_start_handler)
    }

    pub fn thread_end_enabled(&mut self) {
        self.internal.ThreadEnd = Some(jvmti_event_thread_end_handler)
    }

    pub fn class_file_load_hook_enabled(&mut self) {
        self.internal.ClassFileLoadHook = Some(jvmti_event_class_file_load_hook_handler)
    }

    pub fn class_load_enabled(&mut self) {
        self.internal.ClassLoad = Some(jvmti_event_class_load_handler)
    }

    pub fn class_prepare_enabled(&mut self) {
        self.internal.ClassPrepare = Some(jvmti_event_class_prepare_handler)
    }

    pub fn vm_start_enabled(&mut self) {
        self.internal.VMStart = Some(jvmti_event_vm_start_handler)
    }

    pub fn exception_enabled(&mut self) {
        self.internal.Exception = Some(jvmti_event_exception_handler)
    }

    pub fn exception_catch_enabled(&mut self) {
        self.internal.ExceptionCatch = Some(jvmti_event_exception_catch_handler)
    }

    pub fn single_step_enabled(&mut self) {
        self.internal.SingleStep = Some(jvmti_event_single_step_handler)
    }

    pub fn frame_pop_enabled(&mut self) {
        self.internal.FramePop = Some(jvmti_event_frame_pop_handler)
    }

    pub fn breakpoint_enabled(&mut self) {
        self.internal.Breakpoint = Some(jvmti_event_breakpoint_handler)
    }

    pub fn field_access_enabled(&mut self) {
        self.internal.FieldAccess = Some(jvmti_event_field_access_handler)
    }

    pub fn field_modification_enabled(&mut self) {
        self.internal.FieldModification = Some(jvmti_event_field_modification_handler)
    }

    pub fn method_entry_enabled(&mut self) {
        self.internal.MethodEntry = Some(jvmti_event_method_entry_handler)
    }

    pub fn method_exit_enabled(&mut self) {
        self.internal.MethodExit = Some(jvmti_event_method_exit_handler)
    }

    pub fn native_method_bind_enabled(&mut self) {
        self.internal.NativeMethodBind = Some(jvmti_event_native_method_bind_handler)
    }

    pub fn compiled_method_load_enabled(&mut self) {
        self.internal.CompiledMethodLoad = Some(jvmti_event_compiled_method_load_handler)
    }

    pub fn compiled_method_unload_enabled(&mut self) {
        self.internal.CompiledMethodUnload = Some(jvmti_event_compiled_method_unload_handler)
    }

    pub fn dynamic_code_generated_enabled(&mut self) {
        self.internal.DynamicCodeGenerated = Some(jvmti_event_dynamic_code_generated_handler)
    }

    pub fn data_dump_request_enabled(&mut self) {
        self.internal.DataDumpRequest = Some(jvmti_event_data_dump_request_handler)
    }

    pub fn monitor_wait_enabled(&mut self) {
        self.internal.MonitorWait = Some(jvmti_event_monitor_wait_handler)
    }

    pub fn monitor_waited_enabled(&mut self) {
        self.internal.MonitorWaited = Some(jvmti_event_monitor_waited_handler)
    }

    pub fn monitor_contended_enter_enabled(&mut self) {
        self.internal.MonitorContendedEnter = Some(jvmti_event_monitor_contended_enter_handler)
    }

    pub fn monitor_contended_entered_enabled(&mut self) {
        self.internal.MonitorContendedEntered = Some(jvmti_event_monitor_contended_entered_handler)
    }

    pub fn resource_exhausted_enabled(&mut self) {
        self.internal.ResourceExhausted = Some(jvmti_event_resource_exhausted_handler)
    }

    pub fn garbage_collection_start_enabled(&mut self) {
        self.internal.GarbageCollectionStart = Some(jvmti_event_garbage_collection_start_handler)
    }

    pub fn garbage_collection_finish_enabled(&mut self) {
        self.internal.GarbageCollectionFinish = Some(jvmti_event_garbage_collection_finish_handler)
    }

    pub fn object_free_enabled(&mut self) {
        self.internal.ObjectFree = Some(jvmti_event_object_free_handler)
    }

    pub fn vm_object_alloc_enabled(&mut self) {
        self.internal.VMObjectAlloc = Some(jvmti_event_vm_object_alloc_handler)
    }
}

impl<'a> From<JEventCallbacks<'a>> for jvmtiEventCallbacks {
    fn from(callbacks: JEventCallbacks<'a>) -> Self {
        callbacks.internal
    }
}

