use crate::wrapper::{JEventCallbacks, JVMTIEnv, JThreadID, JvmtiEvent, JvmtiEventMode};

#[derive(Clone)]
struct EventItem<'a> {
    mode: JvmtiEventMode,
    event: JvmtiEvent,
    event_thread: Option<JThreadID<'a>>,
}

pub struct JEventManager<'a> {
    callbacks: JEventCallbacks<'a>,
    items: Vec<EventItem<'a>>,
}

impl<'a> JEventManager<'a> {
    pub fn new() -> JEventManager<'a> {
        JEventManager {
            callbacks: JEventCallbacks::new(),
            items: Vec::new(),
        }
    }

    pub fn apply(&mut self, env: &JVMTIEnv<'a>) {
        let _ = env.set_event_callbacks(&self.callbacks);

        self.items.iter().for_each(|e| env.set_event_notification_mode(e.mode, e.event, &e.event_thread).unwrap());
    }

    fn add_enable_event_item(&mut self, event: JvmtiEvent, event_thread: Option<JThreadID<'a>>) {
        self.items.push(EventItem { mode: JvmtiEventMode::Enable, event, event_thread })
    }

    pub fn vm_init_enabled(&mut self) {
        self.callbacks.vm_init_enabled();
        self.add_enable_event_item(JvmtiEvent::VmInit, None);
    }

    pub fn vm_death_enabled(&mut self) {
        self.callbacks.vm_death_enabled();
        self.add_enable_event_item(JvmtiEvent::VmDeath, None);
    }

    pub fn thread_start_enabled(&mut self, event_thread: Option<JThreadID<'a>>) {
        self.callbacks.thread_start_enabled();
        self.add_enable_event_item(JvmtiEvent::ThreadStart, event_thread);
    }

    pub fn thread_end_enabled(&mut self, event_thread: Option<JThreadID<'a>>) {
        self.callbacks.thread_end_enabled();
        self.add_enable_event_item(JvmtiEvent::ThreadEnd, event_thread);
    }

    pub fn class_file_load_hook_enabled(&mut self, event_thread: Option<JThreadID<'a>>) {
        self.callbacks.class_file_load_hook_enabled();
        self.add_enable_event_item(JvmtiEvent::ClassFileLoadHook, event_thread);
    }

    pub fn class_load_enabled(&mut self, event_thread: Option<JThreadID<'a>>) {
        self.callbacks.class_load_enabled();
        self.add_enable_event_item(JvmtiEvent::ClassLoad, event_thread);
    }

    pub fn class_prepare_enabled(&mut self, event_thread: Option<JThreadID<'a>>) {
        self.callbacks.class_prepare_enabled();
        self.add_enable_event_item(JvmtiEvent::ClassPrepare, event_thread);
    }

    pub fn vm_start_enabled(&mut self) {
        self.callbacks.vm_start_enabled();
        self.add_enable_event_item(JvmtiEvent::VmStart, None);
    }

    pub fn exception_enabled(&mut self, event_thread: Option<JThreadID<'a>>) {
        self.callbacks.exception_enabled();
        self.add_enable_event_item(JvmtiEvent::Exception, event_thread);
    }

    pub fn exception_catch_enabled(&mut self, event_thread: Option<JThreadID<'a>>) {
        self.callbacks.exception_catch_enabled();
        self.add_enable_event_item(JvmtiEvent::ExceptionCatch, event_thread);
    }

    pub fn single_step_enabled(&mut self, event_thread: Option<JThreadID<'a>>) {
        self.callbacks.single_step_enabled();
        self.add_enable_event_item(JvmtiEvent::SingleStep, event_thread);
    }

    pub fn frame_pop_enabled(&mut self, event_thread: Option<JThreadID<'a>>) {
        self.callbacks.frame_pop_enabled();
        self.add_enable_event_item(JvmtiEvent::FramePop, event_thread);
    }

    pub fn breakpoint_enabled(&mut self, event_thread: Option<JThreadID<'a>>) {
        self.callbacks.breakpoint_enabled();
        self.add_enable_event_item(JvmtiEvent::Breakpoint, event_thread);
    }

    pub fn field_access_enabled(&mut self, event_thread: Option<JThreadID<'a>>) {
        self.callbacks.field_access_enabled();
        self.add_enable_event_item(JvmtiEvent::FieldAccess, event_thread);
    }

    pub fn field_modification_enabled(&mut self, event_thread: Option<JThreadID<'a>>) {
        self.callbacks.field_modification_enabled();
        self.add_enable_event_item(JvmtiEvent::FieldModification, event_thread);
    }

    pub fn method_entry_enabled(&mut self, event_thread: Option<JThreadID<'a>>) {
        self.callbacks.method_entry_enabled();
        self.add_enable_event_item(JvmtiEvent::MethodEntry, event_thread);
    }

    pub fn method_exit_enabled(&mut self, event_thread: Option<JThreadID<'a>>) {
        self.callbacks.method_exit_enabled();
        self.add_enable_event_item(JvmtiEvent::MethodExit, event_thread);
    }

    pub fn native_method_bind_enabled(&mut self, event_thread: Option<JThreadID<'a>>) {
        self.callbacks.native_method_bind_enabled();
        self.add_enable_event_item(JvmtiEvent::NativeMethodBind, event_thread);
    }

    pub fn compiled_method_load_enabled(&mut self, event_thread: Option<JThreadID<'a>>) {
        self.callbacks.compiled_method_load_enabled();
        self.add_enable_event_item(JvmtiEvent::CompiledMethodLoad, event_thread);
    }

    pub fn compiled_method_unload_enabled(&mut self, event_thread: Option<JThreadID<'a>>) {
        self.callbacks.compiled_method_unload_enabled();
        self.add_enable_event_item(JvmtiEvent::CompiledMethodUnload, event_thread);
    }

    pub fn dynamic_code_generated_enabled(&mut self, event_thread: Option<JThreadID<'a>>) {
        self.callbacks.dynamic_code_generated_enabled();
        self.add_enable_event_item(JvmtiEvent::DynamicCodeGenerated, event_thread);
    }

    pub fn data_dump_request_enabled(&mut self, event_thread: Option<JThreadID<'a>>) {
        self.callbacks.data_dump_request_enabled();
        self.add_enable_event_item(JvmtiEvent::DataDumpRequest, event_thread);
    }

    pub fn monitor_wait_enabled(&mut self, event_thread: Option<JThreadID<'a>>) {
        self.callbacks.monitor_wait_enabled();
        self.add_enable_event_item(JvmtiEvent::MonitorWait, event_thread);
    }

    pub fn monitor_waited_enabled(&mut self, event_thread: Option<JThreadID<'a>>) {
        self.callbacks.monitor_waited_enabled();
        self.add_enable_event_item(JvmtiEvent::MonitorWaited, event_thread);
    }

    pub fn monitor_contended_enter_enabled(&mut self, event_thread: Option<JThreadID<'a>>) {
        self.callbacks.monitor_contended_enter_enabled();
        self.add_enable_event_item(JvmtiEvent::MonitorContendedEnter, event_thread);
    }

    pub fn monitor_contended_entered_enabled(&mut self, event_thread: Option<JThreadID<'a>>) {
        self.callbacks.monitor_contended_entered_enabled();
        self.add_enable_event_item(JvmtiEvent::MonitorContendedEntered, event_thread);
    }

    pub fn resource_exhausted_enabled(&mut self, event_thread: Option<JThreadID<'a>>) {
        self.callbacks.resource_exhausted_enabled();
        self.add_enable_event_item(JvmtiEvent::ResourceExhausted, event_thread);
    }

    pub fn garbage_collection_start_enabled(&mut self, event_thread: Option<JThreadID<'a>>) {
        self.callbacks.garbage_collection_start_enabled();
        self.add_enable_event_item(JvmtiEvent::GarbageCollectionStart, event_thread);
    }

    pub fn garbage_collection_finish_enabled(&mut self, event_thread: Option<JThreadID<'a>>) {
        self.callbacks.garbage_collection_finish_enabled();
        self.add_enable_event_item(JvmtiEvent::GarbageCollectionFinish, event_thread);
    }

    pub fn object_free_enabled(&mut self, event_thread: Option<JThreadID<'a>>) {
        self.callbacks.object_free_enabled();
        self.add_enable_event_item(JvmtiEvent::ObjectFree, event_thread);
    }

    pub fn vm_object_alloc_enabled(&mut self, event_thread: Option<JThreadID<'a>>) {
        self.callbacks.vm_object_alloc_enabled();
        self.add_enable_event_item(JvmtiEvent::VmObjectAlloc, event_thread);
    }
}