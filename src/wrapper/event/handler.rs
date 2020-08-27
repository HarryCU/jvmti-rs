use crate::event::*;

#[derive(Clone, Copy)]
pub struct EventHandlers {
    pub class_prepare: EventClassPrepareFn,
    pub breakpoint: EventBreakpointFn,
    pub class_file_load_hook: EventClassFileLoadHookFn,
    pub class_load: EventClassLoadFn,
    pub compiled_method_load: CompiledMethodLoadFn,
    pub compiled_method_unload: CompiledMethodUnloadFn,
    pub data_dump_request: DataDumpRequestFn,
    pub dynamic_code_generated: DynamicCodeGeneratedFn,
    pub exception: ExceptionFn,
    pub exception_catch: ExceptionCatchFn,
    pub field_access: FieldAccessFn,
    pub field_modification: FieldModificationFn,
    pub frame_pop: FramePopFn,
    pub garbage_collection_finish: GarbageCollectionFinishFn,
    pub garbage_collection_start: GarbageCollectionStartFn,
    pub method_entry: MethodEntryFn,
    pub method_exit: MethodExitFn,
    pub monitor_contended_enter: MonitorContendedEnterFn,
    pub monitor_contended_entered: MonitorContendedEnteredFn,
    pub monitor_wait: MonitorWaitFn,
    pub monitor_waited: MonitorWaitedFn,
    pub native_method_bind: NativeMethodBindFn,
    pub object_free: ObjectFreeFn,
    pub resource_exhausted: ResourceExhaustedFn,
    pub single_step: SingleStepFn,
    pub thread_end: ThreadEndFn,
    pub thread_start: ThreadStartFn,
    pub vm_death: VmDeathFn,
    pub vm_init: VmInitFn,
    pub vm_object_alloc: VmObjectAllocFn,
    pub vm_start: VmStartFn,
}

impl EventHandlers {
    pub fn new() -> EventHandlers {
        empty_event_handlers!()
    }

    pub fn set_breakpoint(&mut self, event: EventBreakpointFn) { self.breakpoint = event }
    pub fn set_class_file_load_hook(&mut self, event: EventClassFileLoadHookFn) { self.class_file_load_hook = event }
    pub fn set_class_load(&mut self, event: EventClassLoadFn) { self.class_load = event }
    pub fn set_class_prepare(&mut self, event: EventClassPrepareFn) { self.class_prepare = event }
    pub fn set_compiled_method_load(&mut self, event: CompiledMethodLoadFn) { self.compiled_method_load = event }
    pub fn set_compiled_method_unload(&mut self, event: CompiledMethodUnloadFn) { self.compiled_method_unload = event }
    pub fn set_data_dump_request(&mut self, event: DataDumpRequestFn) { self.data_dump_request = event }
    pub fn set_dynamic_code_generated(&mut self, event: DynamicCodeGeneratedFn) { self.dynamic_code_generated = event }
    pub fn set_exception(&mut self, event: ExceptionFn) { self.exception = event }
    pub fn set_exception_catch(&mut self, event: ExceptionCatchFn) { self.exception_catch = event }
    pub fn set_field_access(&mut self, event: FieldAccessFn) { self.field_access = event }
    pub fn set_field_modification(&mut self, event: FieldModificationFn) { self.field_modification = event }
    pub fn set_frame_pop(&mut self, event: FramePopFn) { self.frame_pop = event }
    pub fn set_garbage_collection_finish(&mut self, event: GarbageCollectionFinishFn) { self.garbage_collection_finish = event }
    pub fn set_garbage_collection_start(&mut self, event: GarbageCollectionStartFn) { self.garbage_collection_start = event }
    pub fn set_method_entry(&mut self, event: MethodEntryFn) { self.method_entry = event }
    pub fn set_method_exit(&mut self, event: MethodExitFn) { self.method_exit = event }
    pub fn set_monitor_contended_enter(&mut self, event: MonitorContendedEnterFn) { self.monitor_contended_enter = event }
    pub fn set_monitor_contended_entered(&mut self, event: MonitorContendedEnteredFn) { self.monitor_contended_entered = event }
    pub fn set_monitor_wait(&mut self, event: MonitorWaitFn) { self.monitor_wait = event }
    pub fn set_monitor_waited(&mut self, event: MonitorWaitedFn) { self.monitor_waited = event }
    pub fn set_native_method_bind(&mut self, event: NativeMethodBindFn) { self.native_method_bind = event }
    pub fn set_object_free(&mut self, event: ObjectFreeFn) { self.object_free = event }
    pub fn set_resource_exhausted(&mut self, event: ResourceExhaustedFn) { self.resource_exhausted = event }
    pub fn set_single_step(&mut self, event: SingleStepFn) { self.single_step = event }
    pub fn set_thread_end(&mut self, event: ThreadEndFn) { self.thread_end = event }
    pub fn set_thread_start(&mut self, event: ThreadStartFn) { self.thread_start = event }
    pub fn set_vm_death(&mut self, event: VmDeathFn) { self.vm_death = event }
    pub fn set_vm_init(&mut self, event: VmInitFn) { self.vm_init = event }
    pub fn set_vm_object_alloc(&mut self, event: VmObjectAllocFn) { self.vm_object_alloc = event }
    pub fn set_vm_start(&mut self, event: VmStartFn) { self.vm_start = event }
}